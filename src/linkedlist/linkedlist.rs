use std::cell::RefCell;
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;

use super::node::Node;
#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    len: usize
}

// looking at this, this should work for all primitive data types
impl<T> Display for LinkedList<T>
where
    T: Display + Clone + Debug
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "len: {} ", self.len)?;

        write!(f, "[")?;

        let mut curr = self.head.as_ref().map(Rc::clone);
        let mut first = true;

        while let Some(node) = curr {
            if !first {
                write!(f, ", ")?;
            } else {
                first = false;
            }

            write!(f, "{}", node.borrow().data)?;
            curr = node.borrow().next.as_ref().map(Rc::clone);

            if let Some(head) = &self.head {
                if let Some(curr_node) = &curr {
                    if Rc::ptr_eq(head, curr_node) {
                        break;
                    }
                }
            }
        }

        write!(f, "]")
    }
}

impl<T: Clone + Debug + Display> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            len: 0
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn push_front(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node::new(data)));

        if self.is_empty() {
            self.tail = Some(Rc::clone(&new_node));
            self.head = Some(Rc::clone(&new_node));
        } else {
            new_node.borrow_mut().next = Some(Rc::clone(self.head.as_ref().unwrap()));
            self.head.as_ref().unwrap().borrow_mut().prev = Some(Rc::clone(&new_node));
            self.head = Some(Rc::clone(&new_node));
        }
        self.len += 1;
    }

    pub fn push_back(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node::new(data)));

        if self.is_empty() {
            self.tail = Some(Rc::clone(&new_node));
            self.head = Some(Rc::clone(&new_node));
        } else {
            self.tail.as_ref().unwrap().borrow_mut().next = Some(Rc::clone(&new_node));
            new_node.borrow_mut().prev = Some(Rc::clone(self.tail.as_ref().unwrap()));
            self.tail = Some(Rc::clone(&new_node));
        }
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            panic!("empty LinkedList");
        }

        let curr_head = Rc::clone(self.head.as_ref().unwrap());
        let data = curr_head.borrow_mut().data.clone();

        match curr_head.borrow_mut().next.as_ref() {
            Some(next) => {
                let next = Rc::clone(next);
                next.borrow_mut().prev = None;
                self.head = Some(next);
            }
            None => {
                self.head = None;
                self.tail = None;
            }
        }
        self.len -= 1;
        Some(data)
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.tail.is_none() {
            panic!("empty LinkedList");
        }

        let curr_tail = Rc::clone(self.tail.as_ref().unwrap());
        let data = curr_tail.borrow_mut().data.clone();

        match curr_tail.borrow_mut().prev.as_ref() {
            Some(prev) => {
                let prev = Rc::clone(prev);
                prev.borrow_mut().next = None;
                self.tail = Some(prev);
            }
            None => {
                self.head = None;
                self.tail = None;
            }
        }
        self.len -= 1;
        Some(data)
    }

    pub fn remove(&mut self, data: T) -> Option<T> {
        todo!()
    }

    pub fn remove_at(&mut self, index: usize) -> Option<T> {
        todo!()
    }

    pub fn peek_front(&self) -> Option<T> {
        self.head.as_ref().map(|node| {
            node.borrow().data.clone()
        })
    }

    pub fn peek_back(&self) -> Option<T> {
        self.tail.as_ref().map(|node| {
            node.borrow().data.clone()
        })
    }
}