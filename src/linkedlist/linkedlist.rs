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

impl<T> LinkedList<T> {
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

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T: Clone + Debug + Display> LinkedList<T> {
    pub fn push_front(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node::new(data)));

        if let Some(prev_head) = self.head.take() {
            prev_head.borrow_mut().prev = Some(new_node.clone());
            new_node.borrow_mut().next = Some(prev_head);
            self.head = Some(new_node);
        } else {
            self.tail = Some(new_node.clone());
            self.head = Some(new_node);
        }
        self.len += 1;
    }

    pub fn push_back(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node::new(data)));

        if let Some(old_tail) = self.tail.take() {
            old_tail.borrow_mut().next = Some(new_node.clone());
            new_node.borrow_mut().prev = Some(old_tail);
            self.tail = Some(new_node);
        } else {
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);
        }
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            panic!("empty LinkedList");
        }

        let prev_head = self.head.take()?;
        let mut prev_head_borrow = prev_head.borrow_mut();

        if let Some(next_node) = prev_head_borrow.next.take() {
            next_node.borrow_mut().prev = None;
            self.head = Some(next_node);
        } else {
            self.tail = None;
        }
        self.len -= 1;
        Some(prev_head_borrow.data.clone())
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.tail.is_none() {
            panic!("empty LinkedList");
        }

        let prev_tail = self.tail.take()?;
        let mut prev_tail_borrow = prev_tail.borrow_mut();

        if let Some(prev_node) = prev_tail_borrow.prev.take() {
            prev_node.borrow_mut().next = None;
            self.tail = Some(prev_node);
        } else {
            self.head = None;
        }
        self.len -= 1;
        Some(prev_tail_borrow.data.clone())
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