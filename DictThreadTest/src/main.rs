use std::{thread, array};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

fn main() {
    let dict = [["Thread 1.1", "Thread 1.2"], ["Thread 2.1", "Thread 2.2"], ["Thread 3.1", "Thread 3.2"], ["Thread 4.1", "Thread 4.2"], ["Thread 5.1", "Thread 5.2"], ["Thread 6.1", "Thread 6.2"], ["Thread 7.1", "Thread 7.2"], ["Thread 8.1", "Thread 8.2"], ["Thread 9.1", "Thread 9.2"], ["Thread 10.1", "Thread 10.2"]];
    
    let (tx, rx) = mpsc::channel();
    let mut children = Vec::new();

    let threadcount = dict.len() * 2;

    for sublist in dict {
        let tx = tx.clone();
        let thread_name = format!("{}", dict.iter().position(|x| x == &sublist).unwrap() + 1);
        // let thread_id = thread_name;
        let child = thread::spawn(move || {
            for thread_data in sublist{
                tx.send([thread_name, String::from(thread_data)]).unwrap();
            }        
        });
        children.push(child);
    }

    // Here, all the messages are collected
    let mut ids = Vec::with_capacity(threadcount as usize);
    for _ in 0..threadcount {
        // The `recv` method picks a message from the channel
        // `recv` will block the current thread if there are no messages available
        ids.push(rx.recv().unwrap());
    }
    
    // Wait for the threads to complete any remaining work
    for child in children {
        child.join().expect("oops! the child thread panicked");
    }

    
    println!("Unsorted:");
    let mut unsorted: String = format!("");
    for id in ids {
        
        unsorted = format!("{} | [{}, {}]", unsorted, id[0], id[1]);
    }
    println!("{}", unsorted);

    // get user input to keep the main thread alive
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}
