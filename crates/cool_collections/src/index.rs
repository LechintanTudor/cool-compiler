use std::num::NonZeroU32;

/// Strongly-typed index for accessing elements in an [`Arena`].
///
/// # Safety
/// [`ArenaIndex::get`] must return the value passed in [`ArenaIndex::new`].
pub unsafe trait CoolIndex: Copy {
    fn new(value: NonZeroU32) -> Self;

    #[must_use]
    fn get(&self) -> NonZeroU32;

    #[inline]
    #[must_use]
    fn get_index(&self) -> usize {
        self.get().get() as usize - 1
    }
}

#[macro_export]
macro_rules! define_index_newtype {
    ($Ident:ident) => {
        define_index_newtype!($Ident; NoDebug);

        impl ::std::fmt::Debug for $Ident {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_tuple(stringify!($Ident)).field(&self.0).finish()
            }
        }
    };
    ($Ident:ident; NoDebug) => {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $Ident(::std::num::NonZeroU32);

        impl $Ident {
            pub const LAST: Self = Self(::std::num::NonZeroU32::MAX);

            #[inline]
            pub const unsafe fn new_unchecked(index: u32) -> Self {
                Self(::std::num::NonZeroU32::new_unchecked(index))
            }
        }

        unsafe impl $crate::CoolIndex for $Ident {
            #[inline]
            fn new(value: ::std::num::NonZeroU32) -> Self {
                Self(value)
            }

            #[inline]
            fn get(&self) -> ::std::num::NonZeroU32 {
                self.0
            }
        }
    };
}
