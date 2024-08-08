use std::str::FromStr;

use bevy::prelude::{KeyCode, Resource};
use serde::{de::Visitor, Deserialize, Serialize};

use super::ConfigTag;

#[derive(Resource, Serialize, Deserialize)]
pub struct KeyBinds {
    pub up: KeyBind,
    pub left: KeyBind,
    pub down: KeyBind,
    pub right: KeyBind,
    pub next_level: KeyBind,
    pub prev_level: KeyBind,
}
impl ConfigTag for KeyBinds {}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseKeyBindError;
pub struct KeyBind(pub KeyCode);
impl FromStr for KeyBind {
    type Err = ParseKeyBindError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let key_code = match s {
            "KeyW" => KeyCode::KeyW,
            "KeyA" => KeyCode::KeyA,
            "KeyS" => KeyCode::KeyS,
            "KeyD" => KeyCode::KeyD,
            "ArrowDown" => KeyCode::ArrowDown,
            "ArrowUp" => KeyCode::ArrowUp,
            "KeyP" => KeyCode::KeyP,
            "KeyO" => KeyCode::KeyO,
            _ => return Err(ParseKeyBindError),
        };
        return Ok(KeyBind(key_code));
    }
}
impl Serialize for KeyBind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self.0 {
            KeyCode::KeyW => serializer.serialize_str("KeyW"),
            KeyCode::KeyA => serializer.serialize_str("KeyA"),
            KeyCode::KeyS => serializer.serialize_str("KeyS"),
            KeyCode::KeyD => serializer.serialize_str("KeyD"),
            KeyCode::ArrowDown => serializer.serialize_str("ArrowDown"),
            KeyCode::ArrowUp => serializer.serialize_str("ArrowUp"),
            KeyCode::KeyP => serializer.serialize_str("KeyP"),
            KeyCode::KeyO => serializer.serialize_str("KeyO"),
            _ => panic!("Unable to serialize KeyCode {:?}", self.0),
        }
    }
}
impl<'de> Deserialize<'de> for KeyBind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        pub struct KeyBindVisitor;
        impl<'de> Visitor<'de> for KeyBindVisitor {
            type Value = KeyBind;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                return formatter.write_str("A valid KeyCode enum variant");
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                return self.visit_str(&v);
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match KeyBind::from_str(v) {
                    Ok(key_bind) => return Ok(key_bind),
                    Err(err) => panic!("Failed to parse str {v}, {err:?}"),
                }
            }
        }

        return Ok(deserializer.deserialize_string(KeyBindVisitor)?);
    }
}
