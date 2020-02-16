#![no_std]
#![feature(maybe_uninit_ref)]
#![feature(maybe_uninit_extra)]
#[cfg(test)]
extern crate typenum;
#[cfg(test)]
extern crate arraystring;

use core::mem::MaybeUninit;
use core::borrow::{Borrow, BorrowMut};
use core::fmt::{self, Debug, Display};
use core::hash::{Hash, Hasher};
use core::cmp::Ordering;
use core::ops::{Deref, DerefMut};

pub struct Inplace<T>(MaybeUninit<T>);

impl<T: Debug> Debug for Inplace<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl<T: Display> Display for Inplace<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl<T: Default> Default for Inplace<T> {
    fn default() -> Inplace<T> { T::default().into() }
}

impl<T: Clone> Clone for Inplace<T> {
    fn clone(&self) -> Inplace<T> { self.as_ref().clone().into() }
}

impl<T: Copy> Copy for Inplace<T> { }

impl<T: Hash> Hash for Inplace<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state)
    }
}

impl<T: PartialEq> PartialEq for Inplace<T> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref().eq(other.as_ref())
    }
}

impl<T: Eq> Eq for Inplace<T> { }

impl<T: Ord> Ord for Inplace<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_ref().cmp(other.as_ref())
    }
}

impl<T: PartialOrd> PartialOrd for Inplace<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_ref().partial_cmp(other.as_ref())
    }
}

impl<T> Inplace<T> {
    pub fn new(v: T) -> Inplace<T> { Inplace(MaybeUninit::new(v)) }
    
    pub fn deref_move(self) -> T { unsafe { self.0.assume_init() } }

    pub fn inplace<R>(&mut self, f: impl FnOnce(T) -> (R, T)) -> R {
        let (r, v) = f(unsafe { self.0.read() });
        self.0.write(v);
        r
    }

    pub fn inplace_(&mut self, f: impl FnOnce(T) -> T) {
        self.0.write(f(unsafe { self.0.read() }));
    }
}

impl<T> From<T> for Inplace<T> {
    fn from(v: T) -> Inplace<T> { Inplace::new(v) }
}

impl<T> Borrow<T> for Inplace<T> {
    fn borrow(&self) -> &T { unsafe { self.0.get_ref() } }
}

impl<T> BorrowMut<T> for Inplace<T> {
    fn borrow_mut(&mut self) -> &mut T { unsafe { self.0.get_mut() } }
}

impl<T> AsRef<T> for Inplace<T> {
    fn as_ref(&self) -> &T { unsafe { self.0.get_ref() } }
}

impl<T> AsMut<T> for Inplace<T> {
    fn as_mut(&mut self) -> &mut T { unsafe { self.0.get_mut() } }
}

impl<T> Deref for Inplace<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target { self.as_ref() }
}

impl<T> DerefMut for Inplace<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { self.as_mut() }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use arraystring::ArrayString;
    use typenum::{U255, U4, IsGreaterOrEqual};
    use arraystring::prelude::Capacity;

    struct UnclonableValue {
        value: ArrayString<U255>,
    }
    
    struct Container<'a> {
        value: &'a mut Inplace<UnclonableValue>,
    }
    
    fn replace<C: Capacity>(s: ArrayString<C>, f: char, t: char) -> ArrayString<C> where C: IsGreaterOrEqual<U4> {
        let mut res = ArrayString::new();
        for c in s.chars() {
            if s.len() > C::to_u8() - 4 {
                panic!();
            }
            let r = if c == f { t } else { c };
            unsafe { res.push_unchecked(r) };
        }
        res
    }
    
    fn change(v: UnclonableValue) -> UnclonableValue {
        UnclonableValue { value: replace(v.value, '0', '1') }
    }
    
    fn change_inplace(v: &mut Container) {
        v.value.inplace_(change)
    }

    #[test]
    fn it_works() {
        let mut value = UnclonableValue { value: "0123401234".into() }.into();
        let mut container = Container { value: &mut value };
        change_inplace(&mut container);
        assert_eq!(value.deref_move().value, ArrayString::from("1123411234"));
    }
}
