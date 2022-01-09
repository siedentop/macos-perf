/// You need to run this with sudo.
use criterion::black_box;
use rust_macos_perf::{init, timeit_loops};

fn main() {
    init().unwrap();
    let pc = timeit_loops! {10, {
        let n = black_box(1000);
        let _x = (0..n).fold(0, |a, b| a ^ b);
    }}
    .unwrap();
    println!("{:?}", pc);
}
