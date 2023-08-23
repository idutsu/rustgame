use image::DynamicImage;
use minifb::{Window, Key};
mod game;
use game::{Game, GameBase};
use crate::game::EntityBase;
mod render;
use render::Render;
mod mode;
use mode::Mode;


impl EntityBase for Entity {}

enum Entity {
    Player(Player),
    Enemy(Enemy),
}

impl Entity {
    fn move_enemy(&mut self) {
        if let Entity::Enemy(enemy) = self {
            enemy.move_randomly();
        }
    }
}

struct Player {
    image: DynamicImage,
    x: f32,
    y: f32,
}

impl Player {
    fn update(&mut self, window: &Window) {
        if window.is_key_down(Key::Right) {
            self.x += 2.0;
        }
        if window.is_key_down(Key::Left) {
            self.x -= 2.0;
        }
    }
    fn draw(&mut self, render: &mut Render) {
        render.image_at(&self.image, self.x, self.y);
    }
}


struct Enemy {
    image: DynamicImage,
    x: f32,
    y: f32,
}

impl Enemy {
    fn update(&mut self, window: &Window) {
        self.x += 0.50;
    }
    fn draw(&mut self, render: &mut Render) {
        render.image_at(&self.image, self.x, self.y);
    }
    // Enemy特有の関数
    fn move_randomly(&mut self) {
        // ここでランダムな動作を実装する
        self.x += 0.50;
    }
}



impl<T: EntityBase> GameBase for Game<T> {
    fn update_start(&mut self, window: &Window, mode: &mut Mode) {
        *mode = Mode::Play;   
        println!("Updating in start mode. State:");
    }
    fn update_play(&mut self, window: &Window, mode: &mut Mode) {
        
        // if let Some(entity) = self.entities.get_mut("player") {
        //     entity.update(window);
        // }

        // if let Some(entity) = self.entities.get_mut("enemy") {
        //     entity.update(window);
        // }
       
        println!("Updating in play mode. State:");
    }
    fn update_over(&mut self, window: &Window, mode: &mut Mode) {
        println!("Updating in over mode. State:");
    }
    fn draw_start(&mut self, render: &mut Render) {
        println!("Drawing in start mode. State:");
    }
    fn draw_play(&mut self, render: &mut Render) {

        // if let Some(entity) = self.entities.get_mut("player") {
        //     entity.draw(render);
        // }
    
        // if let Some(entity) = self.entities.get_mut("enemy") {
        //     entity.draw(render);
        // }

        println!("Drawing in play mode. State:");
    }
    fn draw_over(&mut self, render: &mut Render) {
        let red = (255u32 << 16) | (0u32 << 8) | 0u32;
        render.color(red);
        println!("Drawing in over mode. State:");
    }
}

fn main() {
    let mut game = Game::new("My Game", 500, 500);
    game.add_image("player.png");
    game.add_image("enemy.png");
    game.load_images();
    game.add_entity(
        "player",
        Entity::Player(
            Player {
                image: game.get_image("player.png"),
                x: 0.0,
                y: 0.0,
            }
        )
    );
    game.add_entity(
        "enemy",
        Entity::Enemy(
            Enemy {
                image: game.get_image("enemy.png"),
                x: 0.0,
                y: 0.0,
            }
        )
    );
    game.start();
}
