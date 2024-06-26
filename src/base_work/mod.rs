use dpi::{PhysicalPosition, Position};
use event::WindowEvent;
use std::borrow::Borrow;
use std::sync::{Arc, Mutex};
use wgpu::{SurfaceConfiguration, *};
use winit::*;
use winit::{application::ApplicationHandler, window::Window};

use crate::scenes::Scene;

pub struct App {
    pub wgpu_thing: Option<Arc<Mutex<WgpuThing>>>,
    pub app_state: i32,
    pub window: Option<Arc<Window>>,
    pub pipeline: Option<wgpu::RenderPipeline>,
}

pub struct WgpuThing {
    pub device: Device,
    pub queue: Queue,
    pub surface: Surface<'static>,
    pub config: SurfaceConfiguration,
    pub adapter: Adapter,
    pub size: winit::dpi::PhysicalSize<u32>,
}

impl ApplicationHandler<WgpuThing> for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        println!("Resumed");
        let window_attris = winit::window::WindowAttributes::default()
            .with_title("Fantastic window number one!")
            .with_inner_size(winit::dpi::LogicalSize::new(860.0, 640.0));

        let window: Window = event_loop.create_window(window_attris).unwrap();
        window.set_outer_position(Position::Physical(PhysicalPosition::new(0, 0)));
        let window = Arc::new(window);
        self.window = Some(Arc::clone(&window));
        pollster::block_on(async move {
            let _ = self.connect_to_gpu().await;
            self.window.as_ref().unwrap().request_redraw();
        });
    }

    fn window_event(
        &mut self,
        event_loop: &event_loop::ActiveEventLoop,
        window_id: window::WindowId,
        event: event::WindowEvent,
    ) {
        match event {
            WindowEvent::Resized(size) => {
                println!("Resized to {:?}", size);
                let Some(ref wgpu_thing) = self.wgpu_thing else {
                    return;
                };
                let mut wgpu_thing = wgpu_thing.lock().unwrap();
                wgpu_thing.size = size;
                wgpu_thing.config.width = size.width;
                wgpu_thing.config.height = size.height;
                wgpu_thing
                    .surface
                    .configure(&wgpu_thing.device, &wgpu_thing.config);
            }
            WindowEvent::RedrawRequested => {
                println!("Redraw requested");
                // Wait for the next available frame buffer.
                let frame: wgpu::SurfaceTexture = self
                    .wgpu_thing
                    .as_ref()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .surface
                    .get_current_texture()
                    .expect("failed to get current texture");

                // TODO: draw frame
                let render_target = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                self.render_frame(&render_target);
                frame.present();
            }
            _ => {}
        }
    }
}
