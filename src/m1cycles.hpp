#ifndef M1CYCLES_H
#define M1CYCLES_H

typedef unsigned long long uint64_t;

struct performance_counters {
  uint64_t cycles;
  uint64_t branches;
  uint64_t missed_branches;
  uint64_t instructions;
};

/// Setup the performance counters.
/// Returns err val from [1] will be returned.
/// [1]: https://opensource.apple.com/source/xnu/xnu-201/bsd/sys/errno.h
int setup_performance_counters();

/// Fill performance_counters arg and return Error code.
/// The error code is undocumented but the same as returned by
/// `kpc_get_thread_counters`.
extern int get_counters_checked(performance_counters &);

#endif
