use std::{collections::HashMap, fmt::Debug, hash::Hash};

use super::{state::State, types::Action};

pub struct Machine<K, T>
where
    T: Sync + Send,
{
    states: HashMap<K, State<K, T>>,
    pub active_state: K,
}

impl<K: Hash + Eq + Debug, T: Sync + Send> Machine<K, T> {
    pub fn new(active_state: K) -> Self {
        Self {
            states: Default::default(),
            active_state,
        }
    }

    pub fn state(mut self, id: K, state: State<K, T>) -> Self {
        self.states.insert(id, state);
        self
    }

    pub async fn set_active_state(&mut self, id: K, actions: Vec<Action<T>>, context: &mut T) {
        if let Some(state) = self.states.get_mut(&self.active_state) {
            state.state.exit(context).await;
        }
        for action in actions {
            (action)(context)
        }
        if let Some(state) = self.states.get_mut(&id) {
            state.state.entry(context).await;
            self.active_state = id;
        }
    }

    pub async fn decide(&mut self, context: &mut T)
    where
        K: Clone,
    {
        if let Some(state) = self.states.get(&self.active_state) {
            if let Some(selected) = state.decide(context).await {
                self.set_active_state(selected.to.clone(), selected.actions.clone(), context)
                    .await;
            }
        }
    }

    pub async fn update(&mut self, context: &mut T) {
        if let Some(state) = self.states.get_mut(&self.active_state) {
            state.state.update(context).await;
        }
    }
}
