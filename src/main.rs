use rust_macos_perf::{get_counters, init};

fn main() {
    println!("Hello, world!");
    init().unwrap();
    let pc = get_counters().unwrap();
    println!("{:?}", pc);
}
