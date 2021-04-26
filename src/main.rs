use std::time::{SystemTime, SystemTimeError};

const KB: u64 = 1024;
const MB: u64 = KB * KB;

const STRIDES: &'static [u64] = &[8, 16, 32, 64, 128, 256, 512,
    KB, 2 * KB, 4 * KB, 8 * KB, 16 * KB, 32 * KB, 64 * KB, 128 * KB, 256 * KB, 512 * KB];
const MEMORY_SIZES: &'static [u64] = &[8, 16, 32, 64, 128, 256, 512,
    KB, 2 * KB, 4 * KB, 8 * KB, 16 * KB, 32 * KB, 64 * KB, 128 * KB, 256 * KB, 512 * KB,
    MB, 2 * MB, 4 * MB, 8 * MB, 16 * MB, 32 * MB, 64 * MB, 128 * MB, 256 * MB];

const WARM_UP: u64 = 10;
const ITERATIONS: u64 = 1000;

fn run_benchmark(stride: u64, memory_size: u64) -> Result<(), SystemTimeError> {
    let elements = memory_size / std::mem::size_of::<u64>() as u64;
    let jump = stride / std::mem::size_of::<u64>() as u64;
    let elements_mask = elements - 1;

    // pre-compute next access position
    let buffer: Vec<u64> = (0..elements).map(|i| ((i + jump) & elements_mask) as u64).collect();

    // warm-up run
    let mut pos: u64 = 0;
    (0..WARM_UP).for_each(|_| pos = buffer[pos as usize]);

    // benchmark run
    let now = SystemTime::now();
    (0..ITERATIONS).for_each(|_| pos = buffer[pos as usize]);
    let elapsed = now.elapsed()?.as_nanos() as f64;

    println!("{}", elapsed / (ITERATIONS as f64));
    Ok(())
}

fn benchmark_memory_latencies() {
    for stride in STRIDES {
        println!("\nStride {}", stride);
        for memory_size in MEMORY_SIZES {
            // stride should not be greater than the size of the memory buffer.
            if stride < memory_size {
                run_benchmark(*stride, *memory_size).expect("benchmark run failed");
            }
        }
    }
}

fn main() {
    benchmark_memory_latencies()
}

