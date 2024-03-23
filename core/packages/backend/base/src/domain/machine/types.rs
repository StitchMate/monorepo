pub type Condition<T> = fn(&mut T) -> bool;

pub type Action<T> = fn(&mut T) -> ();
