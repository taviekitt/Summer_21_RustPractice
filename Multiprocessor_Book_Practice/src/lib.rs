//implementation of a set of non-negative numbers


use std::sync::{Arc, Mutex};
use std::sync::RwLock;

const SET_MIN: i32 = std::i32::MIN; //make sure to go in and make can't be deleted
const SET_MAX: i32 = std::i32::MAX; //or made

fn hash_function(value: &i32) -> i32{ //placeholder hash function
    let key = *value;//key = value in all cases
    key
}

type Link<Node> = RwLock<Option<Arc<Node>>>;

struct Node {
    value: i32,
    key: i32,
    next: Link<Node> //replace with raw pointer? but then loses atomic?
}

impl Node{
    pub fn new(value: i32, next_node: Link<Node>) -> Self{
        if (value < 0) | (value > 2147483646) {
            panic!("Value {} is out of the range of the set.", value);
        }
        Self {
            value,
            key: hash_function(&value),
            next: next_node
        }
    }
}

pub struct PosIntSet {
    head: Link<Node>,
    lock: Mutex<i32>,
}

impl PosIntSet {
    pub fn new() -> Self {
        let tail = RwLock::new(Some(Arc::new(Node::new(SET_MAX, RwLock::new(None)))));
        Self {
            head: RwLock::new(Some(Arc::new(Node::new(SET_MIN, tail)))),
            lock: Mutex::new(0),
        }
    }
}

pub fn link_to_node(read_locked_link: std::sync::RwLockReadGuard<std::option::Option<std::sync::Arc<Node>>>) -> Node{
    let link_option: Option<Arc<Node>> = *read_locked_link; //how to deref?
    let link_arc: Arc<Node> = match link_option {
        Some(arc) => arc,
        None => panic!("There is no arc."),
    };
    let link_node: Node = match Arc::try_unwrap(link_arc) {
        Ok(T) => T,
        Err(_) => panic!("Arc could not unwrap."),
    };
    link_node
}


impl PosIntSet {
    pub fn add(set: PosIntSet, value: i32) -> bool {
        let key = &hash_function(&value);
        set.lock.lock(); //lock list
        let mut prev: &Link<Node> = &set.head; //prev is head
        let mut prev_read_lock = prev.read().unwrap(); //deconstruct to get next out of prev
        let mut prev_node: Node = link_to_node(prev_read_lock);
        let mut curr: &Link<Node> = &prev_node.next; //curr is head.next -- need to assign lifetime?
        let mut curr_read_lock = curr.read().unwrap(); //get current_key
        let mut current_key: &i32 = &link_to_node(curr_read_lock).key;
        while &current_key < &key {//PROBLEMS -- lifetimes of references, assigning to references
            prev = &curr; //prev is curr
            prev_read_lock = prev.read().unwrap(); //update prev's lock
            prev_node = link_to_node(prev_read_lock);//finds its node again
            curr = &prev_node.next;//update curr as next from new prev
            curr_read_lock = curr.read().unwrap();//update curr lock
            current_key = &link_to_node(curr_read_lock).key;//update current_key
        }
        if &key == &current_key {
            return false;
        }
        let new_node: Node = Node::new(value, *curr);
        
        let prev_write_lock = prev.write().unwrap();
        let writer_option: Option<Arc<Node>> = *prev_write_lock; //how to deref?
        let writer_arc: Arc<Node> = match writer_option {
            Some(arc) => arc,
            None => panic!("There is no arc."),
        };
        let mut writer_node: Node = match Arc::try_unwrap(writer_arc) {
            Ok(T) => T,
            Err(_) => panic!("Arc could not unwrap."),
        };
        writer_node.next = RwLock::new(Some(Arc::new(new_node)));
        return true;
    } 
}
