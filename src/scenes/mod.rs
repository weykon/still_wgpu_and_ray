use std::{
    borrow::Borrow,
    sync::{Mutex, Weak},
};

use wgpu::{PipelineCompilationOptions, PipelineLayoutDescriptor};

use crate::base_work::WgpuThing;

pub fn entry() {}

pub trait Scene {
    fn render(&self);
}

pub struct Scene1 {
    pub name: String,
    pub(crate) wpgu_thing: Option<Weak<Mutex<WgpuThing>>>,
}

impl Scene for Scene1 {
    fn render(&self) {
        println!("Rendering {}", self.name);
        let Some(ref wgpu_thing) = self.wpgu_thing else {
            return;
        };
        let wgpu_thing = wgpu_thing.upgrade().unwrap();
        let wgpu_thing = wgpu_thing.lock().unwrap();
        let shader_moduel = compile_shader_module(&wgpu_thing.device);
        create_display_pipeline(&wgpu_thing.device, &shader_moduel);
    }
}

fn compile_shader_module(device: &wgpu::Device) -> wgpu::ShaderModule {
    use std::borrow::Cow;
    let code = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/one.wgsl"));
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(code)),
    })
}

fn create_display_pipeline(
    device: &wgpu::Device,
    shader_module: &wgpu::ShaderModule,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("display"),
        layout: None,
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            front_face: wgpu::FrontFace::Ccw,
            polygon_mode: wgpu::PolygonMode::Fill,
            ..Default::default()
        },
        vertex: wgpu::VertexState {
            module: shader_module,
            entry_point: "display_vs",
            buffers: &[],
            compilation_options: PipelineCompilationOptions::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: shader_module,
            entry_point: "display_fs",
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Bgra8Unorm,
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: PipelineCompilationOptions::default(),
        }),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    })
}
