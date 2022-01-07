use std::cell::RefCell;

use thiserror::Error;

mod bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[derive(Debug, Clone)]
pub struct PerformanceCounters {
    pub cycles: f64,
    pub branches: f64,
    pub missed_branches: f64,
    pub instructions: f64,
}
impl From<bindings::performance_counters> for PerformanceCounters {
    fn from(counters: bindings::performance_counters) -> Self {
        PerformanceCounters {
            cycles: counters.cycles,
            branches: counters.branches,
            missed_branches: counters.missed_branches,
            instructions: counters.instructions,
        }
    }
}

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
