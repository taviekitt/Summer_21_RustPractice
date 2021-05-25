use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use rand::Rng;


const THREAD_NUM: u32 = 16;


fn main() {
    let stopper = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    let mut my_Hash: HashMap<Mutex<i32>, Mutex<String>> = HashMap::new();

    {
        let stopper = Arc::clone(&stopper);
        let s_handle = thread::spawn(move || {
                thread::sleep(Duration::from_millis(10000));
                let mut num = stopper.lock().unwrap();
                *num = 1;
            });
        handles.push(s_handle);
    }   

   for _ in 0..THREAD_NUM {
        let stopper = Arc::clone(&stopper);
        let mut my_Hash = &my_Hash;
        let handle = thread::spawn(move || {
            while *stopper.lock().unwrap() == 0 {
                let rand1: Mutex<i32> = Mutex::new(rand::thread_rng().gen_range(1..101));
                let rand2: Mutex<i32> = Mutex::new(rand::thread_rng().gen_range(1..101));
                let rand3: Mutex<i32> = Mutex::new(rand::thread_rng().gen_range(1..101));
               
                my_Hash.insert(rand1, Mutex::new("insert"));
                my_Hash.remove(rand2.lock().unwrap());
                if my_Hash.contains_key(rand3.lock().unwrap()) {
                    println!("{} has value {}", &rand3, my_Hash.get(&rand3.lock().unwrap()));
                }
            }u<&|>
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
