use rust_macos_perf::get_counters;

fn main() {
    println!("Hello, world!");
    let pc = get_counters().unwrap();
    println!("{:?}", pc);
}
