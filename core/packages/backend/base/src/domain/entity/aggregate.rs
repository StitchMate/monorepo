use std::fmt::Debug;

use super::event::Event;

pub trait Aggregate: Debug + Clone {
    type Event: Event;

    fn aggregate_type(&self) -> String;

    fn aggregate_id(&self) -> Option<String>;
}
