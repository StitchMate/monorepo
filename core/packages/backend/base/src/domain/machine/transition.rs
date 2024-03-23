use super::types::{Action, Condition};

pub struct Transition<K, T> {
    pub to: K,
    pub condition: Condition<T>,
    pub actions: Vec<Action<T>>,
}
