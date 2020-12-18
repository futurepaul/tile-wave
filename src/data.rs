use std::{
    fs,
    ops::{Index, IndexMut},
    path::PathBuf,
};

use druid::{
    im::{vector, Vector},
    piet::ImageFormat,
    Env, EventCtx, ImageBuf, Selector,
};
use druid::{Color, Data, Lens};
use image::{imageops, DynamicImage, GrayImage, ImageBuffer, Pixel, RgbImage};
use rand::Rng;

pub const CANVAS_SIZE: usize = 8;
pub const MAP_SIZE: usize = 16;

pub const SAVE_CANVAS: Selector = Selector::new("tile-wave.save-canvas");
pub const CLEAR_CANVAS: Selector = Selector::new("tile-wave.clear-canvas");
pub const SHOW_MAP_WINDOW: Selector = Selector::new("tile-wave.show-map-window");

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub active_canvas_id: usize,
    pub modules: Vector<Canvas>,
    pub selected_color: Color,
    pub palette: Vector<Color>,
    pub map: Vector<Vector<Canvas>>,
}

impl AppState {
    pub fn new() -> Self {
        let mut state = Self {
            active_canvas_id: 0,
            modules: vector![],
            selected_color: Color::WHITE,
            palette: vector![Color::BLACK, Color::WHITE, Color::rgb8(10, 127, 127)],
            map: vector![],
        };

        state.load_modules_from_path("tile_images");

        state
    }

    pub fn fill_map(&mut self) {
        let mut rng = rand::thread_rng();
        let mut map = vector![];
        for _ in 0..MAP_SIZE {
            let mut row = vector![];
            for _ in 0..MAP_SIZE {
                let index: usize = rng.gen_range(0, self.modules.len());
                let mut module = self.modules[index].clone();
                let rotated: bool = rng.gen();
                let flipped_horizontal: bool = rng.gen();
                let flipped_vertical: bool = rng.gen();
                if rotated {
                    module.rotate_90();
                }
                if flipped_horizontal {
                    module.flip_horizontal();
                }
                if flipped_vertical {
                    module.flip_vertical();
                }
                row.push_back(module);
            }
            map.push_back(row);
        }
        self.map = map;
    }

    pub fn load_modules_from_path(&mut self, path: &str) {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let canvas = Canvas::new_from_image(&path, self.next_id());
            self.modules.push_back(canvas)
        }
        if self.modules.len() == 0 {
            self.modules.push_back(Canvas::new(self.next_id()));
        }
    }

    pub fn click_color(_ctx: &mut EventCtx, (data, color): &mut (Self, Color), _env: &Env) {
        data.selected_color = color.clone();
        data.get_active_module_mut().current_color = color.clone();
    }

    pub fn click_module(_ctx: &mut EventCtx, (data, module): &mut (Self, Canvas), _env: &Env) {
        data.active_canvas_id = module.id;
        data.get_active_module_mut().current_color = data.selected_color.clone();
    }

    pub fn next_id(&self) -> usize {
        if let Some(module) = self.modules.last() {
            module.id + 1
        } else {
            0
        }
    }

    pub fn click_add_module(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        let next_id = data.next_id();
        let mut canvas = Canvas::new(next_id);
        canvas.current_color = data.selected_color.clone();
        data.modules.push_back(canvas);
        data.active_canvas_id = next_id;
    }

    pub fn get_index_from_id(&self, id: usize) -> usize {
        for (index, module) in self.modules.iter().enumerate() {
            if module.id == id {
                return index;
            }
        }
        panic!("No module found with that ID");
    }

    pub fn get_active_module(&self) -> &Canvas {
        let index = self.get_index_from_id(self.active_canvas_id);
        &self.modules[index]
    }

    pub fn get_active_module_mut(&mut self) -> &mut Canvas {
        let index = self.get_index_from_id(self.active_canvas_id);
        &mut self.modules[index]
    }

    pub fn save_active_canvas_as_image(&self) {
        self.get_active_module().save_as_image();
    }

    pub fn clear_active_canvas(&mut self) {
        self.get_active_module_mut().clear()
    }
}

pub struct CanvasLens;

impl Lens<AppState, Canvas> for CanvasLens {
    fn with<V, F: FnOnce(&Canvas) -> V>(&self, data: &AppState, f: F) -> V {
        f(data.get_active_module())
    }

    fn with_mut<V, F: FnOnce(&mut Canvas) -> V>(&self, data: &mut AppState, f: F) -> V {
        f(data.get_active_module_mut())
    }
}

#[derive(Clone, Data, PartialEq)]
pub struct Canvas {
    pub id: usize,
    pub drawing: bool,
    pub current_color: Color,
    pub storage: Vector<Color>,
}

impl Canvas {
    pub fn new(id: usize) -> Self {
        let storage = (0..CANVAS_SIZE * CANVAS_SIZE)
            .into_iter()
            .map(|_| Color::BLACK)
            .collect();
        Self {
            id,
            drawing: false,
            current_color: Color::WHITE,
            storage,
        }
    }

    pub fn image_to_storage(img: RgbImage) -> Vector<Color> {
        let mut storage: [Color; CANVAS_SIZE * CANVAS_SIZE] =
            [Color::BLACK; CANVAS_SIZE * CANVAS_SIZE];

        for (x, y, pixel) in img.enumerate_pixels() {
            let pixel = pixel.channels();
            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];
            let color = Color::rgb8(r.clone(), g.clone(), b.clone());
            storage[(x as usize * CANVAS_SIZE) + y as usize] = color;
        }

        let storage = Vector::from(storage.as_ref());

        storage
    }

    pub fn new_from_image(path: &PathBuf, id: usize) -> Self {
        let img = image::open(path).unwrap().into_rgb8();

        let mut storage: [Color; CANVAS_SIZE * CANVAS_SIZE] =
            [Color::BLACK; CANVAS_SIZE * CANVAS_SIZE];

        for (x, y, pixel) in img.enumerate_pixels() {
            let pixel = pixel.channels();
            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];
            let color = Color::rgb8(r.clone(), g.clone(), b.clone());
            storage[(x as usize * CANVAS_SIZE) + y as usize] = color;
        }

        let storage = Vector::from(storage.as_ref());

        Self {
            id,
            drawing: false,
            current_color: Color::WHITE,
            storage,
        }
    }

    pub fn clear(&mut self) {
        self.storage = (0..CANVAS_SIZE * CANVAS_SIZE)
            .into_iter()
            .map(|_| Color::BLACK)
            .collect();
    }

    pub fn as_image(&self) -> RgbImage {
        let mut imgbuf: RgbImage = ImageBuffer::new(CANVAS_SIZE as u32, CANVAS_SIZE as u32);

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let pos = CanvasPos {
                row: x as usize,
                col: y as usize,
            };

            let (r, g, b, _) = self[pos].as_rgba8();

            *pixel = image::Rgb([r, g, b]);
        }

        imgbuf
    }

    pub fn save_as_image(&self) {
        let imgbuf = self.as_image();

        let path = format!("tile_images/test_{}.png", self.id);

        imgbuf.save(path).unwrap();
    }

    pub fn rotate_90(&mut self) {
        let mut img = self.as_image();
        let rotated = imageops::rotate90(&mut img);
        self.storage = Self::image_to_storage(rotated);
    }

    pub fn flip_horizontal(&mut self) {
        let mut img = self.as_image();
        let flipped = imageops::flip_horizontal(&mut img);
        self.storage = Self::image_to_storage(flipped);
    }

    pub fn flip_vertical(&mut self) {
        let mut img = self.as_image();
        let flipped = imageops::flip_vertical(&mut img);
        self.storage = Self::image_to_storage(flipped);
    }
}

#[derive(Clone, Copy, Data)]
pub struct CanvasPos {
    pub row: usize,
    pub col: usize,
}

impl Index<CanvasPos> for Canvas {
    type Output = Color;
    fn index(&self, pos: CanvasPos) -> &Self::Output {
        let idx = pos.row * CANVAS_SIZE + pos.col;
        self.storage.index(idx)
    }
}

impl IndexMut<CanvasPos> for Canvas {
    fn index_mut(&mut self, pos: CanvasPos) -> &mut Self::Output {
        let idx = pos.row * CANVAS_SIZE + pos.col;
        self.storage.index_mut(idx)
    }
}
