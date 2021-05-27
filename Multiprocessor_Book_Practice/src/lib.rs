//implementation of a set of non-negative numbers

use std::sync::{Mutex};
use std::num::ParseIntError;
use std::io;
use std::clone::Clone;


const SET_MIN: i32 = -1;
const SET_MAX: i32 = 2147483647;

fn hash_function(value: &i32) -> i32{ //placeholder hash function
    let key = *value;//key = value in all cases
    key
}

struct Node {
    value: i32,
    key: i32,
    next: Option<Box<Node>>
}

impl Node{
    pub fn new(value: i32, next_node: Option<Box<Node>>) -> Self{
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
        let tail = Box::new(Node::new(SET_MAX, None));
        Self {
            head: Node::new(SET_MIN, Some(tail)),
        }
    }
    pub fn add(set: PosIntSet, value: i32) -> bool {
        let key = hash_function(&value);
        let locked_set = Mutex::new(set).lock().unwrap();
        let mut prev = &locked_set.head;
        let mut curr = match &prev.next {
            Some(node_pointer) => &*node_pointer, //what types is this
            None => panic!("Head does not point to a node. Where is tail?"),
        };
        while &curr.key < &key {
            let local_prev = prev;
            //let curr = Ok(*curr.clone());
            let local_prev = curr; //set interior prev to earlier curr
            let local_curr = local_prev.next; 
        }
            if key == curr.key {
                return false;
            }
            else {
                let new_node = Node::new(value, Some(curr));
                prev.next = Some(Box::new(new_node));
                return true;
            }
        
    }

}
