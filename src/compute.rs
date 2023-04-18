use std::thread;
use std::sync::mpsc;

use crate::coin::Coin;
use crate::tracker::Tracker;

// Initializes new thread and returns the Receiver.
fn init_thread(iterations: u32) -> mpsc::Receiver<Tracker> {

    // Define Tracker that will be transmitted to the Receiver.
    // This Tracker will be absobed by the main Tracker.
    let mut tracker = Tracker::new();

    // Define Transmitter-Receiver pair.
    let (tx, rx) = mpsc::channel();

    // Spawn thread.
    thread::spawn(move || {
        let mut i = 0u32;

        // Play games according the the delegated workload.
        while i < iterations {

            // Play game.
            let first = Coin::flip();
            let second = Coin::flip();

            // Update the Tracker.
            tracker.update(&first, &second);

            // Continue if the game is not valid.
            if first == Coin::Tails && second == Coin::Tails {
                continue;
            }
            i += 1;
        }
        // Transmit the Tracker.
        tx.send(tracker).unwrap();
    });
    // Return Receiver asynchronously.
    rx
}

pub fn multi_thread(iterations: u32, number_of_threads: u8) -> Tracker {

    // Define the main Tracker.
    // This Tracker will absorb all thread Trackers.
    let mut tracker = Tracker::new();
    
    // Define container that holds all Receivers from the theads.
    let mut thread_container: Vec<mpsc::Receiver<Tracker>> =
        Vec::with_capacity(number_of_threads.into());

    // Define general division of workload per thread.
    let workload = iterations / number_of_threads as u32;

    // Define remaining workload.
    let remaining_work = iterations % number_of_threads as u32;

    // Initialize threads.
    for thread_number in 0..number_of_threads {

        // Define workload delegated to current thread.
        let mut delegated_work = workload;
        // The first thread is delegated the remainder work.
        if thread_number == 0 {
            delegated_work += remaining_work;
        }

        // Start thread and push Receiver to thread container.
        thread_container.push(init_thread(delegated_work));
    }
    // Unwrap the Asynchronous Receiver and absorb the Tracker.
    for thread in thread_container {
        tracker.absorb(thread.recv().unwrap());
    }
    // Return the main Tracker.
    tracker
}
