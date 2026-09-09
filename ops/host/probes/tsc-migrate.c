// Does CLOCK_MONOTONIC go backwards when ONE thread moves between CPUs?
//
// This is the third instrument written for this question, and the first with no
// concurrency in it at all. The first two were multi-threaded and both failed
// their own controls -- the second reported 299,999 backwards reads out of
// 300,000 on CPU0 against CPU1, which are SMT siblings of one physical core
// reading one counter. A result like that is a statement about the program.
//
// So: one thread. It reads the clock, migrates to the next CPU, reads again, and
// compares. Both reads are by the same thread in program order, so a value that
// comes back smaller can only mean the two CPUs disagree about the time. There
// is no lock, no baton, no shared mutable state and no memory ordering to get
// wrong.
//
// Why the question matters: the kernel refused to trust this machine's TSC at
// boot ("Measured 11225813293 cycles TSC warp between CPUs") and fell back to
// hpet, where clock_gettime costs a measured 1,390 ns against 19 ns on tsc.
// `tsc=reliable` overrides the refusal, and this says whether it should.
//
// The control is the same one the pairwise version had, and it is kept because
// it is what caught the last two attempts: the CPU0->CPU1 hop is between SMT
// siblings. If THAT hop ever reports a backwards read, the instrument is broken
// again and no other row means anything.
//
//   cc -O2 -o tsc-migrate tsc-migrate.c && ./tsc-migrate 2000000

#define _GNU_SOURCE
#include <sched.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <unistd.h>
#include <string.h>

static inline uint64_t now_ns(void) {
  struct timespec t;
  clock_gettime(CLOCK_MONOTONIC, &t);
  return (uint64_t)t.tv_sec * 1000000000ull + (uint64_t)t.tv_nsec;
}

static int pin(int cpu) {
  cpu_set_t set;
  CPU_ZERO(&set);
  CPU_SET(cpu, &set);
  return sched_setaffinity(0, sizeof(set), &set);
}

int main(int argc, char **argv) {
  long hops = argc > 1 ? atol(argv[1]) : 2000000;
  int ncpu = (int)sysconf(_SC_NPROCESSORS_ONLN);

  /* Optional second argument: a comma-separated CPU list to cycle around,
     instead of every online CPU. This is how the question "does confining a
     measurement to one physical core avoid the skew?" gets an answer rather
     than an assumption. */
  int cpus[256];
  int ncycle = 0;
  if (argc > 2) {
    char *copy = strdup(argv[2]), *tok = strtok(copy, ",");
    while (tok && ncycle < 256) { cpus[ncycle++] = atoi(tok); tok = strtok(NULL, ","); }
    free(copy);
  } else {
    for (int i = 0; i < ncpu && i < 256; i++) cpus[ncycle++] = i;
  }

  long viol = 0, sibling_viol = 0;
  uint64_t worst = 0;
  long long sum_hop = 0;

  printf("tsc-migrate: one thread, %ld migrations around %d CPUs\n", hops, ncpu);
  printf("             clocksource in use: ");
  fflush(stdout);
  {
    FILE *f = fopen("/sys/devices/system/clocksource/clocksource0/current_clocksource", "r");
    char buf[64] = "unknown";
    if (f) {
      if (fgets(buf, sizeof buf, f) == NULL) {
        snprintf(buf, sizeof buf, "unreadable");
      }
      fclose(f);
    }
    printf("%s", buf);
  }
  printf("\n\n");

  pin(0);
  uint64_t prev = now_ns();
  int prev_cpu = 0;

  for (long i = 0; i < hops; i++) {
    int cpu = cpus[(i + 1) % ncycle];
    if (pin(cpu) != 0) {
      continue;
    }
    uint64_t t = now_ns();
    if (t < prev) {
      viol++;
      /* The control: CPU n -> CPU n+1 where they are SMT siblings, i.e. an even
         source CPU. Same physical core, same counter. */
      if ((prev_cpu % 2) == 0 && cpu == prev_cpu + 1) {
        sibling_viol++;
      }
      if (prev - t > worst) {
        worst = prev - t;
      }
    } else {
      sum_hop += (long long)(t - prev);
    }
    prev = t;
    prev_cpu = cpu;
  }

  printf("migrations              %ld\n", hops);
  printf("backwards reads         %ld\n", viol);
  printf("  of those, SMT-sibling %ld   <- the control; must be 0\n", sibling_viol);
  printf("worst backstep          %llu ns\n", (unsigned long long)worst);
  printf("mean forward hop        %lld ns\n", hops ? sum_hop / hops : 0);
  printf("\n");

  if (sibling_viol != 0) {
    printf("VERDICT: the SMT-sibling control fired. The instrument is wrong again;\n");
    printf("         no row above says anything about the TSC.\n");
    return 2;
  }
  if (viol == 0) {
    printf("VERDICT: no backwards read in %ld migrations across all %d CPUs, and\n", hops, ncpu);
    printf("         the sibling control is clean. tsc=reliable holds on this host.\n");
    return 0;
  }
  printf("VERDICT: %ld genuine cross-CPU backwards reads, worst %llu ns. The TSCs\n",
         viol, (unsigned long long)worst);
  printf("         disagree. Remove tsc=reliable and go back to hpet.\n");
  return 1;
}
