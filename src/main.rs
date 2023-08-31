use dashu::{float::{DBig, FBig}, rational::Relaxed};
use std::{ops::Mul, fs::File};

use image::{Rgb32FImage, Rgb, codecs::png::PngEncoder, ImageEncoder, RgbImage};

#[inline]
fn rgb(r: f64, g: f64, b: f64) -> Rgb<u8> {
    Rgb([(r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8])
}

#[inline]
fn big(value: f64) -> f64 {
    // Relaxed::try_from(value).unwrap()
    value
}

#[derive(Debug, Clone)]
pub struct BigComplex {
    a: f64,
    b: f64,
}

pub trait Square {
    fn square(&self) -> f64;
}
impl Square for f64 {
    fn square(&self) -> f64 {
        self * self
    }
}

impl BigComplex {
    pub fn new(a: f64, b: f64) -> Self {
        Self {
            a,
            b,
        }
    }
}

pub fn lerp(value: f64, out_min: f64, out_max :f64) -> f64 {
    (out_max - out_min) * value + out_min
}
pub fn lerp_f64(value: f64, out_min: f64, out_max :f64) -> f64 {
    (out_max - out_min) * value + out_min
}

fn main() {
    let pics = 30 * 10;
    for level in 0..pics {
        let width = 1280;
        let height = 720;
        let aspect_ratio_f64 = height as f64 / width as f64;
        let aspect_ratio = big(aspect_ratio_f64);
        let mut img = RgbImage::new(width, height);
        let level_factor = lerp_f64(level as f64 / pics as f64, 0.5, 16.0);
        let zoom = big(2.0f64.powf(level_factor));
        let px = big(-1.5);
        let py = big(0.0);
        let yfov_bound_f64 = 2.0;
        let yfov = big(yfov_bound_f64) * big(0.5) / zoom.clone();
        let xfov = yfov.clone() / aspect_ratio.clone();
        for i in 0..width {
            for j in 0..height {
                let u = big(i as f64 / width as f64);
                let v = big(j as f64 / height as f64);
                let x = px.clone() + lerp(u.clone(), -xfov.clone(), xfov.clone());
                let y = py.clone() + lerp(v.clone(), -yfov.clone(), yfov.clone());

                let mut z = BigComplex::new(x.clone(), y.clone());
                let c = z.clone();
                let mut len_sq = (z.a + z.b).abs();
                let max = 32.0;
                let iters = 100;
                let mut niters = 0;
                for _ in 0..iters {
                    let aa = z.a.square() - z.b.square();
                    let bb = big(2.0) * z.a.clone() * z.b.clone();
                    z.a = aa + c.a;
                    z.b = bb + c.b;
                    len_sq = (z.a + z.b).abs();
                    if len_sq > max {
                        break;
                    }
                    niters += 1;
                }

                if len_sq > max {
                    let hard_pixel = niters as f64 / iters as f64;
                    img.put_pixel(i, j, rgb(1.0, hard_pixel, 1.0));
                }
            }
        }
        let file = format!("out/test_{:05}.png", level);
        match img.save(file) {
            Err(e) => println!("{:?}", e),
            _ => {}
        };
        println!("Iter: {:05} / {:05}", level, pics);
    }

    println!("Hello, world!");
}
