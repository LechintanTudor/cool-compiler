use bumpalo::Bump;

#[derive(Default, Debug)]
pub struct UnsafeBump(Bump);

impl UnsafeBump {
    /// # Safery
    /// `self` must not be shared across threads during this call.
    #[inline]
    pub unsafe fn alloc<T>(&self, value: T) -> &T {
        self.0.alloc(value)
    }

    /// # Safery
    /// `self` must not be shared across threads during this call.
    #[inline]
    pub unsafe fn alloc_slice_copy<T>(&self, values: &[T]) -> &[T]
    where
        T: Copy,
    {
        self.0.alloc_slice_copy(values)
    }

    /// # Safery
    /// `self` must not be shared across threads during this call.
    #[inline]
    pub unsafe fn alloc_slice_clone<T>(&self, values: &[T]) -> &[T]
    where
        T: Clone,
    {
        self.0.alloc_slice_clone(values)
    }

    /// # Safery
    /// `self` must not be shared across threads during this call.
    #[inline]
    pub unsafe fn alloc_str(&self, value: &str) -> &str {
        self.0.alloc_str(value)
    }
}

unsafe impl Send for UnsafeBump {}
unsafe impl Sync for UnsafeBump {}
