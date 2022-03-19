use rand::random;
use crate::material::*;
use crate::ray::Ray;
use crate::vec::Vec3;
use crate::vec3;

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Box<Materials>,
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord>;
}

pub type HitableList = Vec<Box<dyn Hitable + Sync>>;

pub fn random_scene() -> HitableList {
    let mut hitable_list : HitableList = Vec::new();

    hitable_list.push(Box::new(Sphere { center: vec3!(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Materials::Lambertian(Lambertian { albedo: vec3!(0.5, 0.5, 0.5)})}));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f32>();
            let center = vec3!(a as f32 + 0.9 * random::<f32>(), 0.2, b as f32 + 0.9 * random::<f32>());
            if (center - vec3!(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 { //difuse
                    hitable_list.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Materials::Lambertian(
                            Lambertian { albedo: vec3!(random::<f32>()*random::<f32>(),
                                                        random::<f32>()*random::<f32>(),
                                                        random::<f32>()*random::<f32>())}),
                    }));
                }
                else if choose_mat < 0.95 { //metal
                    hitable_list.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Materials::Metal(
                            Metal { albedo: vec3!(0.5*(1.0 + random::<f32>()), 0.5*(1.0 + random::<f32>()), 0.5*(1.0 + random::<f32>())),
                                    fuzz: 0.5*random::<f32>()}
                        )
                    }));
                } else { //glass
                    hitable_list.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Materials::Dielectric(Dielectric { ref_index: 1.5 })
                    }));
                }
            }
        }
    }

    hitable_list.push(Box::new(Sphere {
        center: vec3!(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Materials::Dielectric(Dielectric { ref_index: 1.5 })
    }));
    hitable_list.push(Box::new(Sphere {
        center: vec3!(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Materials::Lambertian(Lambertian { albedo: vec3!(0.4, 0.2, 0.1) })
    }));
    hitable_list.push(Box::new(Sphere {
        center: vec3!(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Materials::Metal(Metal { albedo: vec3!(0.7, 0.6, 0.5), fuzz: 0.0 })
    }));

    hitable_list
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        let mut closest_so_far = tmax;
        let mut return_record : Option<HitRecord> = None;
        for hitable in self {
            if let Some(rec) = hitable.hit(ray, tmin, closest_so_far) {
                closest_so_far = rec.t;
                return_record = Some(rec);
            }
        }
        return_record
    }
}

pub struct Sphere {
    pub radius: f32,
    pub center: Vec3,
    pub material: Materials,
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(ray.direction());
        let b = 2.0 * oc.dot(ray.direction());
        let c = oc.dot(oc) - self.radius.powi(2);

        let delta = b.powi(2) - 4.0 * a * c;
        if delta > 0.0 {
            let mut r1 = (-b - crate::fsqrt(delta)) / (2.0 * a);
            if (tmin..tmax).contains(&r1) {
                return Some(HitRecord {
                    t: r1,
                    p: ray.point_at_direction(r1),
                    normal: (ray.point_at_direction(r1) - self.center) / self.radius,
                    material: Box::new(self.material),
                })
            }
            r1 = (-b + crate::fsqrt(delta)) / (2.0 * a);
            if (tmin..tmax).contains(&r1) {
                return Some(HitRecord {
                    t: r1,
                    p: ray.point_at_direction(r1),
                    normal: (ray.point_at_direction(r1) - self.center) / self.radius,
                    material: Box::new(self.material),
                })
            }
        }
        None
    }
}
