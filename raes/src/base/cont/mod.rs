mod entity;
mod row;

const DEFAULT_MAX_ENTITIES: usize = 128;

pub trait Cont {}

pub use entity::{ContEntities, Entity};
pub use row::Row;
