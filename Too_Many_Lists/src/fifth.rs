//making a queue, FIFO
//push will be in the back, because moves pointer forward not backwards

use std::mem;
use std::ptr;

pub struct List<'a, T> {
    head: Link<T>,
    //Do I not need to allocate heap memory for a raw pointer? Or free later?
    tail: *mut Node<T>, //DANGER! RAW POINTER!
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None, tail: ptr::null_mut() }
    }

    pub fn push(&mut self, elem: T) {
        let mut new_tail = Box::new(Node {
            elem: elem,
            next: None,
        });

        let raw_tail: *mut _ = &mut *new_tail;

        // .is_null checks for null, like checking for None
        if !self.tail.is_null() {
            //if old tail existed, update to point to new tail
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        } else {
            //Otherwise, update head to point to point to it
            self.head = Some(new_tail);
        }
        self.tail = raw_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            let head = *head;
            self.head = head.next;

            if self.head.is_none() {
                //!!!necessary to deal with unsafe code in another function entirely
                self.tail = ptr::null_mut(); 
            }
            head.elem
        })
    }
}

