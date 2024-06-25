use std::{
    rc::Weak,
    sync::{Arc, Mutex},
};

use crate::{
    base_work::{App, WgpuThing},
    scenes::Scene1,
};
use anyhow::*;
use wgpu::{core::present::ConfigureSurfaceError, Device, Queue, Surface, SurfaceConfiguration};
use winit::dpi::PhysicalSize;

impl App {
    pub async fn connect_to_gpu(&mut self) -> Result<()> {
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
        println!("Config created : {:?}", config);
        self.wgpu_thing = Some(Arc::new(Mutex::new(WgpuThing {
            device,
            queue,
            surface,
            config,
            size: PhysicalSize::new(860, 640),
            adapter,
            render: Box::new(Scene1 {
                name: "Scene1".to_string(),
                wpgu_thing: None,
            }),
        })));
        self.wgpu_thing.as_mut().unwrap().lock().unwrap().render = Box::new(Scene1 {
            name: "Scene1".to_string(),
            wpgu_thing: Some(Arc::downgrade(&self.wgpu_thing.as_ref().unwrap())),
        });
        Ok(())
    }
}
