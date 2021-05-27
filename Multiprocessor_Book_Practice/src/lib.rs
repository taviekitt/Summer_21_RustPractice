//implementation of a set of non-negative numbers

use std::sync::{Mutex};


const SET_MIN: i32 = -1;
const SET_MAX: i32 = 2147483647;

fn hash_function(value: &i32) -> i32{ //placeholder hash function
    let key = *value;//key = value in all cases
    key
}

struct Node {
    value: i32,
    key: i32,
    next: Result<Box<Node>, i32>
}

impl Node {
    pub fn new(value: i32, next_node: Result<Box<Node>, i32>) -> Self{
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
    head: Node,
}

impl PosIntSet {
    pub fn new() -> Self {
        let tail = Box::new(Node::new(SET_MAX, Err(0)));
        Self {
            head: Node::new(SET_MIN, Ok(tail)),
        }
    }
    pub fn add(set: Pos_Int_Set, value: i32) -> bool {
        let key = hash_function(&value);
        let locked_set = Mutex::new(set).lock().unwrap();
        let prev = &locked_set.head;
        let curr = match &prev.next {
            Ok(node_pointer) => &*node_pointer,
            Err(e) => panic!("Head does not point to a node. Where is tail?"),
        };
        while (&curr.key < &key) {
            prev = curr;
            curr = match &curr.next {
                Ok(node_pointer) => &*node_pointer,
                Err(e) => panic!("I am tail. Key is too large."),//don't panic, may be tail
            };
        }
            if &key == &curr.key {
                return false;
            }
            else {
                let new_node = Node::new(value, OK(curr));
                prev.next = Ok(new_node);
                return true;
            }
        
    }
}
