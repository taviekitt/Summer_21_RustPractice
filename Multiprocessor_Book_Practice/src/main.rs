//implementation of a set of non-negative numbers
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

const THREAD_NUM: u32 = 16;

fn main() {
    use lib::PosIntSet;
    let stopper = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    {
        let stopper = Arc::clone(&stopper);
        let stopper_handle = thread::spawn(move || {
                thread::sleep(Duration::from_millis(10000));
                let mut update = stopper.lock().unwrap();
                *update = 1;
            });
        handles.push(stopper_handle);
    }   

   for _ in 0..THREAD_NUM {
        let stopper = Arc::clone(&stopper);
        let shared_set = PosIntSet::new();
        let general_handle = thread::spawn(move || {
            while *stopper.lock().unwrap() == 0 {
                //add, remove, search method testers
                PosIntSet::add(shared_set, 35);
                PosIntSet::add(shared_set, 13);
                PosIntSet::add(shared_set, 468);
            }
        });
        handles.push(general_handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}


