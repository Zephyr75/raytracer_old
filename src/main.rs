extern crate sdl2; 

use sdl2::{pixels::Color, video::Window};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;

use std::time::{SystemTime, Duration};
use rayon::prelude::*;
use std::sync::Arc;

mod math {
    pub mod vector3;
    pub mod point3;
    pub mod ray;
    pub mod utilities;
}

use math::vector3::Vector3;
use math::point3::Point3;
use math::ray::Ray;
use math::utilities::{*, self};

mod collision {
    pub mod sphere;
    pub mod hittable;
}

use collision::sphere::Sphere;
use collision::hittable::Hit;
use collision::hittable::Hittable;



pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("Raytracer", 800, 400)
        .position_centered()
        .resizable()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut last_frame = SystemTime::now() - Duration::from_millis(1);

    // Viewport settings
    let viewport_height = 2.0;
    let focal_length = 1.0;

    let origin: Point3 = Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let vertical: Vector3 = Vector3 {
        x: 0.0,
        y: viewport_height,
        z: 0.0,
    };

    'running: loop {
        let width = canvas.window().size().0;
        let height = canvas.window().size().1;
        let aspect_ratio = width as f32 / height as f32;
        let viewport_width = aspect_ratio * viewport_height;
        let horizontal: Vector3 = Vector3 {
            x: viewport_width,
            y: 0.0,
            z: 0.0,
        };
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vector3{x: 0.0, y: 0.0, z: focal_length};

        println!("fps: {}", 1000 / SystemTime::now().duration_since(last_frame).unwrap().as_millis());
        last_frame = SystemTime::now();
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        ////////////////////////////////////////////////////////////////////////

        let mut pixels: Vec<(Point, Color)> = vec![(Point::new(0, 0), Color::BLACK); (height * width) as usize];

        let mut world: Vec<Arc<dyn Hittable + Sync + Send>> = vec![];
        world.push(Arc::new(Sphere {
            center: Point3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            radius: 0.5,
        }));
        world.push(Arc::new(Sphere {
            center: Point3 {
                x: 0.0,
                y: 1.0,
                z: -1.0,
            },
            radius: 0.5,
        }));

        
        pixels.par_iter_mut().enumerate().for_each(|(k, p)| {
            let i = k as u32 % width;
            let j = k as u32 / width;
            let u: f32 = i as f32 / (width as f32 - 1.0);
            let v: f32 = j as f32 / (height as f32 - 1.0);
            
            let ray: Ray = Ray { origin: origin, direction: lower_left_corner - origin + horizontal * u + vertical * v };
            
            let mut color = ray_color(&ray, &world);
            
            let point: Point = Point::new(i as i32, (height - j - 1) as i32);

            *p = (point, color);
        });

        pixels.iter().for_each(|p|{
            canvas.set_draw_color(p.1);
            canvas.draw_point(p.0);
        });

        ////////////////////////////////////////////////////////////////////////

        canvas.present();
    }
}

fn ray_color(ray: &Ray, world: &Vec<Box<dyn Hittable>>) -> Color {

    let mut hit: Hit = Hit{
        t: 0.0,
        point: Point3{
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        normal: Vector3{
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        front_face: true,
    };
    if (hit_anything(world, ray, 0.0, utilities::INF as f32, &mut hit)){
        let r = 127.0 * (1.0 - hit.normal.y);
        let g = 127.0 * (1.0 - hit.normal.y);
        let b = 127.0 * (1.0 - hit.normal.y);
        return Color::RGB(r as u8, g as u8, b as u8);
    }
    let unit_direction = ray.direction.unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    let r = (1.0 - t) * 255.0 + t * 255.0 * 0.5;
    let g = (1.0 - t) * 255.0 + t * 255.0 * 0.7;
    let b = (1.0 - t) * 255.0 + t * 255.0;
    Color::RGB(r as u8, g as u8, b as u8)
}

fn hit_anything (objects: &Vec<Box<dyn Hittable>>, r: &Ray, t_min: f32, t_max: f32, hit: &mut Hit) -> bool {
    let mut temp_hit: Hit = Hit{
        t: 0.0,
        point: Point3{x: 0.0, y: 0.0, z: 0.0},
        normal: Vector3{x: 0.0, y: 0.0, z: 0.0},
        front_face: false,
    };
    let mut hit_anything = false;
    let mut closest_so_far = t_max;
    
    objects.iter().for_each(|o|{
        if o.hit(r, t_min, closest_so_far, &mut temp_hit) {
            hit_anything = true;
            closest_so_far = temp_hit.t;
            *hit = temp_hit;
        }
    });

    hit_anything
}

