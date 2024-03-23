use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct RESTResponse<T: Clone, E: Clone> {
    data: Option<T>,
    errors: Option<Vec<E>>,
}

impl<T: Clone, E: Clone> RESTResponse<T, E> {
    fn new(data: Option<T>, errors: Option<Vec<E>>) -> Self {
        Self { data, errors }
    }
    pub fn data(&self) -> Option<T> {
        return self.data.clone();
    }
    pub fn errors(&self) -> Option<Vec<E>> {
        return self.errors.clone();
    }
}

#[derive(Default)]
pub struct RESTResponseBuilder<T: Clone, E: Clone> {
    data: Option<T>,
    errors: Option<Vec<E>>,
}

impl<T: Clone, E: Clone> RESTResponseBuilder<T, E> {
    pub fn new() -> Self {
        Self {
            data: None,
            errors: None,
        }
    }
    pub fn data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }
    pub fn errors(mut self, errors: Vec<E>) -> Self {
        self.errors = Some(errors);
        self
    }
    pub fn build(self) -> RESTResponse<T, E> {
        RESTResponse::new(self.data, self.errors)
    }
}
