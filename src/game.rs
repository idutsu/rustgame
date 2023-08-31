use image::DynamicImage;
use minifb::{Key, Window, WindowOptions};
use std::collections::HashMap;
use std::thread;
use std::time::{Duration, Instant};
use crate::mode::Mode;
use crate::render::Render;
use crate::entity::{ Entity, EntityKey };

pub type ImageFilePaths = Vec<String>;
pub type EntityHashMap = HashMap<EntityKey, Entity>;


pub struct Game {
    pub name   : String,
    pub width  : usize,
    pub height : usize,
    pub score  : usize,
    pub level  : usize,
    images     : HashMap<String, DynamicImage>,
    delta_time : Duration,
}

impl Game {
    pub fn new(name: &str, width: usize, height: usize) -> Self {
        Self {
            name       : name.to_string(),
            width      : width,
            height     : height,
            score      : 0,
            level      : 0,
            images     : HashMap::new(),
            delta_time : Duration::new(0,0),
        }
    }

    fn load_images(&mut self) {
        for filename in self.add_images() {
            let images_dir = String::from("./images/");
            let path = images_dir + &filename;
            if let Some(dynamic_image) = image::open(&path).ok() {
                self.images.insert(filename, dynamic_image);
            }
        }
    }

    pub fn get_image(&self, path: &str) -> Option<DynamicImage> {
        self.images.get(path).map(|img|img.clone())
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

        let mut entities = self.add_entities();

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
            self.delta_time = loop_duration;
            if loop_duration < frame_time {
                thread::sleep(frame_time - loop_duration);
            }
        }
    }
}

pub trait GameBase {
    fn add_images(&self) -> ImageFilePaths;
    fn add_entities(&self) -> EntityHashMap;
    fn update_start(&mut self, window: &Window, mode: &mut Mode, entities: &mut EntityHashMap);
    fn draw_start(&mut self, render: &mut Render, entities: &mut EntityHashMap);
    fn update_play(&mut self, window: &Window, mode: &mut Mode, entities: &mut EntityHashMap);
    fn draw_play(&mut self, render: &mut Render, entities: &mut EntityHashMap);
    fn update_over(&mut self, window: &Window, mode: &mut Mode, entities: &mut EntityHashMap);
    fn draw_over(&mut self, render: &mut Render, entities: &mut EntityHashMap);
}
