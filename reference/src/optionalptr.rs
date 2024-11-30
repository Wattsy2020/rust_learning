pub struct OptionalPtr<T> {
    ptr: *const T
}

impl<T> OptionalPtr<T> {
    pub const fn none() -> OptionalPtr<T> {
        OptionalPtr {
            ptr: std::ptr::null()
        }
    }

    pub const fn some(item: &T) -> OptionalPtr<T> {
        OptionalPtr {
            ptr: item as *const T
        }
    }
    
    pub unsafe fn get(&self) -> Option<&T> {
        self.ptr.as_ref()
    }
}

