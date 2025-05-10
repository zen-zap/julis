use pixels::{Pixels, SurfaceTexture};
use std::rc::Rc;
use winit::{
	dpi::LogicalSize,
	event::{Event, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
	window::WindowBuilder,
};

mod mandel;

fn main() {
	let event_loop = EventLoop::new().unwrap();

	let window = WindowBuilder::new()
		.with_title("Mandelbrot Set")
		.with_inner_size(LogicalSize::new(800.0, 600.0))
		.build(&event_loop)
		.expect("Failed to build the window -- OsError");

	let window = Rc::new(window);

	event_loop.set_control_flow(ControlFlow::Wait);

	// pixel buffer
	let size = window.inner_size();
	let surface_texture = SurfaceTexture::new(size.width, size.height, &*window);
	let mut pixels = Pixels::new(size.width, size.height, surface_texture)
		.expect("Pixels Error? maybe something to do with surface_texture");

	let window_ref = Rc::clone(&window);

	// to setup zooming thingy
	let zoom_factor = 1.17;
	let mut zoom = 1.0;
    let mut offset_x = -0.5;
    let mut offset_y = 0.0;
	// to make the zoom focused on the mouse
	let mut mouse_pos = (0.0_f64, 0.0_f64);

	let _ = event_loop.run(move |event, elwt| {
		match event {
			Event::WindowEvent { window_id: _, event: WindowEvent::CloseRequested } => {
				elwt.exit();
			},
			Event::WindowEvent { window_id: _, event: WindowEvent::Resized(new_size) } => {
				let _ = pixels.resize_surface(new_size.width, new_size.height).expect("Error while resizing pixel buffer");
				window_ref.request_redraw(); // yo gotta redraw after a resize
			},
			Event::WindowEvent { window_id: _, event: WindowEvent::RedrawRequested } => {

				let curr_size = window_ref.inner_size();
				let width = curr_size.width;
				let height = curr_size.height;

				if width != size.width || height != size.height {
					let _ = pixels.resize_buffer(width, height).expect("Error while resizing pixel buffer");
				}

				let frame = pixels.frame_mut();
				let max_iter = 100;

				for y in 0..height {
					for x in 0..width {
						let (c_re, c_im) =
							mandel::pixel_to_complex(x, y, width, height, zoom, offset_x, offset_y);
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
			Event::WindowEvent {
				window_id: _,
				event: WindowEvent::MouseWheel { device_id: _, delta, phase: _ },
			} => {

                let curr_window_size = window_ref.inner_size();
                let width = curr_window_size.width;
                let height = curr_window_size.height;

				let (pre_re, pre_im) = mandel::pixel_to_complex(
					mouse_pos.0 as u32,
					mouse_pos.1 as u32,
					width,
					height,
					zoom,
					offset_x,
					offset_y,
				);

                // "delta" tells us how much we moved
				let scroll_amount = match delta {
                    // for mousewheel
					winit::event::MouseScrollDelta::LineDelta(_, y) => y as f64,
                    // for trackpads
					winit::event::MouseScrollDelta::PixelDelta(pos) => pos.y,
				};

                // scale the zoom to have some smooth slow zooming
				if scroll_amount > 0.0 {
					zoom *= zoom_factor;
				} else {
					zoom /= zoom_factor;
				}

				//zoom = zoom.clamp(0.1, 999999.0); // set limits for zoom values to avoid inverting or
				// excessive zooming

                let aspect = width as f64 / height as f64;
                let scale = 3.0 / zoom;
                offset_x = pre_re - (mouse_pos.0 / width as f64 - 0.5) * scale * aspect;
                offset_y = pre_im - (mouse_pos.1 / height as f64 - 0.5) * scale;

                window_ref.request_redraw();
			},
			Event::WindowEvent {
				window_id: _,
				event: WindowEvent::CursorMoved { device_id: _, position },
            } => {
                // raw cursor position
				mouse_pos = (position.x as f64, position.y as f64);
			},
			_ => {
				// initial render
				window_ref.request_redraw();
			},
		}
	});
}
