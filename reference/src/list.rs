use std::fmt::{Debug, Display, Formatter};
use crate::list::List::*;

#[derive(Debug)]
pub enum List<T> {
    Nil,
    Cons(T, Box<List<T>>)
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        Nil
    }

    pub fn prepend(self, item: T) -> List<T> {
        Cons(item, Box::new(self))
    }

    pub fn iter(&self) -> ListIterator<T> {
        ListIterator { list: self }
    }
}

pub struct ListIterator<'a, T: 'a> {
    list: &'a List<T>
}

impl<'a, T> Iterator for ListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.list {
            Nil => None,
            Cons(item, next) => {
                self.list = next;
                Some(item)
            }
        }
    }
}

impl<T: Debug> Display for List<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let vector: Vec<_> = self.iter().map(|item| format!("{item:?}")).collect();
        let formatted_items = vector.join(", ");
        write!(f, "[{}]", formatted_items)
    }
}
