//RefCell enforces borrow and borrow_mut at runtime
//This program is a terrible idea for some reason

use std::rc::Rc;
use std::cell::{Ref, RefCell};

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem: elem,
            prev: None,
            next: None,
        }))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None, tail: None }
    }

    pub fn push_front(&mut self, elem: T) {
        //new node needs +2 links, everything else should be + 0
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                //non-empty list, need to connect the old_head
                old_head.borrow_mut().prev = Some(new_head.clone()); //+1 new_head
                new_head.borrow_mut().next = Some(old_head); //+1 old_head
                self.head = Some(new_head); //+1 new_h, -1 old_h
            }
            None => {
                //empty list, need to set the tail
                self.tail = Some(new_head.clone()); //+1 new_h
                self.head = Some(new_head); //+1 new_h
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        //need to take the old head, ensuring it's -2
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    //not empty list
                    new_head.borrow_mut().prev.take();//-1 old_h, removes new_h's reference
                    self.head = Some(new_head);//+1 list points to new_head
                }
                None => {
                    //emptying list
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }

    pub fn push_back(&mut self, elem: T) {
        let new_tail = Node::new(elem);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail => Some(new_tail);
                }
                None => {
                    self.head.take()
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner.elem
        })
    }

    //There no way to 'peek' a reference, since the reference goes out of scope when the peek function ends
    //You can map over a reference however -- ??? what exact does mapping mean in this context?
    //Make a new Ref for a component of the borrowed data
    //map applies a function to each element in an iterable and returns the reselting iterable, of each iteration, to the next function
    //ie Iterable.map(|current_item|, function(current_item))
    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head.as_ref().map(|node| { 
            //borrow gets reference to node, returns new reference to elem?
            Ref::map(node.borrow(), |node| &node.elem)
        })
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail.as_ref().map(|node| {
            Ref::map(node.borrow(), |node| &node.elem) //this creates new reference to elem, but still don't understand 100% how
        })
    }

    pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        self.tail.as_ref().map(|node| {
            RefMut::map(node.borrow_mut(), |node| &mut node.elem)
        })
    }

    pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        self.head.as_ref().map(|node| {
            RefMut::map(node.borrow_mut(), |node| &mut node.elem)
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.0.pop_back()
    }
}


#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop_front(), None);

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        list.push_front(4);
        list.push_front(5);

        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }
}