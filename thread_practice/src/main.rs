use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use rand::Rng;


const THREAD_NUM: u32 = 16;


fn main() {
    let stopper = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    let mutable_hash: Arc<Mutex<HashMap<i32, String>>> = Arc::new(Mutex::new(HashMap::new()));

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
        let my_mutable_hash = Arc::clone(&mutable_hash);
        let general_handle = thread::spawn(move || {
            while *stopper.lock().unwrap() == 0 {
                let rand1 = rand::thread_rng().gen_range(1..101);
                let rand2 = rand::thread_rng().gen_range(1..101);
                let rand3 = rand::thread_rng().gen_range(1..101);

                let mut locked_hash = my_mutable_hash.lock().unwrap();
                locked_hash.insert(rand1, String::from("insert"));
                locked_hash.remove(&rand2);
                if locked_hash.contains_key(&rand3) {
                    println!("{} has value {:?}", &rand3, locked_hash.get(&rand3));

                } else {
                    println!("The hash does not contain the key {}", &rand3);
                }
            }
    
        });
        handles.push(general_handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    let print_hash = mutable_hash.lock().unwrap();
    println!("The final hash table is:");
    for key in print_hash.keys(){
        println!("{},{:?}", key, print_hash.get(key));
    }
}
