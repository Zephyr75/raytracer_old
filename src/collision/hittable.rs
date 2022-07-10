use crate::math::point3::Point3;
use crate::math::vector3::Vector3;
use crate::math::ray::Ray;


#[derive(Debug, Copy, Clone)]
pub struct Hit {
    // The hit point
    pub point: Point3,
    // The normal vector at the hit point
    pub normal: Vector3,
    // The t value of the hit
    pub t: f32,
    // Whether the face is facing the outside of the object
    pub front_face: bool,
}

pub trait Hittable: Sync + Send {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit: &mut Hit) -> bool;
}

impl Hit{
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vector3){
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}