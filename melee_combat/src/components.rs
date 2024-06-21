use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovingRandomly;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

/// Naming Monsters
/// * Structs can also be `tuples`
/// * only have one piece of infomation to store inside a struct
/// # Example
/// ```rust
/// #[derive(Clone, PartialEq)]
///pub struct Name {
///    pub name: String,
///}
/// ```
#[derive(Clone, PartialEq)]
pub struct Name(pub String);
