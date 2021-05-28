use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List{ head: Link::Empty }
    }
}

impl List {
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(& mut self.head, Link::Empty), //What does this actually do?? Why accept changing borrowed value?
        });
        self.head = Link::More(new_node); //still seems like assigning to borrowed value?
    }
}

impl List {
    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None, 
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}



