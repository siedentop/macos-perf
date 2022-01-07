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
}

pub fn get_counters() -> Result<PerformanceCounters, ASilPerfError> {
    if !GLOBAL_INITIALIZED.with(|initialized| *initialized.borrow()) {
        return Err(ASilPerfError::NotInitialized);
    }
    let pc = unsafe {
        // ();
        bindings::get_counters()
    };
    Ok(PerformanceCounters::from(pc))
}

pub fn init() -> () {
    unsafe {
        bindings::setup_performance_counters();
    }
    GLOBAL_INITIALIZED.with(|initialized| *initialized.borrow_mut() = true);
}

#[cfg(test)]
mod tests {
    use criterion::black_box;
    use sudo::RunningAs;

    use super::*;

    #[test]
    fn test_counters_are_monotonic() {
        init();

        let has_sudo = sudo::check();
        assert_eq!(has_sudo, RunningAs::Root, "This test requires sudo");

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
        init();
        let result = get_counters();
        let has_sudo = sudo::check();
        // TODO: We should run tests in both configurations.
        if has_sudo == RunningAs::Root {
            assert!(result.is_ok());
        } else {
            // TODO: put back in. Requires modification of C function.
            // assert!(result.is_err());
            // assert_eq!(ASilPerfError::PermissionDenied, result.unwrap_err());
        }
    }
}
