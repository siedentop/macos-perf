#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);

        let _pc = performance_counters {
            cycles: 0.0,
            branches: 0.0,
            missed_branches: 0.0,
            instructions: 0.0,
        };
        unsafe {
            setup_performance_counters();
            let pc = get_counters();
        }
    }
}
