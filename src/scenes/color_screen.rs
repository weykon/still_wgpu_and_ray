use wgpu::PipelineCompilationOptions;

use crate::base_work::App;

use super::Scene;

pub struct Scene1 {
    pub name: String,
}

impl Scene for Scene1 {
    fn prepare_pipeline(&self, app: &App) -> Option<wgpu::RenderPipeline> {
        println!("Rendering Color Screen : {}", self.name);
        let wgpu_thing = &app.wgpu_thing;
        let wgpu_thing = wgpu_thing.as_ref().unwrap().lock().unwrap();
        let shader_moduel = compile_shader_module(&wgpu_thing.device);
        return Some(create_display_pipeline(&wgpu_thing.device, &shader_moduel));
    }
    fn render_frame(&self, app: &App, target: &wgpu::TextureView) {
        println!("Rendering Color Screen frame");
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
        "/assets/color_screen.wgsl"
    ));
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(code)),
    })
}

pub(crate) fn create_display_pipeline(
    device: &wgpu::Device,
    shader_module: &wgpu::ShaderModule,
) -> wgpu::RenderPipeline {
    println!("Creating display pipeline");
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
