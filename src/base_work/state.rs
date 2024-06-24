use wgpu::*;
use winit::window::Window;
pub struct State {
    device: Device,
    queue: Queue,
}

#[derive(Default)]
pub struct ControlFlowDemo {
    pub mode: Mode,
    pub request_redraw: bool,
    pub wait_cancelled: bool,
    pub close_requested: bool,
    pub window: Option<Window>,
}
use std::time;
const WAIT_TIME: time::Duration = time::Duration::from_millis(100);
const POLL_SLEEP_TIME: time::Duration = time::Duration::from_millis(100);
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    #[default]
    Wait,
    WaitUntil,
    Poll,
}
