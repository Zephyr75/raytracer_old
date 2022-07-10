
use crate::math::point3::Point3;
use crate::math::ray::Ray;
use crate::collision::hittable::Hit;
use crate::collision::hittable::Hittable;

pub struct Sphere {
    // The center of the sphere
    pub center: Point3,
    // The radius of the sphere
    pub radius: f32,
}


impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit: &mut Hit) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.length().powi(2);
        let half_b = oc.dot(ray.direction);
        let c = oc.length().powi(2) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrt_discriminant = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrt_discriminant) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrt_discriminant) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }

        hit.t = root;
        hit.point = ray.at(hit.t);
        let outward_normal = (hit.point - self.center) / self.radius;
        hit.set_face_normal(ray, outward_normal);

        true
    }
}