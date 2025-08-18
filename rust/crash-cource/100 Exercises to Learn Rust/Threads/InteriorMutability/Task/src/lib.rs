// TODO: Use `Rc` and `RefCell` to implement `DropTracker<T>`, a wrapper around a value of type `T`
//  that increments a shared `usize` counter every time the wrapped value is dropped.

use std::cell::RefCell;
use std::rc::Rc;

pub struct DropTracker<T> {
    value: T,
    counter: /* TODO */
}

impl<T> DropTracker<T> {
    pub fn new(value: T, counter: /* TODO */) -> Self {
        Self { value, counter }
    }
}

impl<T> Drop for DropTracker<T> {
    fn drop(&mut self) {
        /* TODO */
    }
}
