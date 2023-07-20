use {
    crate::Particle,
    cgmath::{
        prelude::*,
        {Point3, Vector3},
    },
    // rand::{thread_rng, Rng},
    rand::prelude::*,
    std::f32::consts::PI,
};

pub fn create(
    angle: f32,
    normal: Vector3<f32>,
    particles: &mut Vec<Particle>,
    calibrate: f32,
    center_pos: Point3<f32>,
    center_vel: Vector3<f32>,
    center_mass: f32,
    radius: f32,
) {
    // refactor: pull out of for loop
    // normalize(cross(N, T')), T' is arbitrary vector
    let tangent: Vector3<f32> = normal.cross(Vector3::new(-normal.z, normal.x, normal.y));
    // cross(N, T) for movement
    let particle_vectors: Vector3<f32> =
        tangent * angle.sin() + normal.cross(tangent) * angle.cos();
    let movement: Vector3<f32> = particle_vectors.cross(normal).normalize();
    // pos = center + offset * radius
    let pos: Point3<f32> = center_pos + particle_vectors * radius;
    let gravity: f32 = 0.00001;
    // gravitational acceleration formula
    let speed: f32 = (gravity * center_mass * radius as f32
        / ((radius * radius) as f32 + calibrate))
        .sqrt() as f32;
    // V' = V+g, g = gravitational acceleration * vector of movement
    let vel = center_vel + movement * speed;
    particles.push(Particle::new(pos.into(), vel.into(), 0.0, calibrate));
}

pub fn formation(
    particles: &mut Vec<Particle>,
    amount: u32,
    calibrate: f32,
    center_pos: Point3<f32>,
    center_vel: Vector3<f32>,
    center_mass: f32,
    normal: Vector3<f32>,
) {
    for _ in 0..amount / 7 {
        let radius = 50.0 + thread_rng().gen_range(0.0..100.0);
        let angle = thread_rng().gen::<f32>() * 2.0 * PI;
        create(
            angle,
            normal.normalize(),
            particles,
            calibrate,
            center_pos,
            center_vel,
            center_mass,
            radius,
        );
    }
    // based on number of stars in the arms vs center of Milky Way (80%)
    for _ in 0..amount/7*6 {
        let arms = 2;
        let radius = 50.0 + thread_rng().gen_range(0.0..100.0);
        // θ = (2π / N) * A + f(r), N=total arms, A=arm number`
        // f(r) is a function that includes variation in the number
        // arm number
        let arm = thread_rng().gen_range(0..arms);
        let angle = (arm as f32 / (arms as f32) * 2.0 * PI) - (radius * 0.01)
            + thread_rng().gen_range(0.0..0.2);
        //let angle = (2.0 * PI / arms as f32) * arm as f32 + thread_rng().gen_range(-0.05..=0.15);
        create(
            angle,
            normal.normalize(),
            particles,
            calibrate,
            center_pos,
            center_vel,
            center_mass,
            radius,
        );
    }
}
