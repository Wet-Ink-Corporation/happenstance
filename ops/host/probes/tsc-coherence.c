// Does CLOCK_MONOTONIC go BACKWARDS across CPUs on this host?
//
// The kernel refused to trust this machine's TSC at boot --
//
//     TSC synchronization [CPU#0 -> CPU#2]:
//     Measured 11225813293 cycles TSC warp between CPUs, turning off TSC clock.
//     tsc: Marking TSC unstable due to check_tsc_sync_source failed
//
// -- and fell back to `hpet`, where clock_gettime costs a measured 1,390 ns
// against 19 ns on tsc. `tsc=reliable` on the kernel command line overrides that
// refusal. This program is what says whether the override was RIGHT, rather than
// merely convenient: 11.2e9 cycles is 3.5 seconds of apparent skew between two
// cores of one die, which is not a physical quantity, but "implausible" is an
// argument and this is a measurement.
//
// ---------------------------------------------------------------------------
// Why the read, the compare and the publish sit in ONE critical section
// ---------------------------------------------------------------------------
//
// The obvious version -- read the clock, then load a shared high-water mark,
// then compare -- is WRONG, and wrong in the direction that manufactures
// evidence. Between the read and the load a thread can be scheduled out; every
// other CPU publishes meanwhile; the thread wakes, finds its own timestamp below
// the mark, and calls it a warp. The first version of this file did exactly that
// and reported 154,588,122 violations out of 962,602,000 samples with a 2 ms
// worst case across 16 contending threads. That is a scheduler latency
// distribution wearing the label of a TSC skew, and 2 ms is the tell: no cache-
// coherent counter on one die is 2 ms out.
//
// This is the shape the kernel's own check_tsc_warp uses. A spinlock, so nothing
// can come between the read and the comparison. Two CPUs alternate through the
// lock, and the lock establishes that whoever holds it read the clock LATER than
// whoever held it before -- so a value below the last one published is a genuine
// backwards step and nothing else.
//
// A pass is not proof of perfect synchrony. It is evidence that any skew is
// smaller than this test can see, which is the standard the harness actually
// needs: benchmarks/src/paired.rs measures durations, and the failure that would
// matter is a negative one.
//
//   cc -O2 -pthread -o tsc-coherence tsc-coherence.c && ./tsc-coherence 30

#define _GNU_SOURCE
#include <pthread.h>
#include <sched.h>
#include <stdatomic.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <unistd.h>

static atomic_flag lock = ATOMIC_FLAG_INIT;

/* All four guarded by `lock`. */
static uint64_t last = 0;
static unsigned long long nviol = 0;
static unsigned long long nsamp = 0;
static unsigned long long worst = 0;

static volatile int stop = 0;

static inline uint64_t now_ns(void) {
  struct timespec t;
  clock_gettime(CLOCK_MONOTONIC, &t);
  return (uint64_t)t.tv_sec * 1000000000ull + (uint64_t)t.tv_nsec;
}

static void *worker(void *arg) {
  int cpu = (int)(intptr_t)arg;
  cpu_set_t set;
  CPU_ZERO(&set);
  CPU_SET(cpu, &set);
  pthread_setaffinity_np(pthread_self(), sizeof(set), &set);

  while (!stop) {
    while (atomic_flag_test_and_set_explicit(&lock, memory_order_acquire)) {
      /* spin */
    }
    uint64_t t = now_ns(); /* read INSIDE the lock -- this is the whole point */
    if (t < last) {
      nviol++;
      if (last - t > worst) {
        worst = last - t;
      }
    } else {
      last = t;
    }
    nsamp++;
    atomic_flag_clear_explicit(&lock, memory_order_release);
  }
  return NULL;
}

int main(int argc, char **argv) {
  int seconds = argc > 1 ? atoi(argv[1]) : 30;
  int ncpu = (int)sysconf(_SC_NPROCESSORS_ONLN);
  pthread_t th[256];

  printf("tsc-coherence: %d threads, one pinned per online CPU, %d seconds\n", ncpu, seconds);
  printf("               read+compare+publish inside one spinlock\n");
  printf("               (the shape of the kernel check_tsc_warp routine)\n\n");

  for (int c = 0; c < ncpu && c < 256; c++) {
    pthread_create(&th[c], NULL, worker, (void *)(intptr_t)c);
  }
  sleep(seconds);
  stop = 1;
  for (int c = 0; c < ncpu && c < 256; c++) {
    pthread_join(th[c], NULL);
  }

  printf("locked samples     %llu\n", nsamp);
  printf("backwards reads    %llu\n", nviol);
  printf("worst backstep     %llu ns\n", worst);
  if (nviol == 0) {
    printf("\nVERDICT: no cross-CPU backwards read in %llu locked samples.\n", nsamp);
    printf("         tsc=reliable holds on this host.\n");
    return 0;
  }
  printf("\nVERDICT: the TSC IS skewed across CPUs. Remove tsc=reliable from\n");
  printf("         GRUB_CMDLINE_LINUX_DEFAULT and go back to hpet.\n");
  return 1;
}
