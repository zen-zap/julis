use pixels::{Pixels, SurfaceTexture};
use winit::{
	dpi::LogicalSize,
	event::{Event, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
	window::{WindowBuilder},
};

mod mandel;

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
                let width = size.width;
                let height = size.height;

                let zoom = 1.0;
                let offset_x = -0.5;
                let offset_y = 0.0;
                let max_iter = 100;

                for y in 0..height {
                    for x in 0..width {
                        let (c_re, c_im) = mandel::pixel_to_complex(
                            x,
                            y,
                            width,
                            height,
                            zoom,
                            offset_x,
                            offset_y,
                        );
                        let iter = mandel::mandelbrot(c_re, c_im, max_iter);

                        let pixel_index = ((y * width + x) * 4) as usize;
                        let color = if iter == max_iter {
                            [0, 0, 0, 255] // Black for points inside the set
                        } else {
                            let t = iter as f64 / max_iter as f64;
                            let r = (9.0 * (1.0 - t) * t * t * t * 255.0) as u8;
                            let g = (15.0 * (1.0 - t) * (1.0 - t) * t * t * 255.0) as u8;
                            let b = (8.5 * (1.0 - t) * (1.0 - t) * (1.0 - t) * t * 255.0) as u8;
                            [r, g, b, 255]
                        };

                        frame[pixel_index..pixel_index + 4].copy_from_slice(&color);
                    }
                }

                pixels.render().unwrap();
            },
            _ => {}
        }
    });
}
