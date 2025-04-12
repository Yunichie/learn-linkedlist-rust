use std::fmt::{Debug, Display};
mod linkedlist;
use linkedlist::linkedlist::LinkedList;

fn main() {
    let mut ll = LinkedList::new();
    ll.push_front('a');
    ll.push_front('b');
    ll.push_front('c');
    println!("{ll}");

    ll.push_back('d');
    ll.push_back('e');
    println!("{ll}");

    ll.pop_front();
    ll.pop_front();
    ll.pop_front();
    println!("{ll}");

    ll.pop_back();
    ll.pop_back();
    println!("{ll}");
}