use crate::{
    hit::HitRecord,
    ray::Ray,
    vec3::{dot, random_in_unit_sphere, random_unit_vector, reflect, unit_vector, Vec3},
};

pub trait Material {
    fn scatter(
        &mut self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &mut self,
        _: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = record.normal + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = record.normal;
        }
        *scattered = Ray::new(record.point, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &mut self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(unit_vector(ray_in.direction), record.normal);
        *scattered = Ray::new(
            record.point,
            reflected + self.fuzz * random_in_unit_sphere(),
        );
        *attenuation = self.albedo;
        dot(scattered.direction, record.normal) > 0.0
    }
}

pub struct Dialectric {
    refraction_index: f64,
}

impl Dialectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dialectric {
    fn scatter(
        &mut self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let refraction_ratio = if record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = unit_vector(ray_in.direction);
        let refracted = refract(unit_direction, record.normal, refraction_ratio);
        *scattered = Ray::new(record.point, refracted);
        true
    }
}

pub fn refract(uv: Vec3, normal: Vec3, etai_over_etat: f64) -> Vec3 {
    let cons_theta = f64::min(dot(-uv, normal), 1.0);
    let ray_out_perpendicular = etai_over_etat * (uv + cons_theta * normal);
    let ray_out_parallel = -(1.0 - ray_out_perpendicular.length_squared()).abs().sqrt() * normal;
    ray_out_perpendicular + ray_out_parallel
}
