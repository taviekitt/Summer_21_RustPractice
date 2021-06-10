use std::sync::{Arc, Mutex};
use std::sync::RwLock;

const SET_MIN: i32 = std::i32::MIN; //make sure to go in and make can't be deleted
const SET_MAX: i32 = std::i32::MAX; //or made

fn hash_function(value: &i32) -> i32{ //placeholder hash function
    let key = *value;//key = value in all cases
    key
}

type Valid_Link = Arc<RwLock<Node>>; //can clone arc to get multiple pointers to lock
type Link = Option<Valid_Link>;


struct Node {
    value: i32,
    key: i32,
    next: Link
}

impl Node {
    pub fn get_next(&self) -> Valid_Link{
        match &self.next { //try to get rid of this later
            Some(valid_arc) => valid_arc.clone(),
            None => panic!("what"),
        }
    }

    pub fn new(value: i32, next: Link) -> Node {
        if value < SET_MIN || value > SET_MIN {
            panic!("Value is out of range.");
        }
        Self {
            value: value,
            key: hash_function(&value),
            next: next,
        }
    }
}

struct Linked_List {
    head: Link,
    lock: Mutex<i32>, //do I need this if I'm fine-grain lock?
}

impl Linked_List {
    //new
    pub fn new() -> Self{
        let tail = Some(Arc::new(RwLock::new(Node::new(SET_MAX, None))));
        Self {
            head: Some(Arc::new(RwLock::new(Node::new(SET_MIN, tail)))),
            lock: Mutex::new(0),
        }
    }

    pub fn add(&mut self, value: i32) -> bool {
        self.lock.lock();

        //head is prev
        let mut prev: Valid_Link = match &self.head {
            Some(reference) => reference.clone(),
            None => return false,
        };

        //first element is curr
        let read_guard = match &prev.clone().read() {
            Ok(read_lock) => read_lock,
            Err(_) => return false,
        };
        let mut curr: Valid_Link = (*read_guard).get_next(); //

         //make new_node, pointing to curr
        let new_node = Node::new(value, Some(curr));

        //prev points to new_node
        let lock_result: &mut Node = match prev.borrow_mut().get_mut() {
            Ok(lock_result) => lock_result,
            Err(_) => return false,
        };

        lock_result.next = Some(Arc::new(RwLock::new(new_node)));
        true
    }
    //add
    //remove
    //Iterator?
    //print

}
