use crate::prelude::*;
use ron::de::from_reader;
use serde::Deserialize;
use std::collections::HashSet;
use std::fs::File;

#[derive(Clone, Deserialize, Debug)]
pub struct Template {
    pub entity_type: EntityType,
    pub levels: HashSet<usize>,
    pub frequency: i32,
    pub name: String,
    pub glyph: char,
    pub provides: Option<Vec<(String, i32)>>,
    pub hp: Option<i32>,
    pub base_damage: Option<i32>,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub enum EntityType {
    Enemy,
    Item,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Templates {
    pub entities: Vec<Template>,
}

impl Templates {
    pub fn load() -> Self {
        let file =
            File::open("/Users/SM1701/rust-workspace/head_first_rust/loot/resources/template.ron")
                .expect("Failed opening file");
        from_reader(file).expect("Unable to load templates")
    }

    pub fn spawn_entities(
        &self, // (1)
        ecs: &mut World,
        rng: &mut RandomNumberGenerator,
        level: usize,
        spawn_points: &[Point],
    ) {
        // list all of entites
        let mut available_entities = Vec::new();
        self.entities
            .iter()
            .filter(|e| e.levels.contains(&level))
            .for_each(|t| {
                for _ in 0..t.frequency {
                    available_entities.push(t);
                }
            });
        // spawn entities
        let mut cb = CommandBuffer::new(ecs);
        spawn_points.iter().for_each(|pt| {
            if let Some(entity) = rng.random_slice_entry(&available_entities) {
                self.spawn_entity(pt, entity, &mut cb);
            }
        });
        cb.flush(ecs, &mut Resources::default());
    }

    fn spawn_entity(&self, pt: &Point, tpl: &Template, cb: &mut CommandBuffer) {
        let entity = cb.push((
            pt.clone(),
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437(tpl.glyph),
            },
            Name(tpl.name.clone()),
        ));

        match tpl.entity_type {
            EntityType::Item => {
                cb.add_component(entity, Item {});
            }
            EntityType::Enemy => {
                cb.add_component(entity, Enemy {});
                cb.add_component(entity, FieldOfView::new(6));
                cb.add_component(entity, ChasingPlayer {});
                cb.add_component(
                    entity,
                    Health {
                        current: tpl.hp.unwrap(),
                        max: tpl.hp.unwrap(),
                    },
                );
            }
        }

        if let Some(effects) = &tpl.provides {
            effects
                .iter()
                .for_each(|(provide, n)| match provide.as_str() {
                    "Healing" => cb.add_component(entity, ProvidesHealing { amount: *n }),
                    "MagicMap" => cb.add_component(entity, ProvidesDungeonMap {}),
                    _ => {
                        println!("Warning: we don't know how to provide {}", provide)
                    }
                });
        }

        //damge
        if let Some(damage) = &tpl.base_damage {
            cb.add_component(entity, Damage(*damage));
            if tpl.entity_type == EntityType::Item {
                cb.add_component(entity, Weapon {});
            }
        }
    }
}
