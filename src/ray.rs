use crate::vec::Vec3;

pub struct Ray {
    pub a: Vec3,
    pub b: Vec3,
}

impl Ray {
    pub const fn origin(&self) -> Vec3 {
        self.a
    }

    pub const fn direction(&self) -> Vec3 {
        self.b
    }

    pub const fn point_at_direction(&self, t: f32) -> Vec3 {
        self.a + self.b*t
    }

    // pub fn hit_sphere(&self, center: &Vec3, radius: f32) -> f32 {
    //     let oc = self.origin() - *center;
    //     let a = self.direction().dot(self.direction());
    //     let b = 2.0 * oc.dot(self.direction());
    //     let c = oc.dot(oc) - radius.powi(2);
    //
    //     let delta = b.powi(2) - 4.0 * a * c;
    //     if delta < 0.0 {
    //         -1.0
    //     } else {
    //         (-b - delta.sqrt()) / (2.0 * a)
    //     }
    // }
}
