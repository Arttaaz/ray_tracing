use crate::vec::Vec3;
use crate::ray::Ray;

pub struct Camera {
    pub origin: Vec3,
    pub ll_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {

    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f32,
        aspect: f32,
        _aperture: f32,
        focus_dist: f32) -> Camera {

        // let lens_radius = aperture / 2.0;
        let theta = vfov * std::f32::consts::PI / 180.0;
        let half_height = (theta/2.0).tan();
        let half_width = aspect * half_height;
        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        Camera {
            origin: lookfrom,
            ll_corner: lookfrom - u * half_width * focus_dist - v * half_height * focus_dist - w * focus_dist,
            horizontal: u * 2.0 * half_width * focus_dist,
            vertical: v * 2.0 * half_height * focus_dist,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            a: self.origin,
            b: self.ll_corner + self.horizontal * u + self.vertical * v - self.origin,
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            origin: Vec3::default(),
            ll_corner: Vec3 { x: -2.0, y: -1.0, z: -1.0 },
            horizontal: Vec3 { x: 4.0, y: 0.0, z: 0.0},
            vertical: Vec3 { x: 0.0, y: 2.0, z: 0.0},
        }
    }
}
