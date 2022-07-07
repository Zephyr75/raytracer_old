use crate::point3::Point3;
use crate::vector3::Vector3;


pub struct Hit {
    // The hit point
    pub point: Point3,
    // The normal vector at the hit point
    pub normal: Vector3,
    // The t value of the hit
    pub t: f32,
}

trait Hittable {
    fn hit(&self, Ray: ray, f32: t_min, f32: t_max, &Hit: hit) -> bool;
}