# About

Measure the performance of Rust code on Apple's M1 series chips using performance counters.
The resulting measurements will be much more stable than simply timing the execution. However,
we still need to model the expected execution time based off the reported metrics.

# Example

```rust
// See also examples/main.rs
use rust_macos_perf::{init, timeit_loops};

init().unwrap();
let pc = timeit_loops! {10, {
    // Your function here.
}}
.unwrap();
println!("{:?}", pc);
```

# Warning

**This is an extremely unstable API. Also the the underlying Apple functions may change anytime.**

## Requirements

1. This only works on Apple M1 series chips.
2. For this to work, the Apple Developer SDK needs to be installed. This can be installed via `xcode-select --install`. If anything else is missing, please file a ticket here on GitHub.
3. The resulting program needs to be run with `sudo`.

## Development

Please run tests both as sudo and without sudo.

# Inspirations | Related Work

- Lemire's blog post and accompanying code: http://lemire.me/blog/2021/03/24/counting-cycles-and-instructions-on-the-apple-m1-processor/
- Original code by [dougallj](https://twitter.com/dougallj):
  https://gist.github.com/dougallj/5bafb113492047c865c0c8cfbc930155#file-m1_robsize-c-L390
- https://bheisler.github.io/post/criterion-rs-0-3-4/ Introducing me to the world of counter-based profiling.
