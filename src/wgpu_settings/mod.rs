use crate::base_work::{App, WgpuThing};
use crate::scenes::SceneSelector;
use anyhow::*;
use std::borrow::BorrowMut;
use std::sync::{Arc, Mutex};
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

        self.wgpu_thing = Some(Arc::new(Mutex::new(WgpuThing {
            device,
            queue,
            surface,
            config,
            size: PhysicalSize::new(860, 640),
            adapter,
        })));

        self.scene_selector.current_pipeline = Some(
            self.scene_selector
                .get_current_scene()
                .prepare_pipeline(&self)
                .unwrap(),
        );

        println!("Pipeline created");
        Ok(())
    }
}
