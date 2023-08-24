use image::DynamicImage;
use minifb::{Window, Key};
use crate::render::Render;


pub enum Entity {
    Player(Player),
    Enemy(Enemy),
}

pub struct Player {
    pub image: DynamicImage,
    pub x: f32,
    pub y: f32,
}

impl Player {
    pub fn update(&mut self, window: &Window) {
        if window.is_key_down(Key::Right) {
            self.x += 2.0;
        }
        if window.is_key_down(Key::Left) {
            self.x -= 2.0;
        }
    }
    pub fn draw(&mut self, render: &mut Render) {
        render.image_at(&self.image, self.x, self.y);
    }
}


pub struct Enemy {
    pub image: DynamicImage,
    pub x: f32,
    pub y: f32,
}

impl Enemy {
    pub fn update(&mut self, window: &Window) {
        self.x += 0.50;
    }
    pub fn draw(&mut self, render: &mut Render) {
        render.image_at(&self.image, self.x, self.y);
    }
    // Enemy特有の関数
    pub fn move_randomly(&mut self) {
        // ここでランダムな動作を実装する
        self.x += 0.50;
    }
}


impl Entity {
    pub fn move_enemy(&mut self, window: &Window) {
        if let Entity::Enemy(enemy) = self {
            enemy.move_randomly();
        }
    }

    pub fn update(&mut self, window: &Window) {
        match self {
            Entity::Player(player) => player.update(window),
            Entity::Enemy(enemy) => enemy.update(window),
        }

    }

    pub fn draw(&mut self, render: &mut Render) {
        match self {
            Entity::Player(player) => player.draw(render),
            Entity::Enemy(enemy) => enemy.draw(render),
        }

    }
}
