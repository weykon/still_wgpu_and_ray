use std::{cell::RefCell, ops::Deref, rc::Rc};

use super::Scene;
use crate::base_work::App;
use bytemuck::{Pod, Zeroable};
use wgpu::{core::device, BindGroup, BufferDescriptor, PipelineCompilationOptions, RenderPipeline};
pub struct Scene1 {
    pub name: String,
    // 给render pass用
    pub bind_group: RefCell<Option<BindGroup>>,
}

#[derive(Pod, Zeroable, Copy, Clone)]
#[repr(C)]
struct TheFirstUniformBuffer {
    width: u32,
    height: u32,
}
impl Scene for Scene1 {
    fn prepare_pipeline(&self, app: &App) -> Option<wgpu::RenderPipeline> {
        println!("Rendering Buffer first Bind : {}", self.name);
        let wgpu_thing = &app.wgpu_thing;
        let wgpu_thing = wgpu_thing.as_ref().unwrap().lock().unwrap();
        let shader_moduel = compile_shader_module(&wgpu_thing.device);
        let pipeline = self.create_display_pipeline(&wgpu_thing.device, &shader_moduel);
        return Some(pipeline);
    }
    fn render_frame(&self, app: &App, target: &wgpu::TextureView) {
        println!("Rendering Buffer first Bind ");
        let mut encoder = app
            .wgpu_thing
            .as_ref()
            .unwrap()
            .lock()
            .unwrap()
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("render frame"),
            });

        let pipeline = app.scene_selector.current_pipeline.as_ref().unwrap();
        let binding = self.bind_group.borrow();
        let bind_group = binding.deref().as_ref().unwrap();

        {
            println!("Creating render pass");
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("display pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: target,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            render_pass.set_pipeline(pipeline);
            render_pass.set_bind_group(0, bind_group, &[]);

            render_pass.draw(0..6, 0..1);
            println!("Drawing");
        };

        let command_buffer = encoder.finish();
        let queue = &app.wgpu_thing.as_ref().unwrap().lock().unwrap().queue;
        queue.submit(Some(command_buffer));
    }
}

pub(crate) fn compile_shader_module(device: &wgpu::Device) -> wgpu::ShaderModule {
    use std::borrow::Cow;
    let code = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        // "/assets/buffer_first_bind.wgsl"
        "/assets/add_groud.wgsl"
    ));
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(code)),
    })
}

impl Scene1 {
    pub fn create_display_pipeline(
        &self,
        device: &wgpu::Device,
        shader_module: &wgpu::ShaderModule,
    ) -> RenderPipeline {
        println!("Creating display pipeline");
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        // Cpu => Gpu
        let uniform_data = TheFirstUniformBuffer {
            width: 128u32,
            height: 128u32,
        };
        let uniform_buffer: wgpu::Buffer = device.create_buffer(&BufferDescriptor {
            label: Some(&"the first buffer bind group"),
            size: std::mem::size_of::<TheFirstUniformBuffer>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: true,
        });
        {
            uniform_buffer
                .slice(..)
                .get_mapped_range_mut()
                .copy_from_slice(bytemuck::bytes_of(&uniform_data));
            uniform_buffer.unmap();
        }
        // 创建bind group后写回self.bind_group给render pass用
        *self.bind_group.borrow_mut() =
            Some(device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &uniform_buffer,
                        offset: 0,
                        size: None,
                    }),
                }],
            }));

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("display"),
            layout: Some(
                &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    bind_group_layouts: &[&bind_group_layout],
                    ..Default::default()
                }),
            ),
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
                    format: wgpu::TextureFormat::Bgra8UnormSrgb,
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
}
