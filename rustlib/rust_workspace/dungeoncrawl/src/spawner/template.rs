use std::collections::HashSet;

use crate::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Template {
    pub entity_type: EntityType,
    pub levels: HashSet<usize>,
    pub frequency: i8,
    pub name: String,
    pub texture: RenderKey,
    pub provides: Option<Vec<(String, i32)>>,
    pub hp: Option<i32>,
}

#[derive(Deserialize, Clone, Copy, PartialEq, Debug)]
pub enum EntityType {
    Enemy,
    Item,
}

#[derive(Resource, Deserialize, Debug)]
pub struct Templates {
    entities: Vec<Template>,
}

impl Templates {
    pub async fn load() -> Self {
        let template_file = load_file("template.ron").await.expect("Template file");
        let t = String::from_utf8(template_file.clone()).unwrap();
        debug!("{}", t);
        match ron::de::from_bytes(&template_file) {
            Ok(template) => template,
            Err(error) => {
                error!("{}", error);
                panic!("{}", error);
            }
        }
    }

    pub fn spawn_entities(
        &self,
        commands: &mut Commands,
        rng: &mut Rng,
        level: usize,
        spawn_points: &[TilePoint],
    ) {
        let mut available_entities = Vec::new();
        self.entities
            .iter()
            .filter(|e| e.levels.contains(&level))
            .for_each(|e| {
                for _ in 0..e.frequency {
                    available_entities.push(e)
                }
            });

        spawn_points.iter().for_each(|pt| {
            if let Some(entity) = rng.choice(&available_entities) {
                self.spawn_entity(pt, entity, commands);
            }
        });
    }

    fn spawn_entity(&self, pt: &TilePoint, template: &Template, commands: &mut Commands) {
        let mut entity = commands.spawn((
            *pt,
            Render {
                texture: template.texture,
            },
            EntityName(template.name.clone()),
        ));

        match template.entity_type {
            EntityType::Item => {
                entity.insert(Item);
            }
            EntityType::Enemy => {
                entity
                    .insert(Enemy)
                    .insert(FieldOfView::new(6))
                    .insert(ChasePlayer)
                    .insert(Health {
                        current: template.hp.unwrap(),
                        max: template.hp.unwrap(),
                    });
            }
        }

        if let Some(effects) = &template.provides {
            effects
                .iter()
                .for_each(|(effect, n)| match effect.as_str() {
                    "Healing" => {
                        entity.insert(ProvidesHealing { amount: *n });
                    }

                    "Map" => {
                        entity.insert(ProvidesDungeonMap);
                    }

                    _ => error!("Unsupported Effect: {}", effect),
                });
        }
    }
}
