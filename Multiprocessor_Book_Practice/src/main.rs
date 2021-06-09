//call linked list

use Multiprocessor_Book_Practice::mult_thread_two::LinkedList;
//use std::iter::Iterator;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use rand::Rng;


const THREAD_NUM: u32 = 16;


fn main() {
    let stopper = Arc::new(Mutex::new(0));
    let mut first_list = Arc::new(LinkedList::new());
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
        let mut my_first_list = Arc::clone(&first_list);
        
        LinkedList::add(&mut my_first_list, 2);
        LinkedList::add(&mut my_first_list, 1);
        LinkedList::add(&mut my_first_list, 3);
        LinkedList::add(&mut my_first_list, 5);
        println!("Set after add four: ");
        LinkedList::print(&my_first_list);

        let general_handle = thread::spawn(move || {
            while *stopper.lock().unwrap() == 0 {
                let rand1 = rand::thread_rng().gen_range(10..101);
                let rand2 = rand::thread_rng().gen_range(10..101);
                let rand3 = rand::thread_rng().gen_range(10..101);
                LinkedList::add(&mut my_first_list, rand1);
                LinkedList::add(&mut my_first_list, rand2);
                LinkedList::add(&mut my_first_list, rand3);
            
                println!("Set after add three more: ");
                LinkedList::print(&first_list);
            
                println!("Popped: {}\n", LinkedList::pop(&mut my_first_list));
            
                println!("Set after pop, iter print: ");
                LinkedList::print_iter(&my_first_list);
            
                LinkedList::remove(&mut my_first_list, rand2);
            
                println!("Set after remove: ");
                LinkedList::print_iter(&my_first_list);
            }
    
        });
        handles.push(general_handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    LinkedList::print(&first_list);
}

