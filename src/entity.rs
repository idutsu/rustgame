use std::collections::HashMap;
use image::DynamicImage;
use minifb::{Window, Key};
use crate::render::Render;
use crate::game::Game;
use rand::Rng;

type EntityId = u32;

#[derive(Hash, PartialEq, Eq)]
pub enum EntityType {
    Player,
    Enemy,
}

#[derive(Hash, PartialEq, Eq)]
pub struct EntityKey {
    pub id : EntityId,
    pub entity_type : EntityType,
}

pub enum Entity {
    Player(Player),
    Enemy(Enemy),
}


pub struct Player {
    pub image : DynamicImage,
    pub x     : f32,
    pub y     : f32,
}

impl Player {

    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }

    fn width(&self) -> f32 {
        self.image.width() as f32
    }

    fn height(&self) -> f32 {
        self.image.height() as f32
    }

    pub fn update(&mut self, game: &mut Game, window: &Window) {

        let game_width = game.width as f32;
        let game_height = game.height as f32;
        let player_width = self.image.width() as f32;
        let player_height = self.image.height() as f32;
        const PLAYER_SPEED: f32 = 5.0;

        if window.is_key_down(Key::Right) {
            if self.x + player_width < game_width {
                self.x += PLAYER_SPEED;
            } else {
                self.x = game_width - player_width;
            }
        }
        if window.is_key_down(Key::Left) {
            if self.x >= PLAYER_SPEED {
                self.x -= PLAYER_SPEED;
            } else {
                self.x = 0.0;
            }
        }
        if window.is_key_down(Key::Up) {
            if self.y >= PLAYER_SPEED {
                self.y -= PLAYER_SPEED;
            } else {
                self.y = 0.0;
            }
        }
        if window.is_key_down(Key::Down) {
            if self.y + player_height <= game_height - PLAYER_SPEED {
                self.y += PLAYER_SPEED
            } else {
                self.y = game_height - player_height;
            }
        }
    }
    pub fn draw(&mut self, render: &mut Render) {
        render.image_at(&self.image, self.x, self.y);
    }
}

pub struct Enemy {
    pub image : DynamicImage,
    pub x     : f32,
    pub y     : f32,
}

impl Enemy {

    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }

    fn width(&self) -> f32 {
        self.image.width() as f32
    }

    fn height(&self) -> f32 {
        self.image.height() as f32
    }

    pub fn update(&mut self, game: &mut Game, window: &Window) {
    }
    pub fn draw(&mut self, render: &mut Render) {
        render.image_at(&self.image, self.x, self.y);
    }
    // Enemy特有の関数
    pub fn move_randomly(&mut self, game: &mut Game, window: &Window) {
        let game_width = game.width as f32;
        let game_height = game.height as f32;
        let enemy_width = self.image.width() as f32;
        let enemy_height = self.image.height() as f32;
        const ENEMY_SPEED: f32 = 3.0;

        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(1..=4);

        match random_number {
            1 => {
                self.x += ENEMY_SPEED;
                if self.x + enemy_width < game_width {
                    self.x += ENEMY_SPEED;
                } else {
                    self.x = game_width - enemy_width;
                }
            }
            2 => {
                self.x -= ENEMY_SPEED;
                if self.x >= ENEMY_SPEED {
                    self.x -= ENEMY_SPEED;
                } else {
                    self.x = 0.0;
                }
            }
            3 => {
                self.y -= ENEMY_SPEED;
                if self.y >= ENEMY_SPEED {
                    self.y -= ENEMY_SPEED;
                } else {
                    self.y = 0.0;
                }
            }
            4 => {
                self.y += ENEMY_SPEED;
                if self.y + enemy_height <= game_height - 0.2 {
                    self.y += ENEMY_SPEED
                } else {
                    self.y = game_height - enemy_height;
                }
            }
            _ => {}
        } 
    }
}

impl Entity {

    pub fn update_all(entities: &mut HashMap<EntityKey, Entity>, game: &mut Game, window: &Window) {
        for (_, entity) in entities.iter_mut() {
            entity.update(game, window);
        }
    }

    pub fn update(&mut self, game: &mut Game, window: &Window) {
        match self {
            Entity::Player(player) => player.update(game, window),
            Entity::Enemy(enemy) => enemy.update(game, window),
        }

    }

    pub fn draw(&mut self, render: &mut Render) {
        match self {
            Entity::Player(player) => player.draw(render),
            Entity::Enemy(enemy) => enemy.draw(render),
        }

    }

    pub fn hit(&self, other: &Entity) -> bool{
        match (self, other) {
            (Entity::Player(player), Entity::Enemy(enemy)) => {
                player.x() < enemy.x() + enemy.width() &&
                player.x() + player.width() > enemy.x() &&
                player.y() < enemy.y() + enemy.height() &&
                player.y() + player.height() > enemy.y()
            }
            (Entity::Enemy(enemy), Entity::Player(player)) => {
                enemy.x() < player.x() + player.width() &&
                enemy.x() + enemy.width() > player.x() &&
                enemy.y() < player.y() + player.height() &&
                enemy.y() + enemy.height() > player.y()
            }
            _ => {
               false
            }
        }
    }
}
