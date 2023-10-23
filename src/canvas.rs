use crate::color::Color;
use image::{ImageBuffer, Rgb};
use std::cmp::min;
use std::fs::File;
use std::io::Write;
#[derive(Debug)]
pub struct Canvas {
    pub height: usize,
    pub width: usize,
    pixels: Vec<Color>,
}
impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            height,
            width,
            pixels: vec![(0.0, 0.0, 0.0).into(); height * width],
        }
    }
    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        let x = min(x, self.width - 1);
        let y = min(y, self.height - 1);
        self.pixels[y * self.width + x]
    }
    pub fn set_pixel(&mut self, x: usize, y: usize, rgb: (f32, f32, f32)) {
        let x = min(x, self.width - 1);
        let y = min(y, self.height - 1);
        self.pixels[y * self.width + x] = rgb.into();
    }

    pub fn save(&self, file_name: &str) {
        let mut f = File::create(file_name).unwrap_or_else(|_| {
            panic!(
                "{}",
                ("Error: could not create file ".to_owned() + file_name)
            )
        });
        write!(&mut f, "P3\n{} {}\n255\n", self.width, self.height).unwrap_or_else(|_| {
            panic!(
                "{}",
                ("Error: could not write to file ".to_owned() + file_name)
            )
        });
        let mut i = 0;
        println!("saving as {}", file_name);
        // let bar = indicatif::ProgressBar::new(self.pixels.len().try_into().unwrap());
        self.pixels.iter().for_each(|_| {
            let pixel = self.pixels[i];
            writeln!(
                &mut f,
                "{} {} {}",
                (pixel.get_rgb().0 * 255.).clamp(0.0, 255.0).round() as i32,
                (pixel.get_rgb().1 * 255.).clamp(0.0, 255.0).round() as i32,
                (pixel.get_rgb().2 * 255.).clamp(0.0, 255.0).round() as i32,
            )
            .expect("failed writing to file. error occured");
            i += 1;
            //bar.inc(1);
        });
    }
    pub fn save2png(&self, file_name: &str) {
        let mut image = ImageBuffer::new(self.width as u32, self.height as u32);
        println!("saving as {}", file_name);
        for x in 0..self.width {
            for y in 0..self.height {
                let pix = &self.pixels[y * self.width + x].get_rgb();
                image.put_pixel(
                    x as u32,
                    y as u32,
                    Rgb([
                        (pix.0 * 255.0).clamp(0.0, 255.0) as u8,
                        (pix.1 * 255.0).clamp(0.0, 255.0) as u8,
                        (pix.2 * 255.0).clamp(0.0, 255.0) as u8,
                    ]),
                )
            }
        }
        image
            .save(file_name.to_owned() + ".png")
            .expect("couldn't save image");
    }
}
