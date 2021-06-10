//Calls a 16-threaded ordered linked list set for 4 milliseconds
//All but the last of the print statements are not atomic (prints may be broken up weirdly)
//This uses fine grain locking
//The lib.rs compiles and works for a single threaded program, but I can't call the iterator from main correctly

use Multiprocessor_Book_Practice::mult_thread_list::LinkedList; //NOTE: associated code is NOT in lib.rs
//use std::iter::Iterator;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use rand::Rng;


const THREAD_NUM: u32 = 16;
const MILLISECONDS_TO_RUN: u64 = 4;


fn main() {
    let stopper = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    let first_list = Arc::new(LinkedList::new());

    {
        let stopper = Arc::clone(&stopper); //Timekeeper thread -- stops program after constant number of milliseconds
        let stopper_handle = thread::spawn(move || {
                thread::sleep(Duration::from_millis(MILLISECONDS_TO_RUN));
                let mut update = stopper.lock().unwrap();
                *update = 1;
            });
        handles.push(stopper_handle);
    }   
//Worker threads
   for _ in 0..THREAD_NUM {
        let stopper = Arc::clone(&stopper);
        let my_first_list = Arc::clone(&first_list);
        LinkedList::add(&my_first_list, 2); //all threads add 4 constant numbers to list once
        LinkedList::add(&my_first_list, 1);
        LinkedList::add(&my_first_list, 3);
        LinkedList::add(&my_first_list, 5);

        println!("Set after add 2, 1, 3 and 5: ");
        LinkedList::print(&my_first_list);

        let general_handle = thread::spawn(move || {
            while *stopper.lock().unwrap() == 0 { //repeats until termination: add 3 random num, then remove the second

                let rand1 = rand::thread_rng().gen_range(10..101);
                let rand2 = rand::thread_rng().gen_range(10..101);
                let rand3 = rand::thread_rng().gen_range(10..101);
                LinkedList::add(&my_first_list, rand1);
                LinkedList::add(&my_first_list, rand2);
                LinkedList::add(&my_first_list, rand3);
    
                println!("Set after add three random: ");
                LinkedList::print(&my_first_list);
            
                LinkedList::remove(&my_first_list, rand2);

                println!("Set after remove: {}", rand2);
               LinkedList::print_iter(&my_first_list);
            }
    
        });
        handles.push(general_handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Final list: \n");
    LinkedList::print(&first_list);
}

