use crate::app::App;
use tokio;
use winit::error::EventLoopError;
use winit::event_loop::{ControlFlow, EventLoop};

pub mod app;

pub fn counter() {}

#[tokio::main]
async fn main() -> Result<(), EventLoopError> {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::default();
    event_loop.run_app(&mut app)
}
