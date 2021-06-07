//call linked list

use Multiprocessor_Book_Practice::LinkedList;

fn main() {
    let mut first_list = LinkedList::new();

    LinkedList::add(&mut first_list, 1);
    LinkedList::add(&mut first_list, 2);
    LinkedList::add(&mut first_list, 3);

    LinkedList::print(&first_list);
}


