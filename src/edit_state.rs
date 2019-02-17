use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer};


/**
 * A struct representing the edited field in a reddit comment
 */
#[derive(Debug)]
pub enum EditState {
    Bool(bool),
    UTC(u32),
}

/**
 * Visitor pattern to deserialize EditState
 */
struct EditStateVisitor;

impl<'de> Visitor<'de> for EditStateVisitor {
    type Value = EditState;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a boolean or an unsigned 32-bits integer")
    }

    fn visit_bool<E>(self, value: bool) -> Result<EditState, E>
    where
        E: de::Error,
    {
        Ok(EditState::Bool(value))
    }

    fn visit_u64<E>(self, value: u64) -> Result<EditState, E>
    where
        E: de::Error,
    {
        Ok(EditState::UTC(value as u32))
    }
}

impl<'de> Deserialize<'de> for EditState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(EditStateVisitor)
    }
}
