//implementation of a set of non-negative numbers


use std::sync::{Arc, Mutex};

const SET_MIN: i32 = std::i32::MIN; //make sure to go in and make can't be deleted
const SET_MAX: i32 = std::i32::MAX; //or made

fn hash_function(value: &i32) -> i32{ //placeholder hash function
    let key = *value;//key = value in all cases
    key
}

struct Node {
    value: i32,
    key: i32,
    next: Option<Arc<Node>> //replace with raw pointer? but then loses atomic?
}

impl Node{
    pub fn new(value: i32, next_node: Option<Arc<Node>>) -> Self{
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
    head: Option<Arc<Node>>,
    lock: Mutex<i32>,
}

impl PosIntSet {
    pub fn new() -> Self {
        let tail = Ok(Arc::new(Node::new(SET_MAX, None)));
        Self {
            head: Ok(Arc::new(Node::new(SET_MIN, Some(tail))),
            lock: Mutex::new(0),
        }
    }
}

impl PosIntSet {
    pub fn add(set: PosIntSet, value: i32) -> bool {
        let key = hash_function(&value);
        set.lock.lock();
        let mut prev: &Option<Arc<Node>> = &set.head;
        let mut curr: &Option<Arc<Node>> = match prev {
            Some(node_pointer) => &node_pointer.next, 
            None => panic!("Head does not point to a node. Where is tail?"),
        };
        let mut current_key = match &curr {
            Some(inside_curr) => inside_curr.key,
            None => panic!("There should be a key here!"),
        };
        while &current_key < &key {
            prev = curr; //set interior prev to earlier curr
            let curr: &Option<Arc<Node>> = match curr {
                Some(curr_pointer) => &curr_pointer.next, 
                None => panic!("I am tail, but should not be."),
            };
            current_key = match &curr {
                Some(inside_curr) => inside_curr.key,
                None => panic!("There really should be a key here!"),
            }; 
        }
        if key == current_key {
            return false;
        }
        let new_node: Node = match curr {
            Some(current) => Node::new(value, Some(current.clone())),
            None => println!("well this is an odd late problem.")
        };
        unsafe { //loses the multi-threaded ability of arcs -- this seems like a terrible implementation of anything
            //Do I need arcs to guarantee no-race cond if whole list locks? Or just for lifetimes stuff
            let raw_prev_pointer: *mut Node = prev.as_ref().map(|prev| &prev);
            let raw_new_pointer: *mut Node = &new_node;
            *(raw_prev_pointer).next = raw_new_pointer; //difficult line, maybe use unsafe
            //do I have to make every next field a raw pointer? lots of unsafe code
        }
        return true;
    } 
}

//main lesson? Prob should implement a linked-list which can be modified in the middle when 
//working over multiple threads

//Idea -- write an iterator (or multiple) so that searching is easier and seperates add/remove func


