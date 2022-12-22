use std::time::{Duration, Instant};
use std::thread::sleep;

fn main() {
    let now = Instant::now();

    // we sleep for 2 seconds
    sleep(Duration::new(2, 0));
    let new_now = Instant::now();
    
    println!("{}", now.elapsed().as_secs());
    
    let dur = new_now.duration_since(now);
    println!("{:?}", dur);    
    println!("{} s", dur.as_secs());
    println!("{} ms", dur.as_millis());    
    println!("{} ns", dur.as_nanos());    
}