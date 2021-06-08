//call linked list

use Multiprocessor_Book_Practice::LinkedList;

fn main() {
    let mut first_list = LinkedList::new();

    LinkedList::push(&mut first_list, 1);
    LinkedList::push(&mut first_list, 2);
    LinkedList::push(&mut first_list, 3);

    LinkedList::print(&first_list);

    println!("Popped: {}\n", LinkedList::pop(&mut first_list));

    println!("Set after: ");
    LinkedList::print(&first_list);

    println!("> {:?}", first_list.get_head().next());
}


