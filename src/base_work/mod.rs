use dpi::{PhysicalPosition, Position};
use event::{ElementState, KeyEvent, WindowEvent};
use keyboard::KeyCode;
use std::borrow::{Borrow, BorrowMut};
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use wgpu::{SurfaceConfiguration, *};
use winit::*;
use winit::{application::ApplicationHandler, window::Window};

use crate::scenes::{Scene, SceneSelector};

pub struct App {
    pub wgpu_thing: Option<Arc<Mutex<WgpuThing>>>,
    pub app_state: i32,
    pub window: Option<Arc<Window>>,
    pub scene_selector: SceneSelector,
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
            .with_inner_size(winit::dpi::LogicalSize::new(128.0, 128.0))
            .with_active(false);

        let window: Window = event_loop.create_window(window_attris).unwrap();
        window.set_outer_position(Position::Physical(PhysicalPosition::new(2500, 1500)));
        let window = Arc::new(window);
        self.window = Some(Arc::clone(&window));
        pollster::block_on(async move {
            let _ = self.connect_to_gpu().await;
            self.window.as_ref().unwrap().request_redraw();
        });
    }

    fn user_event(&mut self, event_loop: &event_loop::ActiveEventLoop, event: WgpuThing) {}

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
                self.window.as_ref().unwrap().request_redraw();
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

                let render_target = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                self.scene_selector
                    .get_current_scene()
                    .as_ref()
                    .render_frame(&self, &render_target);

                frame.present();
            }
            WindowEvent::KeyboardInput { event, .. } => match event {
                KeyEvent {
                    state: ElementState::Released,
                    physical_key: keyboard::PhysicalKey::Code(KeyCode::Space),
                    ..
                } => {
                    println!("Keyboard input : Space");
                    let current_scene_index = self.scene_selector.current_scene_index;

                    self.scene_selector.select(current_scene_index + 1);

                    let current_scene = self.scene_selector.get_current_scene();

                    let pipeline = current_scene.prepare_pipeline(&self).unwrap();

                    self.scene_selector.current_pipeline = Some(pipeline);

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

                    let render_target = frame
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default());

                    self.scene_selector
                        .get_current_scene()
                        .render_frame(&self, &render_target);

                    self.window.as_ref().unwrap().request_redraw();
                }
                _ => {}
            },
            _ => {}
        }
    }
}
