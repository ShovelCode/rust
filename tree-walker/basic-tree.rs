use std::cmp::Ord;
use std::cell::RefCell;
use std::rc::Rc;

type Tree<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Tree<T>,
    right: Tree<T>,
}

impl<T: Ord> Node<T> {
    fn insert(&mut self, value: T) {
        if value < self.value {
            if self.left.is_none() {
                self.left = Some(Rc::new(RefCell::new(Node {
                    value: value,
                    left: None,
                    right: None,
                })));
            } else {
                self.left.as_ref().unwrap().borrow_mut().insert(value);
            }
        } else if value > self.value {
            if self.right.is_none() {
                self.right = Some(Rc::new(RefCell::new(Node {
                    value: value,
                    left: None,
                    right: None,
                })));
            } else {
                self.right.as_ref().unwrap().borrow_mut().insert(value);
            }
        }
    }
}

fn main() {
    let mut root = Node {
        value: 50,
        left: None,
        right: None,
    };

    root.insert(30);
    root.insert(20);
    root.insert(40);
    root.insert(70);
    root.insert(60);
    root.insert(80);

    println!("{:#?}", root);
}
