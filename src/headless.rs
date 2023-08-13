pub mod state;
use state::State;

use crate::{GpuInfo, Particle};
use wgpu::util::DeviceExt;

pub async fn run(mut gpu_info: GpuInfo, particles: Vec<Particle>, indexes: Vec<usize>) {
    println!("{:?}", indexes);
    let state: State = State::new(gpu_info, particles).await;
    let n: usize = state.particles.len();
    let p_size: u64 = (n * std::mem::size_of::<Particle>()) as u64;
    let workgroups: u32 = ((n + 255 as usize) / 256 as usize) as u32;
    gpu_info.matrix = [[0.0; 4]; 4];
    {
        let mut encoder: wgpu::CommandEncoder =
            state
                .display
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Command Encoder"),
                });

        encoder.copy_buffer_to_buffer(&state.cur_init, 0, &state.cur, 0, p_size);

        state.display.queue.submit([encoder.finish()]);
    }

    loop {
        let mut encoder: wgpu::CommandEncoder =
            state
                .display
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Command Encoder"),
                });
        let new_gpu_info: wgpu::Buffer =
            state
                .display
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("GpuInfo Buffer"),
                    contents: bytemuck::cast_slice(&[gpu_info]),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_SRC,
                });

        encoder.copy_buffer_to_buffer(
            &new_gpu_info,
            0,
            &state.gpu_buffer,
            0,
            std::mem::size_of::<GpuInfo>() as u64,
        );

        for _ in 0..3 {
            encoder.copy_buffer_to_buffer(&state.cur, 0, &state.prev, 0, p_size);
            let mut cpass: wgpu::ComputePass<'_> =
                encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                    label: Some("Compute Pass"),
                });
            cpass.set_pipeline(&state.comp_pipeline);
            cpass.set_bind_group(0, &state.bind_group, &[]);
            cpass.dispatch_workgroups(workgroups + 1, 1, 1);
        }
    }
}
