use std::rc::Rc;

pub mod first;
pub mod second;
pub mod third;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

// pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}


impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn append(&self, elem: T) -> List<T> {
        List { head: Some(Rc::new(Node {
            elem: elem,
            next: self.head.clone(),
        }))}
    }

    pub fn tail(&self) -> List<T> {
        List { head: self.head.as_ref().and_then(|node| node.next.clone())}
    }

    // pub fn push(&mut self, elem: T) {
    //     let new_node = Box::new(Node {
    //         elem: elem,
    //         next: self.head.take(),
    //     });
    //
    //     self.head = Link::Some(new_node);
    // }
    //
    // pub fn pop(&mut self) -> Option<T> {
    //     self.head.take().map(|node| {
    //         self.head = node.next;
    //         node.elem
    //     })
    // }

    // pub fn peek(&self) -> Option<&T> {
    //     // self.head.map(|node| {
    //     self.head.as_ref().map(|node| &node.elem)
    // }
    //
    // pub fn peek_mut(&mut self) -> Option<&mut T> {
    //     self.head.as_mut().map(|node| &mut node.elem)
    // }
    //
    // pub fn into_iter(self) -> IntoIter<T> {
    //     IntoIter(self)
    // }
    //
    // pub fn iter<'a>(&'a self) -> Iter<'a, T> {
    //     Iter { next: self.head.as_deref() }
    // }
}

// impl<T> Iterator for IntoIter<T> {
//     type Item = T;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.0.pop()
//     }
// }
//
// impl<'a, T> Iterator for Iter<'a, T> {
//     type Item = &'a T;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         self.next.map(|node| {
//             self.next = node.next.as_deref();
//             &node.elem
//         })
//     }
// }
//
// impl<T> Drop for List<T> {
//     fn drop(&mut self) {
//         let mut cur_link = self.head.take();
//         while let Link::Some(mut boxed_node) = cur_link {
//             cur_link = boxed_node.next.take();
//         }
//     }
// }

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}
