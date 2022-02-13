extern crate pbr;

use pbr::ProgressBar;
use std::thread;
use rand::Rng;

fn main() {
    const THREAD_COUNT: u64 = 18000;  // The count of frames in 5 minutes of 60FPS footage

    println!("Starting {} threads!", THREAD_COUNT);

    let mut pb = ProgressBar::new(THREAD_COUNT);  // Create progress bar with max count of THREAD_COUNT
    pb.format("╢▌▌░╟");  // Define style of progress bar

    let mut rng = rand::thread_rng();
    let sleep_time: u64 =  rng.gen_range(500..45000);  // Sleep for 0.5-45 seconds

    let (tx, rx) = std::sync::mpsc::channel();  // Create data channel

    // Spawn other threads from within it's own thread to start listening for responses sooner
    thread::spawn(move ||{
        // Spawn THREAD_COUNT threads
        for _ in 0..THREAD_COUNT {
            let thread_tx = tx.clone();  // Create a copy of the data channel sender for each thread

            // Spawn thread
            thread::spawn(move || {
                thread::sleep(std::time::Duration::from_millis(sleep_time));  // Sleep for sleep_time
                thread_tx.send(true).unwrap();  // Sent true over data channel
            });
        }
    });

    for _ in 0..THREAD_COUNT{  // Loop for every thread
        match rx.recv().unwrap() {
            _ => pb.inc(),  // For every single data sent use the catch all in match to increase the progress bar
        };
    }

    pb.finish_print("done!");
}
