use image::{ImageBuffer, ImageResult, Rgba};
use imageproc::drawing;
use rand::{rngs::ThreadRng, Rng};
use rusttype::{Font, Scale};
use std::{fmt::Write, io::Cursor, path::Path};

use crate::{color, Captcha};

///the builder of captcha
pub struct CaptchaBuilder<'a, 'b> {
    ///captcha image width
    pub width: u32,
    ///captcha image height
    pub height: u32,

    ///random string length.
    pub length: u32,

    ///source is a unicode which is the rand string from.
    pub source: String,

    ///image background color (optional)
    pub background_color: Option<Rgba<u8>>,
    ///fonts collection for text
    pub fonts: &'b [Font<'a>],
    ///The maximum number of lines to draw behind of the image
    pub max_behind_lines: Option<u32>,
    ///The maximum number of lines to draw in front of the image
    pub max_front_lines: Option<u32>,
    ///The maximum number of ellipse lines to draw in front of the image
    pub max_ellipse_lines: Option<u32>,
}

impl<'a, 'b> Default for CaptchaBuilder<'a, 'b> {
    fn default() -> Self {
        Self {
            width: 150,
            height: 40,
            length: 5,
            source: String::from("1234567890qwertyuioplkjhgfdsazxcvbnm"),
            background_color: None,
            fonts: &[],
            max_behind_lines: None,
            max_front_lines: None,
            max_ellipse_lines: None,
        }
    }
}

impl<'a, 'b> CaptchaBuilder<'a, 'b> {
    fn gen_random_text(&self, rng: &mut ThreadRng) -> String {
        let mut source_chars = vec![];
        for c in self.source.as_str().chars() {
            source_chars.push(c);
        }
        let source_chars_count = source_chars.len();
        let mut text = String::with_capacity(self.length as usize);
        for _i in 0..self.length {
            let r = rng.gen_range(0..source_chars_count);
            write!(text, "{}", source_chars[r]).unwrap();
        }
        text
    }
    fn write_phrase(
        &self,
        image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        rng: &mut ThreadRng,
        phrase: &str,
    ) {
        //println!("phrase={}", phrase);
        //println!("width={}, height={}", self.width, self.height);
        let font_size = (self.width as f32) / (self.length as f32) - rng.gen_range(1.0..=4.0);
        let scale = Scale::uniform(font_size);
        if self.fonts.is_empty() {
            panic!("no fonts loaded");
        }
        let font_index = rng.gen_range(0..self.fonts.len());
        let font = &self.fonts[font_index];
        let glyphs: Vec<_> = font
            .layout(phrase, scale, rusttype::point(0.0, 0.0))
            .collect();
        let text_height = {
            let v_metrics = font.v_metrics(scale);
            (v_metrics.ascent - v_metrics.descent).ceil() as u32
        };
        let text_width = {
            let min_x = glyphs.first().unwrap().pixel_bounding_box().unwrap().min.x;
            let max_x = glyphs.last().unwrap().pixel_bounding_box().unwrap().max.x;
            let last_x_pos = glyphs.last().unwrap().position().x as i32;
            (max_x + last_x_pos - min_x) as u32
        };
        let node_width = text_width / self.length;
        //println!("text_width={}, text_height={}", text_width, text_height);
        let mut x = ((self.width as i32) - (text_width as i32)) / 2;
        let y = ((self.height as i32) - (text_height as i32)) / 2;
        //
        for s in phrase.chars() {
            let text_color = color::gen_text_color(rng);
            let offset = rng.gen_range(-5..=5);
            //println!("x={}, y={}", x, y);
            drawing::draw_text_mut(
                image,
                text_color,
                x,
                y + offset,
                scale,
                font,
                &s.to_string(),
            );
            x += node_width as i32;
        }
    }

    fn draw_line(&self, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, rng: &mut ThreadRng) {
        let line_color = color::gen_line_color(rng);
        let is_h = rng.gen();
        let (start, end) = if is_h {
            let xa = rng.gen_range(0.0..(self.width as f32) / 2.0);
            let ya = rng.gen_range(0.0..(self.height as f32));
            let xb = rng.gen_range((self.width as f32) / 2.0..(self.width as f32));
            let yb = rng.gen_range(0.0..(self.height as f32));
            ((xa, ya), (xb, yb))
        } else {
            let xa = rng.gen_range(0.0..(self.width as f32));
            let ya = rng.gen_range(0.0..(self.height as f32) / 2.0);
            let xb = rng.gen_range(0.0..(self.width as f32));
            let yb = rng.gen_range((self.height as f32) / 2.0..(self.height as f32));
            ((xa, ya), (xb, yb))
        };
        let thickness = rng.gen_range(2..4);
        for i in 0..thickness {
            let offset = i as f32;
            if is_h {
                drawing::draw_line_segment_mut(
                    image,
                    (start.0, start.1 + offset),
                    (end.0, end.1 + offset),
                    line_color,
                );
            } else {
                drawing::draw_line_segment_mut(
                    image,
                    (start.0 + offset, start.1),
                    (end.0 + offset, end.1),
                    line_color,
                );
            }
        }
    }

    fn draw_ellipse(&self, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, rng: &mut ThreadRng) {
        let line_color = color::gen_line_color(rng);
        let thickness = rng.gen_range(2..4);
        for i in 0..thickness {
            let center = (
                rng.gen_range(-(self.width as i32) / 4..(self.width as i32) * 5 / 4),
                rng.gen_range(-(self.height as i32) / 4..(self.height as i32) * 5 / 4),
            );
            drawing::draw_hollow_ellipse_mut(
                image,
                (center.0, center.1 + i),
                (self.width * 6 / 7) as i32,
                (self.height * 5 / 8) as i32,
                line_color,
            );
        }
    }

    fn build_image(&self) -> (ImageBuffer<Rgba<u8>, Vec<u8>>, String) {
        let mut rng = rand::thread_rng();
        let bgc = match self.background_color {
            Some(v) => v,
            None => color::gen_background_color(&mut rng),
        };
        let mut image = ImageBuffer::from_fn(self.width, self.height, |_, _| bgc);
        //draw behind line
        let square = self.width * self.height;
        let effects = match self.max_behind_lines {
            Some(s) => {
                if s > 0 {
                    rng.gen_range(square / 3000..square / 2000).min(s)
                } else {
                    0
                }
            }
            None => rng.gen_range(square / 3000..square / 2000),
        };
        for _ in 0..effects {
            self.draw_line(&mut image, &mut rng);
        }
        //write phrase
        let phrase = self.gen_random_text(&mut rng);
        self.write_phrase(&mut image, &mut rng, &phrase);
        //draw front line
        let effects = match self.max_front_lines {
            Some(s) => {
                if s > 0 {
                    rng.gen_range(square / 3000..=square / 2000).min(s)
                } else {
                    0
                }
            }
            None => rng.gen_range(square / 3000..=square / 2000),
        };
        for _ in 0..effects {
            self.draw_line(&mut image, &mut rng);
        }
        //draw ellipse
        let effects = match self.max_front_lines {
            Some(s) => {
                if s > 0 {
                    rng.gen_range(square / 4000..=square / 3000).min(s)
                } else {
                    0
                }
            }
            None => rng.gen_range(square / 4000..=square / 3000),
        };
        for _ in 0..effects {
            self.draw_ellipse(&mut image, &mut rng);
        }
        (image, phrase)
    }

    ///build a captcha in png format
    pub fn build(&self) -> ImageResult<Captcha> {
        let (image, phrase) = self.build_image();
        let format = image::ImageOutputFormat::Png;
        let mut bytes: Vec<u8> = Vec::new();
        image.write_to(&mut Cursor::new(&mut bytes), format)?;
        Ok(Captcha::new(bytes, phrase))
    }

    ///build captcha and save in png format
    pub fn save<P: AsRef<Path>>(&self, path: P) -> ImageResult<String> {
        let (image, phrase) = self.build_image();
        image.save(path)?;
        Ok(phrase)
    }
}
