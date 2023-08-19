use glam::Vec3;

use crate::{
    hits::HitRecord,
    ray::{self, Ray},
    sphere, utils,
};

pub trait Material {
    fn scatter(
        &self,
        ray: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}

#[derive(Debug, Clone, Copy)]
pub enum MaterialType {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3 },
}

impl Material for MaterialType {
    fn scatter(
        &self,
        ray: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            MaterialType::Lambertian { albedo } => {
                let target = record.p + record.normal + sphere::random_in_unit_sphere();
                *scattered = Ray::new(record.p, target - record.p);
                *attenuation = *albedo;

                true
            }
            MaterialType::Metal { albedo } => {
                let reflected = ray::reflect(utils::unit_vec(ray.direction()), record.normal);
                *scattered = Ray::new(record.p, reflected);
                *attenuation = *albedo;

                scattered.direction().dot(record.normal) > 0.0
            }
        }
    }
}