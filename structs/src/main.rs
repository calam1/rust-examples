use std::rc::Rc;
use std::borrow::BorrowMut;

fn main() {
    println!("Hello, world!");

    // struct Node {
    //     tag: String,
    //     children: Vec<Rc<Node>>
    // }
    //
    // impl Node {
    //     fn new(tag: &str) -> Node {
    //         Node {
    //             tag: tag.to_string(),
    //             children: vec![],
    //         }
    //     }
    //
    //     fn append_to(self: Rc<Node>, parent: &mut Node) {
    //         parent.children.push(self);
    //     }
    // }
    //
    // let mut parent = Node::new("parent");
    // let shared_node = Rc::new(Node::new("first"));
    // shared_node.append_to(&mut parent);


    struct Extrema<'elt> {
        greatest: &'elt i32,
        least: &'elt i32,
    }

    fn find_extrema(slice: &[i32]) -> Extrema {
        let mut greatest = &slice[0];
        let mut least = &slice[0];

        for i in 1..slice.len() {
            if slice[i] < *least { least = &slice[i]; }
            if slice[i] > *greatest { greatest = &slice[i]; }
        }

        Extrema { greatest, least }
    }

    let a = [0, -3, 0, 15, 48];
    let e = find_extrema(&a);
    assert_eq!(*e.least, -3);
    assert_eq!(*e.greatest, 48);
}
