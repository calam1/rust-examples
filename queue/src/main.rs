

pub struct Queue<T> {
    in_values: Vec<T>,
    out_values: Vec<T>
}

impl<T> Queue<T> {
    pub fn push(&mut self, value: T) {
        self.in_values.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.out_values.is_empty() {
            if self.in_values.is_empty() {
                return None;
            }

            use std::mem::swap;
            swap(&mut self.out_values, &mut self.in_values);
            self.out_values.reverse();
        }

        self.out_values.pop()
    }
}

fn main() {
    println!("Hello, world!");

    let mut q = Queue {
        in_values: Vec::new(),
        out_values: Vec::new()
    };

    q.push(1);
    q.push(2);


    // assert_eq!(q.pop(), Some(1));
    println!("{:?}", q.pop());

    q.push(3);
    q.push(4);

    println!("{:?}", q.pop());
    println!("{:?}", q.pop());
    println!("{:?}", q.pop());

}
