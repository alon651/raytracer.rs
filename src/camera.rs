use crate::canvas::Canvas;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::tuple::Tuple;
use crate::world::World;
use rayon::prelude::*;
use std::sync::Mutex;

pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub field_of_view: f32,
    transform: Matrix,
    pub pixel_size: f32,
    half_height: f32,
    half_width: f32,
    inverse:Matrix,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f32) -> Camera {
        let half_view = (field_of_view / 2.).tan();
        let aspect = (hsize as f32 / vsize as f32);
        let half_width: f32;
        let half_height: f32;
        if (aspect >= 1.) {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            // println!("width:{half_width:?}");
            // println!("aspect:{aspect:?}");
            half_height = half_view;
        }

        Camera {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix::identity_matrix(4),
            pixel_size: (half_width * 2.) / hsize as f32,
            half_width,
            half_height,
            inverse: Matrix::identity_matrix(4).inverse()
        }
    }

    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        let xoffset = (x as f32 + 0.5) * self.pixel_size;
        let yoffset = (y as f32 + 0.5) * self.pixel_size;

        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let pixel = &self.inverse * Tuple::new_point(world_x, world_y, -1.);
        let origin = &self.inverse * Tuple::new_point(0., 0., 0.);
        let direction = (pixel - origin).normalize();
        Ray::new(origin, direction)
    }

    pub fn render(self, w: World) -> Canvas {
        let y_range: Vec<usize> = (0..self.vsize - 1).collect();
        let x_range: Vec<usize> = (0..self.hsize - 1).collect();
        let canvas = Mutex::new(Canvas::new(self.hsize, self.vsize));

        y_range.par_iter().for_each(|&y| {
            x_range.par_iter().for_each(|&x| {
                let r = self.ray_for_pixel(x, y);
                let c = w.color_at(r);
                let mut canvas_lock = canvas.lock().unwrap();
                canvas_lock.set_pixel(x, y, c.get_rgb());
            })
        });

        // y_range.iter().for_each(|&y| {
        //     x_range.iter().for_each(|&x| {
        //         let r = self.ray_for_pixel(x, y);
        //         let c = w.color_at(r);
        //         let mut canvas_lock = canvas.lock().unwrap();
        //         canvas_lock.set_pixel(x, y, c.get_rgb());
        //     })
        // });


        canvas.into_inner().unwrap()
    }
    pub fn set_transform(&mut self, t:Matrix){
        self.inverse = t.inverse();
        self.transform = t;
    }
}
