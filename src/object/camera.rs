use crate::math::point3::Point3;
use crate::math::vector3::Vector3;

pub struct Camera {
    // The aspect ratio of the camera
    pub aspect_ratio: f32,
    // The height of the viewport
    pub viewport_height: f32,
    // The width of the viewport
    pub viewport_width: f32,
    // The focal length of the camera
    pub focal_length: f32,
    // The origin of the camera
    pub origin: Point3,
    // The vertical vector of the camera
    pub vertical: Vector3,
    // The horizontal vector of the camera
    pub horizontal: Vector3,
    // The lower left corner of the viewport
    pub lower_left_corner: Point3,
}

impl Default for Camera {
    fn default() -> Camera {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;
        let origin = Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let vertical = Vector3 {
            x: 0.0,
            y: viewport_height,
            z: 0.0,
        };
        let horizontal = Vector3 {
            x: viewport_width,
            y: 0.0,
            z: 0.0,
        };
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vector3{x: 0.0, y: 0.0, z: focal_length};

        Camera {
            aspect_ratio,
            viewport_height,
            viewport_width,
            focal_length,
            origin,
            vertical,
            horizontal,
            lower_left_corner,
        }
    }
}