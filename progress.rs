use std::thread;
use std::time::{Duration, Instant};
use std::io::{self, Write}; // Import Write trait

const DURATION: u64 = 60; // Total duration of the progress bar in seconds

fn main() {
    let interval = Duration::from_secs(DURATION) / 50; // Dividing total duration by number of steps (50)
    let steps = 50;

    let mut stdout = io::stdout(); // Obtain stdout

    for step in 0..=steps {
        print!("\r[");
        for _ in 0..step {
            print!("=");
        }
        for _ in step..steps {
            print!(" ");
        }
        print!("] {:3}% ", step * 100 / steps);
        let remaining_time = Duration::from_secs((steps - step) as u64 * interval.as_secs());
        print!("(ETA: {:02}:{:02})", remaining_time.as_secs() / 60, remaining_time.as_secs() % 60);
        stdout.flush().unwrap(); // Flush stdout
        thread::sleep(interval);
    }

    println!("\nDone!");
}
