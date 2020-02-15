#![feature(maybe_uninit_ref)]
#![feature(maybe_uninit_extra)]
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
    
    struct UnclonableValue {
        value: String,
    }
    
    struct Container<'a> {
        value: &'a mut Inplace<UnclonableValue>,
    }
    
    fn change(v: UnclonableValue) -> UnclonableValue {
        UnclonableValue { value: v.value.replace("0", "1") }
    }
    
    fn change_inplace(v: &mut Container) {
        v.value.inplace_(change)
    }

    #[test]
    fn it_works() {
        let mut value = UnclonableValue { value: "0123401234".into() }.into();
        let mut container = Container { value: &mut value };
        change_inplace(&mut container);
        assert_eq!(value.take().value, "1123411234");
    }
}
