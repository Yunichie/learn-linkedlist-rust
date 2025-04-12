use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Node<T> {
    pub(crate) next: Option<Rc<RefCell<Node<T>>>>,
    pub(crate) prev: Option<Rc<RefCell<Node<T>>>>,
    pub(crate) data: T
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Node {
            next: None,
            prev: None,
            data
        }
    }
}