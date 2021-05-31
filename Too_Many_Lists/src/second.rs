use std::mem;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List{ head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(), //replaces &mut option with None
        });
        self.head = Some(new_node); //still seems like assigning to borrowed value?
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }
    
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, None);
            //boxed_node goes out of scrope and gets dropped here
            //but its Node next had has been set to Link::Empty
            //so no unbounded recursion occurs (what's the risk there?)
        }
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
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {//when to apply lifetime? Is there a set of rules or do you just do it when errors pop up
        Iter { next: self.head.as_deref() }//deref acts like map iterates. In head finds link<T>, and follow through to node
    }
}

impl<'a, T> Iterator for Iter<'a, T> { //takes in Iter<T>
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {//next is a field, Option<&Node<T>>
            self.next = node.next.as_deref(); //should get the node out, and return reference
            &node.elem //return the value of the node
        })
    }
}

//IterMut -- iterator consumes the list

pub struct IterMut<'a, T> {
    next: Option<& 'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> { //gets an InterMut from list
        IterMut { next: self.head.as_deref_mut()} //makes iterMut with next field rereferenced link's node
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {//gives Iterator trait, next function
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {//function to iterate through iterator
        self.next.take().map(|node| { //still a bit confused how take works -- trades option for none value? Is that ok (yes, but only works once)
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}





#[cfg(test)]
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



