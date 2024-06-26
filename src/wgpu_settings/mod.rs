use std::{
    rc::Weak,
    sync::{Arc, Mutex},
};

use crate::{
    base_work::{App, WgpuThing},
    scenes::{Scene, Scene1},
};
use anyhow::*;
use wgpu::{core::present::ConfigureSurfaceError, Device, Queue, Surface, SurfaceConfiguration};
use winit::dpi::PhysicalSize;

impl App {
    pub async fn connect_to_gpu(&mut self) -> Result<()> {
        println!("Connecting to GPU");
        let window = match self.window.clone() {
            Some(window) => window,
            None => return Err(anyhow!("Window not found")),
        };

        let instance = wgpu::Instance::default();
        let surface = unsafe { instance.create_surface(window.clone()).unwrap() };
        println!("Surface created");

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or(anyhow!("Failed to find an appropriate adapter"))?;
        println!("Adapter found : {}", adapter.get_info().name);

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .map_err(|_| anyhow!("Failed to create device"))?;
        println!("Device created : {:?}", device.global_id());
        let size = PhysicalSize::new(860, 640);
        println!("size : {:?}", size);

        let config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();
        println!("Config created : {:?}", config.present_mode);
        let scene = Box::new(Scene1 {
            name: "Scene1".to_string(),
        });

        self.wgpu_thing = Some(Arc::new(Mutex::new(WgpuThing {
            device,
            queue,
            surface,
            config,
            size: PhysicalSize::new(860, 640),
            adapter,
        })));

        let pipeline = scene.prepare_pipeline(&self);
        self.pipeline = pipeline;
        println!("Pipeline created");

        Ok(())
    }

    pub fn render_frame(&self, target: &wgpu::TextureView) {
        println!("Rendering frame");
        let mut encoder = self
            .wgpu_thing
            .as_ref()
            .unwrap()
            .lock()
            .unwrap()
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("render frame"),
            });

        let pipeline = self.pipeline.as_ref().unwrap();
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
            render_pass.draw(0..3, 0..1);
            println!("Drawing");
        };

        let command_buffer = encoder.finish();
        let queue = &self.wgpu_thing.as_ref().unwrap().lock().unwrap().queue;
        queue.submit(Some(command_buffer));
    }
}
