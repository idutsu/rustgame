use std::collections::HashMap;
use std::thread;
use minifb::{Window, WindowOptions, Key};
use std::time::{Duration, Instant};
use image::DynamicImage;

use crate::render::Render;
use crate::mode::Mode;

pub trait EntityBase {
}

pub trait GameBase {
    fn update_start(&mut self, window: &Window, mode: &mut Mode);
    fn update_play(&mut self, window: &Window, mode: &mut Mode);
    fn update_over(&mut self, window: &Window, mode: &mut Mode);
    fn draw_start(&mut self, render: &mut Render);
    fn draw_play(&mut self, render: &mut Render);
    fn draw_over(&mut self, render: &mut Render);
}

pub struct Game<T:EntityBase> {
    name     : String,
    width    : usize,
    height   : usize,
    assets   : Vec<String>,
    images   : HashMap<String, DynamicImage>,
    pub entities : HashMap<String, T>,
}

impl<T:EntityBase> Game<T> {
    pub fn new(name: &str, width: usize, height: usize) -> Self{
        Self {
            name   : name.to_string(),
            width  : width,
            height : height,
            assets : Vec::new(), 
            images : HashMap::new(),
            entities: HashMap::new(),
        }
    }

    pub fn add_image(&mut self, path: &str) {
        self.assets.push(path.to_string());
    }
    
    pub fn load_images(&mut self) {
        for img in &self.assets {
            match image::open(img) {
                Ok(dynamic_image) => {
                    self.images.insert(img.to_string(), dynamic_image);
                }
                Err(e) => {
                    eprintln!("Failed to load asset {}: {}", img, e);
                }
            }
        }
    }

    pub fn get_image(&self, filename: &str) -> DynamicImage{
        self.images.get(filename).unwrap().clone()
    }

    pub fn add_entity(&mut self, name: &str, entity: T) {
        self.entities.insert(name.to_string(), entity);
    }

    pub fn get_entity(&self, name: &str) -> Option<&T> {
        self.entities.get(name)
    }

    pub fn get_entity_mut(&mut self, name: &str) -> Option<&mut T> {
        self.entities.get_mut(name)
    }

    fn update(&mut self, window: &Window, mode: &mut Mode) {
        match mode {
            Mode::Start => self.update_start(window, mode),
            Mode::Play => self.update_play(window, mode),
            Mode::Over => self.update_over(window, mode),
        }
    }

    fn draw(&mut self, render: &mut Render, mode: &Mode) {
        println!("Drawing game...");
        match mode {
            Mode::Start => self.draw_start(render),
            Mode::Play => self.draw_play(render),
            Mode::Over => self.draw_over(render),
        }
    }

    pub fn start(&mut self) {

        const FRAME_RATE: u64 = 1_000_000 / 60;

        let width = self.width;
        let height = self.height;

        let mut mode = Mode::Start;

        let mut window = Window::new(
            &self.name,
            width,
            height,
            WindowOptions::default(),
        ).unwrap_or_else(|e| {
            panic!("{}", e);
        });

        let mut render =  Render {
            buffer: vec![0; width * height],
            width: width,
            height: height,
        };

        while window.is_open() && !window.is_key_down(Key::Escape) {
            
            let loop_start = Instant::now();
            
            for pixel in render.buffer.iter_mut() {
                *pixel = 0;
            }

            self.update(&window, &mut mode);
            self.draw(&mut render, &mode);
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
