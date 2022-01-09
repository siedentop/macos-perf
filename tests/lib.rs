use criterion::black_box;
use macos_perf::{init, timeit_loops, PerformanceCounters};
use sudo::RunningAs;
extern crate macos_perf;

/// Test timeit_loops! macro.
#[test]
fn test_timeit_loops_macro() -> eyre::Result<()> {
    let has_sudo = sudo::check();
    if has_sudo != RunningAs::Root {
        println!("This test requires sudo");
        return Ok(());
    }

    init()?;

    let pc: PerformanceCounters = timeit_loops! {1000, {
        // Do some random work.
        let n = black_box(1000);
        let x = (0..n).fold(0, |a, b| a ^ b);
        assert_eq!(x, 0);
    }}?;

    // Check that the counters are monotonic.
    assert!(pc.cycles > 0.0);
    assert!(pc.branches > 0.0);
    assert!(pc.missed_branches > 0.0);
    assert!(pc.instructions > 0.0);
    Ok(())
}
