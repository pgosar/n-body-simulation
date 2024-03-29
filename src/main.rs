#![deny(nonstandard_style, unused)]

use rand::Rng;

mod gen;
mod render;
mod headless;

use {
    cgmath::{Matrix4, Vector3, Point3, PerspectiveFov, Rad},
    serde::{Deserialize, Serialize},
    std::f32::consts::PI,
    rand::SeedableRng,
};

const CALIBRATE: f32 = 1e-1;

#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable, Serialize, Deserialize)]
#[repr(C)]
pub struct Particle {
    pos: [f32; 3],
    _pad1: f32,
    vel: [f32; 3],
    _pad2: f32,
    mass: f32,
    calibrate: f32,
    _pad3: [f32; 2],
}

#[derive(Deserialize, Clone, Debug, Copy)]
pub enum Galaxy {
    Particle {
        pos: [f32; 3],
        vel: [f32; 3],
        mass: f32,
    },
    Init {
        center_pos: [f32; 3],
        center_vel: [f32; 3],
        center_mass: f32,
        amount: u32,
        normal: [f32; 3],
    },
}

#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct GpuInfo {
    matrix: [[f32; 4]; 4],
    particles: u32,
    motion: f32,
    _pad1: [f32; 2],
}

impl Particle {
    fn new(pos: [f32; 3], vel: [f32; 3], mass: f32, calibrate: f32) -> Self {
        Self {
            pos,
            vel,
            mass,
            calibrate,
            _pad1: 0.0,
            _pad2: 0.0,
            _pad3: [0.0; 2],
        }
    }
}

fn build_matrix(pos: Point3<f32>, dir: Vector3<f32>, aspect: f32) -> Matrix4<f32> {
    Matrix4::from(PerspectiveFov {
        fovy: Rad(PI / 2.0),
        aspect,
        near: 1E-10,
        far: 1E7,
    }) * Matrix4::look_to_rh(pos, dir, Vector3::new(0.0, 1.0, 0.0))
}

pub fn init_galaxy(calibrate: f32, galaxies: Vec<Galaxy>) -> Vec<Particle> {
    let mut particles: Vec<Particle> = Vec::new();
    for c in &galaxies {
        particles.push(match c {
            Galaxy::Particle { pos, vel, mass } => {
                Particle::new((*pos).into(), (*vel).into(), *mass, calibrate)
            }
            Galaxy::Init {
                center_pos,
                center_vel,
                center_mass,
                ..
            } => Particle::new(
                (*center_pos).into(),
                (*center_vel).into(),
                *center_mass,
                calibrate,
            ),
        })
    }

    for i in &galaxies {
        if let Galaxy::Init {
            center_pos,
            center_vel,
            center_mass,
            amount,
            normal,
        } = i
        {
            gen::formation(
                &mut particles,
                *amount,
                CALIBRATE,
                (*center_pos).into(),
                (*center_vel).into(),
                *center_mass,
                (*normal).into(),
            );
        }
    }
    particles
}

fn main() {
    let galaxies: Vec<Galaxy> = vec![
        Galaxy::Init {
            center_pos: [-2e-9, -2e-9, 0.0],
            center_vel: [1e-14, 0.0, 0.0],
            center_mass: 1e14,
            amount: 10000,
            normal: [1.0, 0.0, 0.0],
        },
        Galaxy::Init {
            center_pos: [2e-9, 2e-9, 0.0],
            center_vel: [0.0, 0.0, 0.0],
            center_mass: 4e14,
            amount: 10000,
            normal: [1.0, 1.0, 0.0],
        },
    ];

    let particles: Vec<Particle> = init_galaxy(CALIBRATE, galaxies);
    let gpu_info: GpuInfo = GpuInfo {
        matrix: Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0)).into(),
        particles: particles.len() as u32,
        motion: 2.0,
        _pad1: [0.0; 2],
    };

    let render: bool = false;
    if render {
        pollster::block_on(render::run(gpu_info, particles));
    } else {
        let mut rng = rand::rngs::StdRng::seed_from_u64(0);
        let mut indexes: Vec<usize> = Vec::new();
        indexes.push(0);
        indexes.push(1);
        for _ in 0..4 {
            let index: usize = rng.gen_range(2..particles.len()/2);
            let index2: usize = rng.gen_range(2..particles.len()/2) + particles.len()/2;
            indexes.push(index);
            indexes.push(index2);
        }

        pollster::block_on(headless::run(gpu_info, particles, indexes));        
    }
}