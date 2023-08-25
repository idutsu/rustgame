use std::{collections::HashMap, hash::Hash, vec};
use image::DynamicImage;
use minifb::{Window, Key};
mod game;
use game::{Game, GameBase};
mod render;
use render::Render;
mod mode;
use mode::Mode;
mod entity;
use entity::{Entity, EntityKey, EntityType, Player, Enemy};

const PLAYER_IMAGE_PATH:&str = "player.png";
const ENEMY_IMAGE_PATH:&str = "enemy.png";


impl GameBase for Game {

    fn set_assets(&self) -> Vec<String> {
        let mut assets = Vec::new();
        assets.push(PLAYER_IMAGE_PATH.to_string());
        assets.push(ENEMY_IMAGE_PATH.to_string());
        assets
    }

    fn set_entities(&self) -> HashMap<EntityKey, Entity> {

        let mut entities = HashMap::new();

        if let Some(player_image) = self.get_image(PLAYER_IMAGE_PATH) {
            let player = Entity::Player(
                Player {
                    image: player_image,
                    x: 0.0,
                    y: 0.0,
                }
            );
            entities.insert(EntityKey{id: 0, entity_type: EntityType::Player}, player);
        }

        if let Some(enemy_image) = self.get_image(ENEMY_IMAGE_PATH) {
            let enemy = Entity::Enemy(
                Enemy {
                    image: enemy_image,
                    x: 0.0,
                    y: 0.0,
                }
            );
            entities.insert(EntityKey{id: 1, entity_type: EntityType::Enemy}, enemy);
        }
        
        entities

    }

    fn update_start(&mut self, window: &Window, mode: &mut Mode, entities: &mut HashMap<EntityKey, Entity>) {
        *mode = Mode::Play;   
        println!("Updating in start mode. State:");
    }

    fn draw_start(&mut self, render: &mut Render, entities: &mut HashMap<EntityKey, Entity>) {
        println!("Drawing in start mode. State:");
    }

    fn update_play(&mut self, window: &Window, mode: &mut Mode, entities: &mut HashMap<EntityKey, Entity>) {
        
        if let Some(entity) = entities.get_mut(&EntityKey{id:0,entity_type:EntityType::Player}) {
            entity.update(window);
        }

        if let Some(entity) = entities.get_mut(&EntityKey{id:1,entity_type:EntityType::Enemy}) {
            entity.update(window);
        }
       
        println!("Updating in play mode. State:");
    }

    fn draw_play(&mut self, render: &mut Render, entities: &mut HashMap<EntityKey, Entity>) {

        if let Some(entity) = entities.get_mut(&EntityKey{id:0,entity_type:EntityType::Player}) {
            entity.draw(render);
        }
    
        if let Some(entity) = entities.get_mut(&EntityKey{id:1,entity_type:EntityType::Enemy}) {
            entity.draw(render);
        }

        println!("Drawing in play mode. State:");
    }

    fn update_over(&mut self, window: &Window, mode: &mut Mode, entities: &mut HashMap<EntityKey, Entity>) {
        println!("Updating in over mode. State:");
    }

    fn draw_over(&mut self, render: &mut Render, entities: &mut  HashMap<EntityKey, Entity>) {
        let red = (255u32 << 16) | (0u32 << 8) | 0u32;
        render.color(red);
        println!("Drawing in over mode. State:");
    }
}

fn main() {
    Game::new("My Game", 500, 500).start();
}
