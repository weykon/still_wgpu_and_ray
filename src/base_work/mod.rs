use dpi::{PhysicalPosition, Position};
use event::WindowEvent;
use event_loop::EventLoop;
use platform::macos::EventLoopBuilderExtMacOS;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;
use wgpu::*;
use winit::*;
use winit::{application::ApplicationHandler, dpi::PhysicalSize, window::Window};

pub fn entry() {
    println!("Hello from base_work");
    let event_loop = EventLoop::with_user_event()
        .with_activate_ignoring_other_apps(false)
        .build()
        .unwrap();
    let mut app = App {
        wgpu_thing: None,
        app_state: 0,
        window: None,
    };
    event_loop.run_app(&mut app).unwrap();
}

struct App {
    wgpu_thing: Option<WgpuThing>,
    app_state: i32,
    window: Option<Box<Window>>,
}

struct WgpuThing {
    device: Device,
    queue: Queue,
    surface: Surface<'static>,
    window: Rc<Window>,
    config: SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: RenderPipeline,
}

impl ApplicationHandler<WgpuThing> for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        println!("Resumed");
        let window_attris = winit::window::WindowAttributes::default()
            .with_title("Fantastic window number one!")

            .with_inner_size(winit::dpi::LogicalSize::new(860.0, 640.0))
            .with_active(false);

        let window: Window = event_loop.create_window(window_attris).unwrap();
        window.set_outer_position(Position::Physical(PhysicalPosition::new(0, 0)));
        self.window = Some(Box::new(window));
    }

    fn window_event(
        &mut self,
        event_loop: &event_loop::ActiveEventLoop,
        window_id: window::WindowId,
        event: event::WindowEvent,
    ) {
        if let Some(wgpu_thing) = self.wgpu_thing.as_mut() {
            match event {
                WindowEvent::Resized(size) => {
                    wgpu_thing.size = size;
                    wgpu_thing.config.width = size.width;
                    wgpu_thing.config.height = size.height;
                    wgpu_thing
                        .surface
                        .configure(&wgpu_thing.device, &wgpu_thing.config);
                }
                WindowEvent::RedrawRequested => {
                    println!("Redraw requested");
                }
                _ => {}
            }
        }
    }
}
