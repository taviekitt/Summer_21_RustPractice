//implementation of single thread linked list of integers

//pub mod mult_thread_attempt;

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

struct Node {
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
        let curr = match self.current {
            Some(pointer) => {
                let next = get_next_link(Some(pointer.clone()));
                self.current = next;
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
        Self {
            head: Some(Rc::new(RefCell::new(Node::new(SET_MIN, Some(tail))))),
            lock: Mutex::new(0),
        }
    }

   // pub fn get_head(&mut self) -> ValidLink {
   //     let head: ValidLink = match &self.head {
   //         Some(reference) => reference.clone(),
    //        None => panic!("Nothing left to remove"),
   //     };
   //     head
   // }

    pub fn push(&mut self, value: i32) -> bool {
        let _locked_set = self.lock.lock(); 
        //unnecessary as this is a single-threaded program

        //head is prev
        let prev: ValidLink = match &self.head {
            Some(reference) => reference.clone(),
            None => return false,
        };

        let curr: ValidLink = prev.borrow().get_next();  //first element is curr
        let next_link: ValidLink = Rc::new(RefCell::new(Node::new(value, Some(Rc::clone(&curr)))));
        prev.borrow_mut().next = Some(next_link); //reset head
        true
    }

    pub fn pop(&mut self) -> i32 {
        let _locked_set = self.lock.lock();

        //head is prev
        let prev: ValidLink = match &self.head {
            Some(reference) => reference.clone(),
            None => panic!("Nothing left to remove"),
        };

        //first element (to be removed) is curr
        let curr: ValidLink = prev.borrow().get_next();
        //next is curr.next
        let next: ValidLink = curr.borrow().get_next();
        //head points to next
        prev.borrow_mut().next = Some(next);
        //curr points to None
        let popped_value = curr.borrow().value;
        curr.borrow_mut().next = None;
        popped_value
    }

    pub fn print(&self) {
        self.print_rec(&self.head);
    }

    fn print_rec(&self, link: &Link) {
        if link.is_some() {
            let to_print = link.as_ref().unwrap().borrow().key;
            if (to_print > SET_MIN) && (to_print < SET_MAX) { //don't print head or tail
                println!("{}", to_print);
            }
            self.print_rec(&link.as_ref().unwrap().borrow().next);
        }
    }

    //print iterative
}




































//type Link<Node> = RwLock<Option<Arc<Node>>>;

//impl Node{
    //pub fn new(value: i32, next_node: Link<Node>) -> Self{
       // if (value < 0) | (value > 2147483646) {
       //     panic!("Value {} is out of the range of the set.", value);
        //}
        //Self {
         //   value,
          //  key: hash_function(&value),
           // next: next_node
       // }
   // }
//}

//pub struct PosIntSet {
    //head: Link<Node>,
    //lock: Mutex<i32>,
//}

//impl PosIntSet {
    //pub fn new() -> Self {
      //  let tail = RwLock::new(Some(Arc::new(Node::new(SET_MAX, RwLock::new(None)))));
       // Self {
          //  head: RwLock::new(Some(Arc::new(Node::new(SET_MIN, tail)))),
           // lock: Mutex::new(0),
      //  }
   // }
//}

//pub fn link_to_node(read_locked_link: std::sync::RwLockReadGuard<std::option::Option<std::sync::Arc<Node>>>) -> Node{
  //  let link_option: Option<Arc<Node>> = *read_locked_link; //how to deref?
  //  let link_arc: Arc<Node> = match link_option {
   //     Some(arc) => arc,
   //     None => panic!("There is no arc."),
   // };
   // let link_node: Node = match Arc::try_unwrap(link_arc) {
  //      Ok(T) => T,
  //      Err(_) => panic!("Arc could not unwrap."),
  //  };
  //  link_node
//}


//impl PosIntSet {
    //pub fn add(set: PosIntSet, value: i32) -> bool {
    //    let key = &hash_function(&value);
     //   set.lock.lock(); //lock list
     //   let mut prev: &Link<Node> = &set.head; //prev is head
      //  let mut prev_read_lock = prev.read().unwrap(); //deconstruct to get next out of prev
      //  let mut prev_node: Node = link_to_node(prev_read_lock);
      //  let mut curr: &Link<Node> = &prev_node.next; //curr is head.next -- need to assign lifetime?
      //  let mut curr_read_lock = curr.read().unwrap(); //get current_key
      //  let mut current_key: &i32 = &link_to_node(curr_read_lock).key;
      //  while &current_key < &key {//PROBLEMS -- lifetimes of references, assigning to references
      //      prev = &curr; //prev is curr
      //      prev_read_lock = prev.read().unwrap(); //update prev's lock
      //      prev_node = link_to_node(prev_read_lock);//finds its node again
       //     curr = &prev_node.next;//update curr as next from new prev
       //     curr_read_lock = curr.read().unwrap();//update curr lock
       //     current_key = &link_to_node(curr_read_lock).key;//update current_key
       // }
      //  if &key == &current_key {
       //     return false;
       // }
       // let new_node: Node = Node::new(value, *curr);
        
       // let prev_write_lock = prev.write().unwrap();
      //  let writer_option: Option<Arc<Node>> = *prev_write_lock; //how to deref?
       // let writer_arc: Arc<Node> = match writer_option {
       //     Some(arc) => arc,
       //     None => panic!("There is no arc."),
       // };
       // let mut writer_node: Node = match Arc::try_unwrap(writer_arc) {
       //     Ok(T) => T,
       //     Err(_) => panic!("Arc could not unwrap."),
        //};
       // writer_node.next = RwLock::new(Some(Arc::new(new_node)));
       // return true;
   // } 
//}
