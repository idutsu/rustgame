use image::DynamicImage;
use minifb::Window;
use crate::render::Render;


pub enum Entity {
    Player(Player),
    Enemy(Enemy),
}

trait EntityBase {
    fn update(&mut self, window: &Window) {}
    fn draw(&mut self, render: &mut Render) {}
}

struct Player {
    name: String,
    image: DynamicImage,
    x: f32,
    y: f32,
}

impl EntityBase for Player {
    fn update(&mut self, window: &Window) {
    }
    fn draw(&mut self, render: &mut Render) {
    }
}


struct Enemy {
    name: String,
    image: DynamicImage,
    x: f32,
    y: f32,
}

impl EntityBase for Enemy {
    fn update(&mut self, window: &Window) {
    }
    fn draw(&mut self, render: &mut Render) {
    }
}

impl Entity {
    fn update(&mut self, window: &Window) {
        match self {
            Entity::Player(player) => player.update(window),
            Entity::Enemy(enemy) => enemy.update(window),
            _ => {},
        }
    }
    fn draw(&mut self, render: &mut Render) {
        match self {
            Entity::Player(player) => player.draw(render),
            Entity::Enemy(enemy) => enemy.draw(render),
            _ => {},
        }
    }

}
