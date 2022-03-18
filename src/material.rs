use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::vec::Vec3;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)>;
}

#[derive(Copy, Clone)]
pub enum Materials {
    Dielectric(Dielectric),
    Lambertian(Lambertian),
    Metal(Metal),
}

impl Material for Materials {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        match self {
            Materials::Lambertian(l) => l.scatter(ray, hit_record),
            Materials::Metal(m) => m.scatter(ray, hit_record),
            Materials::Dielectric(d) => d.scatter(ray, hit_record),
        }
    }

}

#[derive(Copy, Clone)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let target = hit_record.p + hit_record.normal + Vec3::random_in_unit_sphere();
        Some((self.albedo, Ray { a: hit_record.p, b: target - hit_record.p }))
    }
}

#[derive(Copy, Clone)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Metal {
        Metal {
            albedo,
            fuzz: if fuzz > 1.0 { 1.0 } else if fuzz < 0.0 { 0.0 } else { fuzz },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = ray.direction().unit_vector().reflect(hit_record.normal);
        let scattered = Ray { a: hit_record.p, b: reflected + Vec3::random_in_unit_sphere() * self.fuzz };
        if scattered.direction().dot(hit_record.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }

}

#[derive(Copy, Clone)]
pub struct Dielectric {
    pub ref_index: f32,
}

impl Dielectric {
    fn schlick(&self, cosine: f32) -> f32 {
        let r0 = ((1.0 - self.ref_index) / (1.0 + self.ref_index)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let (ni_over_nt, outward_normal, cosine) = if ray.direction().dot(hit_record.normal) > 0.0 {
            (
                self.ref_index,
                -hit_record.normal,
                self.ref_index * ray.direction().dot(hit_record.normal) / ray.direction().length(),
            )
        } else {
            (
                1.0 / self.ref_index,
                hit_record.normal,
                -ray.direction().dot(hit_record.normal) / ray.direction().length(),
            )
        };
        let reflected = ray.direction().reflect(hit_record.normal);
        if let Some(refracted) = ray.direction().refract(outward_normal, ni_over_nt) {
            if rand::random::<f32>() < self.schlick(cosine) {
                Some((Vec3 { x: 1.0, y: 1.0, z: 1.0}, Ray { a: hit_record.p, b: reflected }))
            } else {
                Some((Vec3 { x: 1.0, y: 1.0, z: 1.0}, Ray { a: hit_record.p, b: refracted }))
            }
        } else {
            Some((Vec3 { x: 1.0, y: 1.0, z: 1.0}, Ray { a: hit_record.p, b: reflected }))
        }
    }
}
