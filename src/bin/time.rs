use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    let start_time = Instant::now();
    let d = Duration::from_secs(1);
    sleep(d);

    let elapsed_duration = start_time.elapsed();
    println!("{} elapsed.", elapsed_duration.as_micros());
}
