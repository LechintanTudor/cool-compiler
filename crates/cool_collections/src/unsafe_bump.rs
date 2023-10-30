use bumpalo::Bump;

#[derive(Default, Debug)]
pub struct UnsafeBump(Bump);

impl UnsafeBump {
    /// # Safety
    /// `self` must not be shared across threads during this call.
    pub unsafe fn alloc<T>(&self, value: T) -> &T {
        self.0.alloc(value)
    }

    /// # Safety
    /// `self` must not be shared across threads during this call.
    pub unsafe fn alloc_slice_copy<T>(&self, values: &[T]) -> &[T]
    where
        T: Copy,
    {
        self.0.alloc_slice_copy(values)
    }

    /// # Safety
    /// `self` must not be shared across threads during this call.
    pub unsafe fn alloc_slice_clone<T>(&self, values: &[T]) -> &[T]
    where
        T: Clone,
    {
        self.0.alloc_slice_clone(values)
    }

    /// # Safety
    /// `self` must not be shared across threads during this call.
    pub unsafe fn alloc_str(&self, value: &str) -> &str {
        self.0.alloc_str(value)
    }
}

unsafe impl Send for UnsafeBump {
    // Empty
}

unsafe impl Sync for UnsafeBump {
    // Empty
}
