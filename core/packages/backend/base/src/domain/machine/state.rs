use super::{
    traits::TState,
    transition::Transition,
    types::{Action, Condition},
};

pub struct State<K, T>
where
    T: Sync + Send,
{
    pub state: Box<dyn TState<T> + Sync + Send>,
    pub transitions: Vec<Transition<K, T>>,
}

impl<K, T: Sync + Send> State<K, T> {
    pub fn new<S: TState<T> + Sync + Send + 'static>(state: S) -> Self {
        Self {
            state: Box::new(state),
            transitions: vec![],
        }
    }

    pub fn transition(mut self, to: K, condition: Condition<T>, actions: Vec<Action<T>>) -> Self {
        self.transitions.push(Transition {
            to,
            condition,
            actions,
        });
        self
    }

    pub async fn decide(&self, context: &mut T) -> Option<&Transition<K, T>>
    where
        K: Clone,
    {
        for transition in self.transitions.iter() {
            if (transition.condition)(context) {
                return Some(transition);
            }
        }
        None
    }
}
