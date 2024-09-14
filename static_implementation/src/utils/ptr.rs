use std::cell::RefCell;
use std::rc::Rc;

pub type Ptr<T> = Rc<RefCell<T>>;

pub fn new_ptr<T>(x: T) -> Ptr<T> {
    Rc::new(RefCell::new(x))
}

pub fn copy_ptr<T: ?Sized>(x: &Ptr<T>) -> Ptr<T> {
    Rc::clone(x)
}
