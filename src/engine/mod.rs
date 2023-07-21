extern crate minifb;
use minifb::*;
use std::time::{Duration, Instant};

pub struct VRAM {
    size_x: usize,
    size_y: usize,
    buffer: Vec<u32>,
}
impl VRAM {
    pub fn new(size_x: usize, size_y: usize) -> Self {let buffer = vec![0; size_x * size_y]; Self {size_x, size_y, buffer}}
    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {if x < self.size_x && y < self.size_y {self.buffer[y * self.size_x + x] = color;}}
    pub fn get_pixel(&self, x: usize, y: usize) -> Option<u32> {if x < self.size_x && y < self.size_y {Some(self.buffer[y * self.size_x + x])} else {None}}
    pub fn fill(&mut self, color: u32) {for pixel in self.buffer.iter_mut() {*pixel = color;}}
}

pub struct DvigWindow {
    pub X: i32,
    pub Y: i32,
    pub Width: i32,
    pub Height: i32,
    pub Title: String,
    pub Fullscreen: bool,
    pub Resizable: bool,
    pub Borderless: bool,
    pub CloseRequested: bool,
    pub FPS: i32,
    pub window: Window,
    pub vram: VRAM,
}
impl DvigWindow {
    pub fn new(
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        title: String,
        fullscreen: bool,
        resizable: bool,
        borderless: bool,
        fps: i32,
    ) -> DvigWindow {
        let window_options = WindowOptions {
            borderless,
            title: true,
            resize: resizable,
            scale: Scale::X1,
        };
        let window = Window::new(&title, width as usize, height as usize, window_options).unwrap();
        return DvigWindow {
            X: x,
            Y: y,
            Width: width,
            Height: height,
            Title: title,
            Fullscreen: fullscreen,
            Resizable: resizable,
            Borderless: borderless,
            CloseRequested: false,
            FPS: fps,
            window,
            vram: VRAM::new(width as usize, height as usize),
        };
    }

    pub fn Run(&mut self) {
        let frame_time = Duration::from_micros(1_000_000 / self.FPS as u64);
        let mut last_frame_time = Instant::now();

        while self.window.is_open() {
            let current_time = Instant::now();
            let elapsed_time = current_time - last_frame_time;
            if elapsed_time >= frame_time {
                last_frame_time = current_time;
                if self.CloseRequested {break;}

                // UPDATE

                self.draw_vram_to_window();
                self.debug(frame_time, last_frame_time.elapsed());
                let excess_time = elapsed_time - frame_time;
                if excess_time < frame_time {
                    std::thread::sleep(frame_time - excess_time);
                }
            }
        }
    }

    fn draw_vram_to_window(&mut self) {self.window.update_with_buffer(&self.vram.buffer);}
    fn debug(&mut self, frame_time: Duration, elapsed_time: Duration) {
        self.window.set_title(format!(
            "{}; {} FPS",
            self.Title,
            1_000_000 / elapsed_time.as_micros(),
        ).as_str());
        let mut fre = 0;
        if frame_time.as_micros() > elapsed_time.as_micros() {
            fre = frame_time.as_micros() - elapsed_time.as_micros();
        }
        println!("MAX: {:>5}us; REL: {:>5}us; FRE: {:>5}us", frame_time.as_micros(), elapsed_time.as_micros(), fre);
    }
}
