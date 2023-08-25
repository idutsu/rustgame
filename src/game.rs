use crate::mode::Mode;
use crate::render::Render;
use image::DynamicImage;
use minifb::{Key, Window, WindowOptions};
use std::collections::HashMap;
use std::thread;
use std::time::{Duration, Instant};
use crate::entity::{ Entity, EntityKey };


pub struct Game {
    name: String,
    width: usize,
    height: usize,
    images: HashMap<String, DynamicImage>,
}

impl Game {
    pub fn new(name: &str, width: usize, height: usize) -> Self {
        Self {
            name: name.to_string(),
            width: width,
            height: height,
            images: HashMap::new(),
        }
    }

    pub fn load_images(&mut self) {
        for img in self.set_assets() {
            if let Some(dynamic_image) = image::open(&img).ok() {
                self.images.insert(img.to_string(), dynamic_image);
            }
        }
    }

    pub fn get_image(&self, filename: &str) -> Option<DynamicImage> {
        self.images.get(filename).map(|img|img.clone())
    }

    pub fn start(&mut self) {
        const FRAME_RATE: u64 = 1_000_000 / 60;

        let width = self.width;
        let height = self.height;

        let mut mode = Mode::Start;

        let mut window = Window::new(&self.name, width, height, WindowOptions::default())
            .unwrap_or_else(|e| {
                panic!("{}", e);
            });
        
        let mut render = Render {
            buffer: vec![0; width * height],
            width: width,
            height: height,
        };

        self.load_images();

        let mut entities = self.set_entities();

        while window.is_open() && !window.is_key_down(Key::Escape) {
            let loop_start = Instant::now();

            for pixel in render.buffer.iter_mut() {
                *pixel = 0;
            }

            match mode {
                Mode::Start => {
                    self.update_start(&window, &mut mode, &mut entities);
                    self.draw_start(&mut render, &mut entities);
                },
                Mode::Play => {
                    self.update_play(&window, &mut mode, &mut entities);
                    self.draw_play(&mut render, &mut entities);
                },
                Mode::Over => {
                    self.update_over(&window, &mut mode, &mut entities);
                    self.draw_over(&mut render, &mut entities);
                },
            }

            window.update_with_buffer(&render.buffer, width, height).unwrap();

            let loop_end = Instant::now();
            let loop_duration = loop_end - loop_start;
            let frame_time = Duration::from_micros(FRAME_RATE);
            if loop_duration < frame_time {
                thread::sleep(frame_time - loop_duration);
            }
        }
    }
}

pub trait GameBase {
    fn set_assets(&self) ->  Vec<String>;
    fn set_entities(&self) -> HashMap<EntityKey, Entity>;
    fn update_start(&mut self, window: &Window, mode: &mut Mode, entities: &mut  HashMap<EntityKey, Entity>);
    fn update_play(&mut self, window: &Window, mode: &mut Mode, entities: &mut  HashMap<EntityKey, Entity>);
    fn update_over(&mut self, window: &Window, mode: &mut Mode, entities: &mut  HashMap<EntityKey, Entity>);
    fn draw_start(&mut self, render: &mut Render, entities: &mut  HashMap<EntityKey, Entity>);
    fn draw_play(&mut self, render: &mut Render, entities: &mut  HashMap<EntityKey, Entity>);
    fn draw_over(&mut self, render: &mut Render, entities: &mut  HashMap<EntityKey, Entity>);
}
