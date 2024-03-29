// This is actually so horrible, the sheer amount of cloning required..
// Thanks, I hate it

// Linked list
mod linkedlist {
    #![allow(dead_code)]
    use std::{cell::RefCell, rc::Rc};

    #[derive(Clone)]
    pub struct Node<T: Clone>(PNode<T>);

    type PNode<T> = Rc<RefCell<_Node<T>>>;

    struct _Node<T: Clone> {
        value: T,
        next: Option<Node<T>>,
    }

    impl<T: Clone> Node<T> {
        pub fn new(value: T) -> Self {
            Self(Rc::new(RefCell::new(_Node::new(value))))
        }

        pub fn push(&mut self, value: T) {
            let mut node = self.clone();
            while let Some(ref n) = node.clone().0.borrow().next {
                node = n.clone();
            }
            let new = Self::new(value);
            node.0.borrow_mut().next = Some(new);
        }

        pub fn get(&self) -> T {
            self.0.borrow().value.clone()
        }

        pub fn next(&self) -> Option<Node<T>> {
            self.0.borrow().next.clone()
        }
    }

    impl<T: Clone> _Node<T> {
        fn new(value: T) -> Self {
            Self { value, next: None }
        }
    }

    pub struct NodeIterator<T: Clone> {
        curr: Option<Node<T>>,
    }

    impl<T: Clone> NodeIterator<T> {}

    impl<T: Clone> Iterator for NodeIterator<T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            self.curr.as_ref()?;

            let current_value = self.curr.clone().unwrap().0.borrow().value.to_owned();
            if let Some(ref next) = self.curr.clone().unwrap().0.borrow().next {
                self.curr = Some(next.to_owned());
            } else {
                self.curr = None;
            }
            Some(current_value)
        }
    }

    impl<T: Clone> IntoIterator for Node<T> {
        type Item = T;

        type IntoIter = NodeIterator<T>;

        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter { curr: Some(self) }
        }
    }
}


fn main() {
    let mut head = linkedlist::Node::new(0i32);

    // Append some values
    for i in 1..=30 {
        head.push(i);
    }

    // Iterate through it
    for n in head {
        println!("{}", n);
    }
}
