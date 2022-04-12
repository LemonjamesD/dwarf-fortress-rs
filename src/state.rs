use wgpu::{Surface, Queue, Device, SurfaceConfiguration, SurfaceError};
use winit::{dpi::PhysicalSize, event::WindowEvent, window::Window};

pub struct State {
    surface: Surface,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    size: PhysicalSize<u32>,    
}

impl State {
    async fn new(Window: &Window) -> Self {

    }

    fn resize(&mut self, new_size: PhysicalSize<u32>) {

    }

    fn input(&mut self, event: &WindowEvent) -> bool {

    }

    fn update(&mut self) {

    }

    fn render(&mut self) -> Result<(), SurfaceError> {

    }
}