//call linked list

use Multiprocessor_Book_Practice::mult_thread_two::LinkedList;
//use std::iter::Iterator;

fn main() {
    let mut first_list = LinkedList::new();

    LinkedList::add(&mut first_list, 2);
    LinkedList::add(&mut first_list, 1);
    LinkedList::add(&mut first_list, 3);
    LinkedList::add(&mut first_list, 5);

    println!("Set after add four: ");
    LinkedList::print(&first_list);

    LinkedList::add(&mut first_list, 7);
    LinkedList::add(&mut first_list, 10);
    LinkedList::add(&mut first_list, 3);

    println!("Set after add two more: ");
    LinkedList::print(&first_list);

    println!("Popped: {}\n", LinkedList::pop(&mut first_list));

    println!("Set after pop, iter print: ");
    LinkedList::print_iter(&first_list);

    LinkedList::remove(&mut first_list, 5);

    println!("Set after remove: ");
    LinkedList::print_iter(&first_list);

    //how to call on iterator?
    //let iter = Iterator::<LinkedList>::iter(&first_list);

    //println!("The last element is {}", iter.count());

}


