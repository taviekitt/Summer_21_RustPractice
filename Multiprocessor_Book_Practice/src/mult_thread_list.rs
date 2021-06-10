//lock links
//mutex interior mutability
//https://doc.rust-lang.org/book/ch16-03-shared-state.html#similarities-between-refcelltrct-and-mutextarct

use std::sync::{Mutex, Arc};
use std::result::Result;

const SET_MIN: i32 = std::i32::MIN;
const SET_MAX: i32 = std::i32::MAX;

type ValidLink = Arc<Mutex<Node>>;
type Link = Option<ValidLink>;

fn hash_function(value: &i32) -> i32{
    let key = *value;//key = value in all cases
    key
}

pub struct Node {
    value: i32,
    key: i32,
    next: Link
}

pub fn get_next_link(link: Link) -> Link {
    match &link.unwrap().lock().unwrap().next {
        Some(reference) => Some(reference.clone()),
        None => None,
    }
}

impl Node {
    pub fn get_next(&self) -> Link { //maybe switch to link?
        match &self.next {
            Some(reference) => Some(reference.clone()),
            None => None, //this panics more often than it should, prob when reaches tail
        }
    }
    pub fn new(value: i32, next: Link) -> Self {
        if (value < SET_MIN) | (value > SET_MAX) {
            panic!("value is out of range");
        }
        Self {
            value: value,
            key: hash_function(&value),
            next: next,
        }
    }
}

pub struct LinkedList {
    head: Link,
    current: Link,
}

impl Iterator for LinkedList {
    type Item = Link;
    fn next(& mut self) -> Option<Self::Item> {
        let curr = match &self.current {
            Some(pointer) => {
                let next = get_next_link(Some(pointer.clone()));
                self.current = next.clone();
                next
            }
            None => None,
        };
        Some(curr)
    }
}

impl LinkedList {
    pub fn new() -> Self {
        let tail = Arc::new(Mutex::new(Node::new(SET_MAX, None)));
        let head = Some(Arc::new(Mutex::new(Node::new(SET_MIN, Some(tail)))));
        Self {
            current: head.clone(),
            head: head,
        }
    }

    pub fn add(&self, value: i32) -> bool {
        //let _locked_set = self.lock.lock(); //remove?
        let key = hash_function(&value);

        let mut prev: ValidLink = match &self.head { //head is prev
            Some(reference) => reference.clone(),
            None => return false,
        };

        let mut curr: ValidLink = match prev.lock().unwrap().get_next() {
            Some(reference) => reference,
            None => return false,
        }; //first element is curr

        while curr.lock().unwrap().value < key { 
            let next = curr.lock().unwrap().get_next();
            prev = curr;
            curr = match next {
                Some(reference) => reference,
                None => return false,
            };
        }

        if curr.lock().unwrap().key == key { //curr.key == key
            return false
        }
        else { //curr.key > key
            let next_link: ValidLink = Arc::new(Mutex::new(Node::new(value, Some(Arc::clone(&curr)))));
            prev.lock().unwrap().next = Some(next_link); //reset prev
        }
        true
    }

    pub fn remove(&self, value: i32) -> bool {
        //let _locked_set = self.lock.lock();
        let key = hash_function(&value);

        //head is prev
        let mut prev: ValidLink = match &self.head {
            Some(reference) => reference.clone(),
            None => panic!("No head in list"),
        };

        let mut curr: ValidLink = match prev.lock().unwrap().get_next() {
            Some(reference) => reference,
            None => return false,
        };

        while curr.lock().unwrap().key < key {
            let next = curr.lock().unwrap().get_next();
            prev = curr;
            curr = match next {
                Some(reference) => reference,
                None => return false,
            }
        }

        if curr.lock().unwrap().key == key { //if there, remove
            let next = curr.lock().unwrap().get_next();
            prev.lock().unwrap().next = next;
            curr.lock().unwrap().next = None;
            return true;
        } 
        false
    }

    pub fn print(&self) {
        self.print_rec(&self.head);
    }

    fn print_rec(&self, link: &Link) {
        if link.is_some() {
            let to_print = link.as_ref().unwrap().lock().unwrap().value;
            if (to_print > SET_MIN) && (to_print < SET_MAX) { //don't print head or tail
                println!("{}", to_print);
            }
            self.print_rec(&link.as_ref().unwrap().lock().unwrap().next);
        }
    }

    pub fn print_iter(&self) {
        let mut link = self.head.clone();
        while link.is_some() {
            let to_print = link.as_ref().unwrap().lock().unwrap().value;
            if (to_print > SET_MIN) && (to_print < SET_MAX) { //don't print head or tail
                println!("{}", to_print);
            }
            let temp_link = link.as_ref().unwrap().lock().unwrap().next.clone();
            link = temp_link;
        }
    }
}

