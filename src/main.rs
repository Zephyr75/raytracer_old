extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use std::time::Duration;

mod vector3;
use vector3::Vector3;
mod point3;
use point3::Point3;
mod ray;
use ray::Ray;

const WIDTH: u32 = 800;
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const HEIGHT: u32 = 450;
 
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("Raytracer", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
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

        // Viewport settings
        let viewport_height = 2.0;
        let viewport_width = ASPECT_RATIO as f32 * viewport_height;
        let focal_length = 1.0;

        let origin: Point3 = Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        let horizontal: Vector3 = Vector3 {
            x: viewport_width,
            y: 0.0,
            z: 0.0,
        };

        let vertical: Vector3 = Vector3 {
            x: 0.0,
            y: viewport_height,
            z: 0.0,
        };

        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - vector3 :: Vector3{x: 0.0, y: 0.0, z: focal_length};

        for j in 0..HEIGHT {
            for i in 0..WIDTH {
    
                let u: f32 = i as f32 / (WIDTH as f32 - 1.0);
                let v: f32 = j as f32 / (HEIGHT as f32 - 1.0);
                
                let r: Ray = Ray { origin: origin, direction: lower_left_corner - origin + horizontal * u + vertical * v };
    
                let mut color = ray_color(r);
                
                let p: Point = Point::new(i as i32, (HEIGHT - j - 1) as i32);
                canvas.set_draw_color(color);
                canvas.draw_point(p);
    
            }
        }


        

        ////////////////////////////////////////////////////////////////////////

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn ray_color(r: Ray) -> Color {
    let center: Point3 = Point3 {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };
    let mut t = hit_sphere(center, 0.5, r);
    if (t > 0.0){
        let N = (r.at(t) - center).unit();
        let r = 255.0 * 0.5 * (N.x + 1.0);
        let g = 255.0 * 0.5 * (N.y + 1.0);
        let b = 255.0 * 0.5 * (N.z + 1.0);
        return Color::RGB(r as u8, g as u8, b as u8);
    }

    let unit_direction: Vector3 = r.direction.unit();
    t = 0.5*(unit_direction.y + 1.0);
    let r = (1.0 - t) * 255.0 + t * 127.0;
    let g = (1.0 - t) * 255.0 + t * 180.0;
    let b = (1.0 - t) * 255.0 + t * 255.0;
    Color::RGB(r as u8, g as u8, b as u8)
}

fn hit_sphere(center: Point3, radius: f32, r: Ray) -> f32 {
    let oc: Vector3 = r.origin - center;
    let a: f32 = oc.dot(oc) - radius * radius;
    let b: f32 = 2.0 * oc.dot(r.direction);
    let c: f32 = r.direction.dot(r.direction);
    let discriminant: f32 = b * b - 4.0 * c * a;
    if (discriminant < 0.0) {
        -1.0
    } else {
        (-b - discriminant.sqrt() ) / (2.0*c)
    }
}