#![no_std]
#![feature(maybe_uninit_ref)]
#![feature(maybe_uninit_extra)]
#[cfg(test)]
extern crate typenum;
#[cfg(test)]
extern crate arraystring;

use core::mem::MaybeUninit;
use core::borrow::{Borrow, BorrowMut};

pub struct Inplace<T>(MaybeUninit<T>);

impl<T> Inplace<T> {
    pub fn take(self) -> T { unsafe { self.0.assume_init() } }

    pub fn inplace<R>(&mut self, f: impl FnOnce(T) -> (T, R)) -> R {
        let (v, r) = f(unsafe { self.0.read() });
        self.0.write(v);
        r
    }

    pub fn inplace_(&mut self, f: impl FnOnce(T) -> T) {
        self.0.write(f(unsafe { self.0.read() }));
    }
}

impl<T> From<T> for Inplace<T> {
    fn from(v: T) -> Inplace<T> { Inplace(MaybeUninit::new(v)) }
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
        assert_eq!(value.take().value, ArrayString::from("1123411234"));
    }
}
