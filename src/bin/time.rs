use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now();
    let d = Duration::from_secs(1);
    sleep(d);

    let end = start.elapsed();
    println!("{} elapsed.", end.as_micros());
}
