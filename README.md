USE [`replace_with`](https://crates.io/crates/replace_with) INSTEAD!

![travis](https://travis-ci.org/A1-Triard/inplace.svg?branch=master)

# inplace

USE [`replace_with`](https://crates.io/crates/replace_with) INSTEAD!

A container that allows you temporarily take ownership of the stored value.

Sometimes you can find yourself in situation when you have mutable reference to some value `a`,
and function kind of `a -> a`, and you want mutate referenced value with this function. So, you can think,
you need some empowered version of `mem:replace` function, which could be named as `inplace` and should look like

```rust
fn inplace<T>(a: &mut T, f: impl FnOnce(T) -> T);
```

But such function does not exist! And even further: it _cannot_ exist.
So in such situation, you need to change some part of your code: either part, providing mutable reference,
either mutating function. The second way is conventional: normally, mutable reference is the only adequate way to
express temporary owning, thus instead of `FnOnce(T) -> T`, it should be `FnOnce(&mut T)`.
But in some situations it can be better to stay with `FnOnce(T) -> T`, and change another part, namely `&mut T`.

This crate provides special type wrap `Inplace<T>`, which have same size and memory representation as source type `T`,
but have desired `inplace` function. So, changing `&mut T` to `&mut Inplace<T>` solves the problem.
