use rust_macos_perf::{get_counters, setup_performance_counters};

fn main() {
    println!("Hello, world!");
    let pc = unsafe {
        setup_performance_counters();
        get_counters()
    };
    println!("{:?}", pc);
}
