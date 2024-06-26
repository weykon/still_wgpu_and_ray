use base_work::App;
use winit::{
    event_loop::{ControlFlow, EventLoop},
    platform::macos::EventLoopBuilderExtMacOS,
};

mod base_work;
mod camera;
mod scenes;
mod wgpu_settings;
fn main() {
    let event_loop = EventLoop::with_user_event()
        // .with_activate_ignoring_other_apps(false)
        .build()
        .unwrap();
    let mut app = App {
        wgpu_thing: None,
        app_state: 0,
        window: None,
        pipeline: None,
    };
    // event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut app).unwrap();
}
