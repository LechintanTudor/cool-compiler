#[macro_export]
macro_rules! id_newtype {
    ($Wrapper:ident) => {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        pub struct $Wrapper(std::num::NonZeroU32);

        impl $Wrapper {
            #[inline]
            pub const fn new(index: u32) -> Option<Self> {
                if index == 0 {
                    return None;
                }

                unsafe { Some(Self(std::num::NonZeroU32::new_unchecked(index))) }
            }

            #[inline]
            pub fn new_unwrap(index: u32) -> Self {
                Self(std::num::NonZeroU32::new(index).unwrap())
            }

            #[inline]
            pub const unsafe fn new_unchecked(index: u32) -> Self {
                Self(std::num::NonZeroU32::new_unchecked(index))
            }

            #[inline]
            pub const fn dummy() -> Self {
                unsafe { Self::new_unchecked(u32::MAX) }
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
    };
}

id_newtype!(Handle);

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
            pub fn new_unwrap(index: u32) -> Self {
                Self($crate::Handle::new_unwrap(index))
            }

            #[inline]
            pub const unsafe fn new_unchecked(index: u32) -> Self {
                Self($crate::Handle::new_unchecked(index))
            }

            #[inline]
            pub const fn dummy() -> Self {
                Self($crate::Handle::dummy())
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
