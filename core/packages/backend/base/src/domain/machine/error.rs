use thiserror::Error;

#[derive(Error, Debug)]
pub enum StateMachineError {
    #[error("the state machine has a transition defined with the same to and condition: ({0})")]
    DuplicateTransition(String),
    #[error("more than one transition defines the same condition, which means only the first will be honored: ({0})")]
    DuplicateCondition(String),
}
