use std::num::NonZeroU32;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Handle(NonZeroU32);

impl Handle {
    #[inline]
    pub const fn new(index: u32) -> Option<Self> {
        if index == 0 {
            return None;
        }

        unsafe { Some(Self(NonZeroU32::new_unchecked(index))) }
    }

    #[inline]
    pub const fn new_unchecked(index: u32) -> Self {
        unsafe { Self(NonZeroU32::new_unchecked(index)) }
    }

    #[inline]
    pub const fn index(&self) -> u32 {
        self.0.get()
    }

    #[inline]
    pub const fn as_usize(&self) -> usize {
        self.0.get() as usize
    }
}

#[macro_export]
macro_rules! handle_newtype {
    ($Wrapper:ident) => {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $Wrapper(pub(crate) $crate::Handle);

        impl $Wrapper {
            #[inline]
            pub const fn new(index: u32) -> Option<Self> {
                if index == 0 {
                    return None;
                }

                unsafe { Some(Self($crate::Handle::new_unchecked(index))) }
            }

            #[inline]
            pub const unsafe fn new_unchecked(index: u32) -> Self {
                Self($crate::Handle::new_unchecked(index))
            }

            #[inline]
            pub const fn index(&self) -> u32 {
                self.0.index()
            }

            #[inline]
            pub const fn as_usize(&self) -> usize {
                self.0.as_usize()
            }
        }
    };
    ($Wrapper:ident; Debug) => {
        handle_newtype!($Wrapper);

        impl std::fmt::Debug for $Wrapper {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.debug_tuple(stringify!($Wrapper))
                    .field(&self.index())
                    .finish()
            }
        }
    };
}
