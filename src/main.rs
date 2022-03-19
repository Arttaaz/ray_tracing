#![feature(const_fn_floating_point_arithmetic,
           const_trait_impl)]
extern crate rand;
extern crate rayon;

use crate::rand::Rng;
use rand::thread_rng;
use crate::rayon::iter::ParallelIterator;
use crate::rayon::iter::IntoParallelIterator;
use crate::material::*;
//use rand::random;

union Fi {
    f: f32,
    i: u32,
}

pub const fn fsqrt(x: f32) -> f32 {
	unsafe {
        let mut y = Fi {f: 0.0};
        y.f = x;
        const threehalfs: f32 = 1.5;

        let x2 = x * 0.5;
        y.i  = 0x5f3759df - ( y.i >> 1 );               // what the fuck? 
        y.f  = y.f * ( threehalfs - ( x2 * y.f * y.f ) );   // 1st iteration
        y.f  = y.f * ( threehalfs - ( x2 * y.f * y.f ) );   // 2nd iteration
        y.f  = y.f * ( threehalfs - ( x2 * y.f * y.f ) );   // 2nd iteration

        x * y.f
    }
}

mod camera;
mod hitable;
mod material;
mod ray;
#[macro_use]
mod vec;

use camera::Camera;
use hitable::*;
use ray::Ray;
use vec::Vec3;

fn color(ray: Ray, world: &HitableList, depth: u8) -> Vec3 {
    if let Some(rec) = world.hit(&ray, 0.001, std::f32::MAX) {
        if depth < 50 {
            if let Some((attenuation, scattered)) = rec.material.scatter(&ray, &rec) {
                return attenuation * color(scattered, world, depth+1)
            } else {
                return Vec3::default()
            }
        } else {
            return Vec3::default()
        }
    } else {
        let unit_direction = ray.direction().unit_vector();
        let t : f32 = 0.5 * (unit_direction.y + 1.0);
        vec3!(1.0, 1.0, 1.0) * (1.0-t) + vec3!(0.5, 0.7, 1.0) * t
    }
}

fn main() {
    let nx = 800;
    let ny = 400;
    let ns = 100;
    let lookfrom = vec3!(6.5, 1.5, 2.0);
    let lookat = vec3!(0.0, 0.0, -1.0);
    let dist_focus = (lookfrom - lookat).length();
    let camera = Camera::new(
            lookfrom,
            lookat,
            vec3!(0.0, 1.0, 0.0 ),
            70.0,
            nx as f32 / ny as f32,
            2.0,
            dist_focus);

    // let world : HitableList = vec![
    //     Box::new(Sphere {center: vec3!(0.0, 0.0, -1.0), radius: 0.5,
    //         material: Materials::Lambertian(Lambertian { albedo: vec3!(0.1, 0.2, 0.5 ),}) }),
    //     Box::new(Sphere {center: vec3!(0.0, -100.5, -1.0), radius: 100.0,
    //         material: Materials::Lambertian(Lambertian { albedo: vec3!(0.8, 0.8, 0.0 ),}) }),
    //     Box::new(Sphere {center: vec3!(1.0, 0.0, -1.0), radius: 0.5,
    //         material: Materials::Metal(Metal::new(vec3!(0.8, 0.6, 0.2 ), 0.3)) }),
    //     Box::new(Sphere {center: vec3!(-1.0, 0.0, -1.0), radius: 0.5,
    //         material: Materials::Dielectric(Dielectric { ref_index: 1.5 }) }),
    //     Box::new(Sphere {center: vec3!(-1.0, 0.0, -1.0), radius: -0.45,
    //         material: Materials::Dielectric(Dielectric { ref_index: 1.5 }) }),
    //     ];
    let world = hitable::random_scene();

    println!("P3\n{} {}\n255", nx, ny);
    // for j in (0..(ny)).rev() {
    //     for i in 0..nx {
    //         let mut col = Vec3::default();
    //         for _ in 0..ns {
    //             let u = (i as f32 + random::<f32>()) / nx as f32;
    //             let v = (j as f32 + random::<f32>()) / ny as f32;
    //             let ray = camera.get_ray(u, v);
    //
    //             col += color(ray, &world, 0);
    //         }
    //
    //         col /= ns as f32;
    //         let v = Vec3 {
    //             x: 255.99*col.x.sqrt(),
    //             y: 255.99*col.y.sqrt(),
    //             z: 255.99*col.z.sqrt(),
    //         };
    //         println!("{} {} {}", v.x as u64, v.y as u64, v.z as u64);
    //     }
    // }
    //
    let v : Vec<Vec3> = (0..nx * ny)
       .into_par_iter()
       .map_init(
           || thread_rng(),
           |rng, screen_pos| {
               let mut c = vec3!(0.0, 0.0, 0.0);
               let j = ny - 1 - screen_pos / nx;
               let i = screen_pos % nx;
               for _ in 0..ns {
                   let u = ((i as f32) + rng.gen::<f32>()) / (nx as f32);
                   let v = ((j as f32) + rng.gen::<f32>()) / (ny as f32);
                   let r = camera.get_ray(u, v);
                   c += color(r, &world, 0);
               }
               c /= ns as f32;
               let ir = 255.99 * c.x.sqrt();
               let ig = 255.99 * c.y.sqrt();
               let ib = 255.99 * c.z.sqrt();

               vec3!(ir, ig, ib)
           },
).collect();

for e in v {
    println!("{} {} {}", e.x as u32, e.y as u32, e.z as u32);
}}
