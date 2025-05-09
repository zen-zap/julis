use pixels::{Pixels, SurfaceTexture};
use winit::{
	dpi::LogicalSize,
	event::{Event, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
	window::{WindowBuilder},
};

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new()
        .with_title("Mandelbrot Set")
        .with_inner_size(LogicalSize::new(800.0, 600.0))
        .build(&event_loop)
        .unwrap();

    event_loop.set_control_flow(ControlFlow::Wait);

    // pixel buffer
    let size = window.inner_size();
    let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
    let mut pixels = Pixels::new(size.width, size.height, surface_texture).unwrap();

    let _ = event_loop.run(move |event, elwt| {

        match event {
            Event::WindowEvent { 
                window_id: _,
                event: WindowEvent::CloseRequested
            } => {
                elwt.exit();
            },
            Event::WindowEvent { 
                window_id: _,
                event: WindowEvent::Resized(new_size),
            } => {
                let _ = pixels.resize_surface(new_size.width, new_size.height);
            },
            Event::WindowEvent {
                window_id: _,
                event: WindowEvent::RedrawRequested,
            } => {

                let frame = pixels.frame_mut();

                for pixel in frame.chunks_exact_mut(4) {
                    pixel.copy_from_slice(&[0,0,0,255]); // each pixel set to black with full opacity
                }

                pixels.render().unwrap();
            },
            _ => {}
        }
    });
}
