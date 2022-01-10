use std::borrow::{BorrowMut};
use std::cell::RefCell;
use std::fmt::Debug;
use std::ops::Range;
use std::rc::Rc;

#[cfg(test)]
mod tests {
    use std::borrow::BorrowMut;
    use crate::List;

    #[test]
    fn it_works() {
        let mut list = List::new(1);
        list.append(2);
        list.append(3);
        list.append(4);
        list.append(5);

        println!("Test 1:");
        list.iter(|l| {
            unsafe {
                println!("{}", l.0.borrow_mut().as_ptr().read());
            }
        });

        println!("Test 2:");
        list.iter_in(|l| {
            unsafe {
                println!("{}", l.0.borrow_mut().as_ptr().read());
            }
        }, 0..50);
    }
}

#[derive(Debug)]
pub struct Node<T: Copy> (TLink<T>, Option<NodeLink<T>>);

#[derive(Debug)]
pub struct List<T: Copy> (NodeLink<T>, usize);

impl<T: Copy> Node<T> {
    pub fn new(i: T) -> Self {
        Node(Rc::new(RefCell::new(i)), None)
    }

    pub fn set_pointer(&mut self, to: NodeLink<T>) -> &Self {
        self.1 = Some(to);
        self
    }
}

type NodeLink<T> = Rc<RefCell<Node<T>>>;
type TLink<T> = Rc<RefCell<T>>;

impl<T: Copy + Debug > List<T> {
    pub fn new(with_init: T) -> Self{
        let node = Node::new(with_init);
        let red: NodeLink<T> = Rc::new(RefCell::new(node));

        let mut list = List(Rc::clone(&red), 1);
        unsafe { (*list.0.borrow_mut().as_ptr()).1 = Some(Rc::clone(&red)) }
        list
    }

    pub fn append(&mut self,  element: T) {
        let mut node = Node::new(element);
        node.1 = Some(Rc::clone(&self.0));
        let red: NodeLink<T> = Rc::new(RefCell::new(node));
        unsafe {
            let mut current: &Option<NodeLink<T>> = &(*self.0.borrow_mut().as_ptr()).1;

            for _ in 1..self.1-1 {
                current = &(*current.as_ref().unwrap().as_ptr()).1;
            }

            //println!("{:?}", (*current.as_ref().unwrap().as_ptr()).0);
            (*current.as_ref().unwrap().as_ptr()).1 = Some(Rc::clone(&red));
        }

        self.1 += 1;


    }

    pub fn iter(&mut self, action: impl Fn(&mut Node<T>)) {

        unsafe {
            action(&mut (*self.0.borrow_mut().as_ptr()));
            let mut current: &Option<NodeLink<T>> = &(*self.0.borrow_mut().as_ptr()).1;
            for _ in 1..self.1  {
                action(&mut (*current.as_ref().unwrap().as_ptr()));
                current = &(*current.as_ref().unwrap().as_ptr()).1;
            }
        }
    }

    pub fn iter_in(&mut self, action: impl Fn(&mut Node<T>), range: Range<usize>) {

        unsafe {
            action(&mut (*self.0.borrow_mut().as_ptr()));
            let mut current: &Option<NodeLink<T>> = &(*self.0.borrow_mut().as_ptr()).1;
            for _ in range.start..range.end - 1  {
                action(&mut (*current.as_ref().unwrap().as_ptr()));
                current = &(*current.as_ref().unwrap().as_ptr()).1;
            }
        }
    }

}

