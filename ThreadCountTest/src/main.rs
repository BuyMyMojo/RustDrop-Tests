use std::{thread, time};

fn main() {
    let loop_range = 0..100000;

    println!("Press ENTER any time after threads finish spawning to exit.");
    println!("Spawning {} threads...", loop_range.len());
    println!("These threads will sleep for 30 seconds then spring into action shouting thier thread number\n");

    for x in loop_range {
        thread::spawn(move || {
            thread::sleep(time::Duration::from_secs(15));
            println!("Thread {} reporting for duty!", x);
        });
    }

    println!("All threads spawned! Waiting for them to start...");

    // get user input to pause the program
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}
