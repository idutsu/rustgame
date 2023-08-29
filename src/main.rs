use std::{collections::HashMap, hash::Hash, vec};
use image::DynamicImage;
use minifb::{Window, Key};
mod game;
use game::{Game, GameBase, EntityHashMap, ImageFilePaths};
mod render;
use render::Render;
mod mode;
use mode::Mode;
mod entity;
use entity::{Entity, EntityKey, EntityType, Player, Enemy};

impl GameBase for Game {

    fn add_images(&self) -> ImageFilePaths {
        let mut assets = Vec::new();
        assets.push(String::from("player.png"));
        assets.push(String::from("enemy.png"));
        assets
    }

    fn add_entities(&self) -> EntityHashMap {

        let game_width= self.width as f32;
        let game_height= self.height as f32;

        let mut entities = HashMap::new();

        if let Some(player_image) = self.get_image("player.png") {
            let player_width = player_image.width() as f32;
            let player_height = player_image.height() as f32;
            let player = Entity::Player(
                Player {
                    image : player_image,
                    x     : (game_width - player_width)*0.5,
                    y     : game_height - player_height,
                }
            );
            entities.insert(EntityKey{id: 0, entity_type: EntityType::Player}, player);
        }

        if let Some(enemy_image) = self.get_image("enemy.png") {
            let enemy_width = enemy_image.width() as f32;
            let enemy = Entity::Enemy(
                Enemy {
                    image : enemy_image,
                    x     : (game_width - enemy_width)*0.5,
                    y     : 0.0,
                }
            );
            entities.insert(EntityKey{id: 1, entity_type: EntityType::Enemy}, enemy);
        }
        
        entities

    }

    fn update_start(&mut self, window: &Window, mode: &mut Mode, entities: &mut EntityHashMap) {
        if window.is_key_down(Key::Key1) {
            *mode = Mode::Play;   
        }
    }

    fn draw_start(&mut self, render: &mut Render, entities: &mut EntityHashMap) {
        println!("Drawing in start mode. State:");
    }

    fn update_play(&mut self, window: &Window, mode: &mut Mode, entities: &mut EntityHashMap) {
        
        if let Some(entity) = entities.get_mut(&EntityKey{id:0,entity_type:EntityType::Player}) {
            entity.update(self, window);
        }

        if let Some(entity) = entities.get_mut(&EntityKey{id:1,entity_type:EntityType::Enemy}) {
            entity.update(self, window);
        }
       
        // println!("Updating in play mode. State:");
    }

    fn draw_play(&mut self, render: &mut Render, entities: &mut EntityHashMap) {

        if let Some(entity) = entities.get_mut(&EntityKey{id:0,entity_type:EntityType::Player}) {
            entity.draw(render);
        }
    
        if let Some(entity) = entities.get_mut(&EntityKey{id:1,entity_type:EntityType::Enemy}) {
            entity.draw(render);
        }

        // println!("Drawing in play mode. State:");
    }

    fn update_over(&mut self, window: &Window, mode: &mut Mode, entities: &mut EntityHashMap) {
        if window.is_key_down(Key::Key1) {
            *mode = Mode::Start;   
        }
    }

    fn draw_over(&mut self, render: &mut Render, entities: &mut  EntityHashMap) {
        let red = (255u32 << 16) | (0u32 << 8) | 0u32;
        render.color(red);
    }
}

fn main() {
    let mut game = Game::new("My Game", 500, 500);
    game.start();
}
