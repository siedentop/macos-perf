use std::cell::RefCell;

use thiserror::Error;

pub mod performance_counters;
pub use performance_counters::{compare_perf_counters, PerformanceCounters};

mod bindings;

thread_local!(static GLOBAL_INITIALIZED: RefCell<bool> = RefCell::new(false));

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ASilPerfError {
    #[error("Library has not been initialized. Call `ASilPerf::init()` first.")]
    NotInitialized,
    #[error("kpc_get_thread_counters failed, run as sudo?")]
    PermissionDenied,
    #[error("Initialization failed {0}")]
    InitFailed(String),
}

pub fn get_counters() -> Result<PerformanceCounters, ASilPerfError> {
    if !GLOBAL_INITIALIZED.with(|initialized| *initialized.borrow()) {
        return Err(ASilPerfError::NotInitialized);
    }
    let pc = unsafe {
        let mut pc = bindings::performance_counters {
            cycles: 0.0,
            branches: 0.0,
            missed_branches: 0.0,
            instructions: 0.0,
        };
        let err_code = bindings::get_counters_checked(&mut pc);
        if err_code != 0 {
            log::error!(
                "get_counters_checked returned {}. Check `bsd/sys/errno.h`.",
                err_code
            ); // Check here: https://opensource.apple.com/source/xnu/xnu-201/bsd/sys/errno.h
            return Err(ASilPerfError::PermissionDenied);
        }
        pc
    };
    Ok(PerformanceCounters::from(pc))
}

pub fn init() -> Result<(), ASilPerfError> {
    unsafe {
        let errno = bindings::setup_performance_counters();
        if errno != 0 {
            return Err(ASilPerfError::InitFailed(format!("{}", errno)));
        }
    }
    GLOBAL_INITIALIZED.with(|initialized| *initialized.borrow_mut() = true);
    Ok(())
}

#[macro_export]
/// Runs a block a specified number of times and returns the average performance counters.
// Inspired by the timeit crate.
macro_rules! timeit_loops {
    ($loops:expr, $code:block) => {{
        use rust_macos_perf::get_counters;
        use rust_macos_perf::PerformanceCounters;

        let n = $loops;
        let start = get_counters();
        for _ in 0..n {
            $code
        }
        let end = get_counters();

        match (start, end) {
            (Ok(start), Ok(end)) => Ok((end - start) / n),
            (Err(start), _) => Err(start),
            (_, Err(end)) => Err(end),
        }
    }};
}

#[cfg(test)]
mod tests {
    use criterion::black_box;
    use sudo::RunningAs;

    use super::*;

    #[test]
    fn test_counters_are_monotonic() {
        init().unwrap();

        let has_sudo = sudo::check();
        if has_sudo != RunningAs::Root {
            println!("This test requires sudo");
            return;
        }

        let start = get_counters().unwrap();

        // Do some random work.
        let n = black_box(1000);
        let x = (0..n).fold(0, |a, b| a ^ b);
        assert_eq!(x, 0);

        let end = get_counters().unwrap();
        assert!(end.cycles > start.cycles);
        assert!(end.branches > start.branches);
        assert!(end.missed_branches > start.missed_branches);
        assert!(end.instructions > start.instructions);
    }

    /// An uninitialized library will result in Errs.
    #[test]
    fn test_load_required() {
        let result = get_counters();
        assert!(result.is_err());
        assert_eq!(ASilPerfError::NotInitialized, result.unwrap_err());

        // Now initialize the library.
        let iresult = init();
        assert!(iresult.is_ok());
        let result = get_counters();
        let has_sudo = sudo::check();

        if has_sudo == RunningAs::Root {
            assert!(result.is_ok());
        } else {
            assert!(result.is_err());
            assert_eq!(ASilPerfError::PermissionDenied, result.unwrap_err());
        }
    }
}
