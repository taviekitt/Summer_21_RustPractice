//Below holds single-threaded linked list library, with an iterator I can't figure out how to call from main

use std::sync::Mutex;
use std::cell::RefCell;
use std::rc::Rc;

const SET_MIN: i32 = std::i32::MIN;
const SET_MAX: i32 = std::i32::MAX;

type ValidLink = Rc<RefCell<Node>>;
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
    match &link.unwrap().clone().borrow().next {
        Some(reference) => Some(reference.clone()),
        None => None,
    }
}

impl Node {
    pub fn get_next(&self) -> ValidLink {
        match &self.next {
            Some(reference) => reference.clone(),
            None => panic!("There is no next."),
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
    lock: Mutex<i32>
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
        let tail = Rc::new(RefCell::new(Node::new(SET_MAX, None)));
        let head = Some(Rc::new(RefCell::new(Node::new(SET_MIN, Some(tail)))));
        Self {
            current: head.clone(),
            head: head,
            lock: Mutex::new(0),
        }
    }

    pub fn add(&mut self, value: i32) -> bool {
        let _locked_set = self.lock.lock();
        let key = hash_function(&value);

        let mut prev: ValidLink = match &self.head { //head is prev
            Some(reference) => reference.clone(),
            None => return false,
        };

        let mut curr: ValidLink = prev.borrow().get_next(); //first element is curr

        while curr.borrow().value < key { 
            let next = curr.borrow().get_next();
            prev = curr;
            curr = next;
        }

        if curr.borrow().key == key { //curr.key == key
            return false
        }
        else { //curr.key > key
            let next_link: ValidLink = Rc::new(RefCell::new(Node::new(value, Some(Rc::clone(&curr)))));
            prev.borrow_mut().next = Some(next_link); //reset prev
        }
        true
    }

    pub fn pop(&mut self) -> i32 {
        let _locked_set = self.lock.lock();

        //head is prev
        let prev: ValidLink = match &self.head {
            Some(reference) => reference.clone(),
            None => panic!("Nothing left to remove"),
        };

        let curr: ValidLink = prev.borrow().get_next();
        let next: ValidLink = curr.borrow().get_next();
        prev.borrow_mut().next = Some(next);
        let popped_value = curr.borrow().value;
        curr.borrow_mut().next = None;
        popped_value
    }

    pub fn remove(&mut self, value: i32) -> bool {
        let _locked_set = self.lock.lock();
        let key = hash_function(&value);

        //heas is prev
        let mut prev: ValidLink = match &self.head {
            Some(reference) => reference.clone(),
            None => panic!("No head in list"),
        };

        let mut curr: ValidLink = prev.borrow().get_next();

        while curr.borrow().key < key {
            let next = curr.borrow().get_next();
            prev = curr;
            curr = next;
        }

        if curr.borrow().key == key { //if there, remove
            let next = curr.borrow().get_next();
            prev.borrow_mut().next = Some(next);
            curr.borrow_mut().next = None;
            return true;
        } 
        false
    }

    pub fn print(&self) {
        self.print_rec(&self.head);
    }

    fn print_rec(&self, link: &Link) {
        if link.is_some() {
            let to_print = link.as_ref().unwrap().borrow().value;
            if (to_print > SET_MIN) && (to_print < SET_MAX) { //don't print head or tail
                println!("{}", to_print);
            }
            self.print_rec(&link.as_ref().unwrap().borrow().next);
        }
    }

    pub fn print_iter(&self) {
        let mut link = self.head.clone();
        while link.is_some() {
            let to_print = link.as_ref().unwrap().borrow().value;
            if (to_print > SET_MIN) && (to_print < SET_MAX) { //don't print head or tail
                println!("{}", to_print);
            }
            let temp_link = link.as_ref().unwrap().borrow().next.clone();
            link = temp_link;
        }
    }
}

