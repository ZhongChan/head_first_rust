use std::collections::HashSet;
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToAttact {
    pub attacter: Entity,
    pub victim: Entity,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChasingPlayer;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Item;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AmuletOfYala;

#[derive(Clone, Debug, PartialEq)]
pub struct FiledOfView {
    pub visible_tiles: HashSet<Point>,
    pub radius: i32,
    pub is_dirty: bool,
}

impl FiledOfView {
    pub fn new(radius: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius,
            is_dirty: true,
        }
    }

    pub fn clone_dirty(&self) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius: self.radius,
            is_dirty: true,
        }
    }
}