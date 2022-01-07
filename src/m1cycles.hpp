#ifndef M1CYCLES_H
#define M1CYCLES_H

#include <cstdint>

struct performance_counters {
  double cycles;
  double branches;
  double missed_branches;
  double instructions;
  performance_counters(uint64_t c, uint64_t b, uint64_t m, uint64_t i)
      : cycles(c), branches(b), missed_branches(m), instructions(i) {}
  performance_counters(double c, double b, double m, double i)
      : cycles(c), branches(b), missed_branches(m), instructions(i) {}
  performance_counters(double init)
      : cycles(init), branches(init), missed_branches(init),
        instructions(init) {}

  inline performance_counters &operator-=(const performance_counters &other) {
    cycles -= other.cycles;
    branches -= other.branches;
    missed_branches -= other.missed_branches;
    instructions -= other.instructions;
    return *this;
  }
  inline performance_counters &min(const performance_counters &other) {
    cycles = other.cycles < cycles ? other.cycles : cycles;
    branches = other.branches < branches ? other.branches : branches;
    missed_branches = other.missed_branches < missed_branches
                          ? other.missed_branches
                          : missed_branches;
    instructions =
        other.instructions < instructions ? other.instructions : instructions;
    return *this;
  }
  inline performance_counters &operator+=(const performance_counters &other) {
    cycles += other.cycles;
    branches += other.branches;
    missed_branches += other.missed_branches;
    instructions += other.instructions;
    return *this;
  }

  inline performance_counters &operator/=(double numerator) {
    cycles /= numerator;
    branches /= numerator;
    missed_branches /= numerator;
    instructions /= numerator;
    return *this;
  }
};

inline performance_counters operator-(const performance_counters &a,
                                      const performance_counters &b) {
  return performance_counters(a.cycles - b.cycles, a.branches - b.branches,
                              a.missed_branches - b.missed_branches,
                              a.instructions - b.instructions);
}

/// Setup the performance counters.
/// Returns err val from [1] will be returned.
/// [1]: https://opensource.apple.com/source/xnu/xnu-201/bsd/sys/errno.h
int setup_performance_counters();

/// Fill performance_counters arg and return Error code.
/// The error code is undocumented but the same as returned by
/// `kpc_get_thread_counters`.
extern int get_counters_checked(performance_counters &);

#endif
