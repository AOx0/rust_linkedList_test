use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::fmt::Debug;
use std::ops::Range;
use std::rc::Rc;

#[cfg(test)]
mod tests {
    use crate::List;

    #[test]
    fn it_works() {
        let mut list = List::new(1);
        list.push(2);
        list.push(3);
        list.push(4);
        list.push(5);

        println!("Test 1:");
        list.iter(|l| {
            if l.value.read() == 4 {
                l.value.write(6);
            }
            println!("{}", l.value.read());
        });

        println!("Test 2:");
        list.iter_in(0..50, |l| {
            println!("{}", l.value.read());
        });
    }
}

#[derive(Debug)]
pub struct Node<T: Copy> {
    value: Value<T>,
    next: Option<NodeLink<T>>,
    pos: Option<NodeLink<T>>,
}

#[derive(Debug)]
pub struct List<T: Copy> {
    list: NodeLink<T>,
    len: usize,
}

#[derive(Debug)]
struct Value<T>(TLink<T>);

impl<T> Value<T> {
    pub fn read(&self) -> T {
        unsafe { self.0.as_ptr().read() }
    }

    pub fn write(&mut self, value: T) {
        unsafe {
            self.0.borrow_mut().as_ptr().write(value);
        }
    }
}

impl<T: Copy> Node<T> {
    pub fn new(i: T) -> Self {
        Node {
            value: Value(Rc::new(RefCell::new(i))),
            next: None,
            pos: None,
        }
    }

    pub fn set_pointer(&mut self, to: NodeLink<T>) -> &Self {
        self.next = Some(to);
        self
    }
}

type NodeLink<T> = Rc<RefCell<Node<T>>>;
type TLink<T> = Rc<RefCell<T>>;

impl<T: Copy + Debug> List<T> {
    pub fn new(init_value: T) -> Self {
        let node = Node::new(init_value);
        let red: NodeLink<T> = Rc::new(RefCell::new(node));

        let mut list = List {
            list: Rc::clone(&red),
            len: 1,
        };
        unsafe { (*list.list.borrow_mut().as_ptr()).next = Some(Rc::clone(&red)) }
        list
    }

    pub fn push(&mut self, value: T) {
        let mut node = Node::new(value);
        node.next = Some(Rc::clone(&self.list));
        let red: NodeLink<T> = Rc::new(RefCell::new(node));
        unsafe {
            let mut current: &Option<NodeLink<T>> = &(*self.list.borrow_mut().as_ptr()).next;

            for _ in 1..self.len - 1 {
                current = &(*current.as_ref().unwrap().as_ptr()).next;
            }

            //println!("{:?}", (*current.as_ref().unwrap().as_ptr()).0);
            (*current.as_ref().unwrap().as_ptr()).next = Some(Rc::clone(&red));
        }

        self.len += 1;
    }

    pub fn iter(&mut self, action: impl Fn(&mut Node<T>)) {
        unsafe {
            action(&mut (*self.list.borrow_mut().as_ptr()));
            let mut current: &Option<NodeLink<T>> = &(*self.list.borrow_mut().as_ptr()).next;
            for _ in 1..self.len {
                action(&mut (*current.as_ref().unwrap().as_ptr()));
                current = &(*current.as_ref().unwrap().as_ptr()).next;
            }
        }
    }

    pub fn iter_in(&mut self, range: Range<usize>, action: impl Fn(&mut Node<T>)) {
        unsafe {
            action(&mut (*self.list.borrow_mut().as_ptr()));
            let mut current: &Option<NodeLink<T>> = &(*self.list.borrow_mut().as_ptr()).next;
            for _ in range.start..range.end - 1 {
                action(&mut (*current.as_ref().unwrap().as_ptr()));
                current = &(*current.as_ref().unwrap().as_ptr()).next;
            }
        }
    }
}
