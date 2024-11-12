use std::marker::PhantomData;

/// A trait to track whether a vector is empty or not
pub trait Emptiness {}

#[derive(Debug)]
pub struct Empty;

#[derive(Debug)]
pub struct NonEmpty;

impl Emptiness for Empty {}
impl Emptiness for NonEmpty {}

#[derive(Debug)]
pub struct AwareVec<T, E: Emptiness> {
    vec: Vec<T>,
    phantom_data: PhantomData<E>
}

impl<T> AwareVec<T, Empty> {
    pub fn new() -> AwareVec<T, Empty> {
        AwareVec {
            vec: Vec::new(),
            phantom_data: PhantomData,
        }
    }

    pub fn push(self, item: T) -> AwareVec<T, NonEmpty> {
        let mut vec = self.vec;
        vec.push(item);
        AwareVec {
            vec,
            phantom_data: PhantomData
        }
    }
}

impl<T> AwareVec<T, NonEmpty> {
    #[allow(dead_code)]
    pub fn of(item: T) -> AwareVec<T, NonEmpty> {
        AwareVec {
            vec: vec![item],
            phantom_data: PhantomData
        }
    }

    pub fn push(self, item: T) -> AwareVec<T, NonEmpty> {
        let mut vec = self.vec;
        vec.push(item);
        AwareVec {
            vec,
            phantom_data: PhantomData
        }
    }
    
    pub fn head(&self) -> &T {
        self.vec.get(0).unwrap()
    }

    pub fn tail(&self) -> &T {
        self.vec.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_new() {
        let vec = AwareVec::new();
        // vec.tail() - isn't even implemented for the type, it's impossible to call
        let vec = vec.push(1).push(2);
        assert_ne!(format!("{vec:?}"), "");
        assert_eq!(*vec.head(), 1);
        assert_eq!(*vec.tail(), 2);
    }

    #[test]
    fn test_vec_of() {
        let vec = AwareVec::of(1);
        assert_eq!(*vec.tail(), 1);
        let vec = vec.push(5);
        assert_eq!(*vec.head(), 1);
        assert_eq!(*vec.tail(), 5);
    }
}