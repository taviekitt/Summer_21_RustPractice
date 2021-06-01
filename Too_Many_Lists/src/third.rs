use std::sync::Arc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Arc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    //adds element to the front of the list
    pub fn append(&self, elem:T) -> List<T> {
        List { head: Some(Arc::new(Node {
            elem: elem,
            next: self.head.clone(),
        }))}
    }

    //returns whole list minus first element
    pub fn tail(&self) -> List<T> {
        List { head: self.head.as_ref().and_then(|node| node.next.clone())} 
    }//and_then returns Option<Y> rather than Option<Option<Y>>
}

//Only Iter works, not mutable Iters
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref() }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Arc::try_unwrap(node) { //takes out node only if last pointer
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}