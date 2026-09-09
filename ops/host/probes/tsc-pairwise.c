// Pairwise CLOCK_MONOTONIC skew: cpu0 against every other CPU, one pair at a
// time, ping-pong handshake.
//
// This exists because the 16-thread version could not tell a real skew from its
// own contention, twice. Two threads alternating strictly -- A reads, hands the
// baton to B, B reads and compares, hands it back -- has no queue to wait in and
// no third party to be preempted behind. If a pair still reports a backwards
// read, the two clocks really do disagree.
//
// The discriminator that makes the whole run interpretable is the FIRST pair.
// CPU0 and CPU1 are SMT siblings: one physical core, one TSC, physically the
// same counter. A backwards read there cannot be skew, so a non-zero count on
// pair (0,1) condemns this instrument rather than the machine -- and every other
// row should then be read as noise too.
//
//   cc -O2 -pthread -o tsc-pairwise tsc-pairwise.c && ./tsc-pairwise

#define _GNU_SOURCE
#include <pthread.h>
#include <sched.h>
#include <stdatomic.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <unistd.h>

#define ROUNDS 300000

static atomic_int baton;   /* whose turn: 0 or 1 */
static uint64_t last;
static long long viol;
static uint64_t worst;
static int done;

static inline uint64_t now_ns(void) {
  struct timespec t;
  clock_gettime(CLOCK_MONOTONIC, &t);
  return (uint64_t)t.tv_sec * 1000000000ull + (uint64_t)t.tv_nsec;
}

struct arg { int cpu; int side; };

static void *worker(void *p) {
  struct arg *a = p;
  cpu_set_t set;
  CPU_ZERO(&set);
  CPU_SET(a->cpu, &set);
  pthread_setaffinity_np(pthread_self(), sizeof(set), &set);

  for (int i = 0; i < ROUNDS; i++) {
    while (atomic_load_explicit(&baton, memory_order_acquire) != a->side) {
      if (atomic_load_explicit(&baton, memory_order_relaxed) == -1) return NULL;
    }
    uint64_t t = now_ns();
    if (last != 0 && t < last) {
      viol++;
      if (last - t > worst) worst = last - t;
    }
    last = t;
    atomic_store_explicit(&baton, 1 - a->side, memory_order_release);
  }
  done++;
  return NULL;
}

int main(void) {
  int ncpu = (int)sysconf(_SC_NPROCESSORS_ONLN);
  printf("tsc-pairwise: cpu0 against each other CPU, %d strict alternations per pair\n", ROUNDS);
  printf("              pair (0,1) are SMT siblings -- ONE physical core, ONE TSC.\n");
  printf("              A backwards read there condemns the instrument, not the machine.\n\n");
  printf("%-10s %-14s %-16s %s\n", "pair", "backwards", "worst backstep", "reading");

  int bad_sibling = 0;
  for (int other = 1; other < ncpu; other++) {
    pthread_t ta, tb;
    struct arg a = {0, 0}, b = {other, 1};
    baton = 0; last = 0; viol = 0; worst = 0; done = 0;

    pthread_create(&ta, NULL, worker, &a);
    pthread_create(&tb, NULL, worker, &b);
    pthread_join(ta, NULL);
    atomic_store(&baton, -1);
    pthread_join(tb, NULL);

    const char *reading = (viol == 0) ? "coherent" : "DISAGREES";
    char pair[16];
    snprintf(pair, sizeof pair, "(0,%d)", other);
    printf("%-10s %-14lld %-16llu %s\n", pair, viol, (unsigned long long)worst, reading);
    if (other == 1 && viol != 0) bad_sibling = 1;
  }

  printf("\n");
  if (bad_sibling) {
    printf("VERDICT: pair (0,1) disagreed. Those are two threads of ONE core reading\n");
    printf("         ONE counter, so this instrument is measuring itself. Every row\n");
    printf("         above is noise; draw no conclusion about the TSC from it.\n");
    return 2;
  }
  printf("VERDICT: the SMT-sibling control is clean, so the rows above mean what\n");
  printf("         they say.\n");
  return 0;
}
