// https://aloso.github.io/2021/04/12/linked-list.html
fn main() {

}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq )]
pub enum List<'a, T> {
    Node {
        data: T,
        next: &'a List<'a, T>,
    },
    Tail
}

impl<T> Default for List<'_, T> {
    fn default() -> Self {
        List::Tail
    }
}

// persistent data structure
impl<'a, T> List<'a, T> {
    pub fn add(&'a self, data: T) -> Self {
        List::Node {
            data,
            next: self,
        }
    }

    // internal iterator
    pub fn rev_iter(&'a self, f: impl Fn(&'a T)) {
        if let List::Node { data, next} = self {
            next.rev_iter(&f);
            f(data)
        }
    }

    pub fn try_rev_iter<E, F>(&'a self, f: F) -> Result<(), E>
        where
            F: Fn(&'a T) -> Result<(), E>,
        {
            if let List::Node { data, next} = self {
                next.try_rev_iter(&f)?;
                f(data)?;
            }

            Ok(())
        }
}

// This is a by-reference iterator. Writing an iterator that owns the element is not possible.
// This is a fundamental limitation of this list type, since the nodes are behind shared references.
impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            List::Node { data, next } => {
                self.0 = next;
                Some(data)
            }
            List::Tail => None
        }
    }
}

impl<'a, T> IntoIterator for &'a List<'a, T> {
    type Item = &'a T;

    type IntoIter = ListIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        ListIter(self)
    }
}

pub struct ListIter<'a, T> (&'a List<'a, T>);

#[test]
fn test_add() {
    let tail = List::Tail;
    let first = tail.add(5);
    let second = first.add(10);

    assert_eq!(tail, List::Tail);
    assert_eq!(first, List::Node {
        data: 5,
        next: &List::Tail,
    });
    assert_eq!(second, List::Node {
        data: 10,
        next: &List::Node {
            data: 5,
            next: &List::Tail,
        },
    });
}