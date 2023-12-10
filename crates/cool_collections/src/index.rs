/// Trait for defining strongly-typed indexes.
///
/// # Safety
/// [`CoolIndex::get`] must return the value passed to [`CoolIndex::new`]
pub unsafe trait CoolIndex: Copy {
    fn new(value: u32) -> Self;

    #[must_use]
    fn get(&self) -> u32;
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
        pub struct $Ident(u32);

        impl $Ident {
            #[inline]
            pub const fn new(value: u32) -> Self {
                Self(value)
            }
        }

        unsafe impl $crate::CoolIndex for $Ident {
            #[inline]
            fn new(value: u32) -> Self {
                Self(value)
            }

            #[inline]
            fn get(&self) -> u32 {
                self.0
            }
        }
    };
}
