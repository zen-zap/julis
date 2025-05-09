/// function to determine the number of iterations it takes for a point to escape a certain
/// threshold
pub fn mandelbrot(
	c_re: f64,
	c_im: f64,
	max_iter: u32,
) -> u32 {
	let mut z_re = 0.0;
	let mut z_im = 0.0;
	let mut iter = 0;

	while z_re * z_re + z_im * z_im <= 4.0 && iter < max_iter {
		
        let temp_z_re = z_re * z_re - z_im * z_im + c_re;

		z_im = 2.0 * z_re * z_im + c_im;

		z_re = temp_z_re;

		iter += 1;
	}

	iter
}

/// for pixel mappings
pub fn pixel_to_complex(
	x: u32,
	y: u32,
	width: u32,
	height: u32,
	zoom: f64,
	offset_x: f64,
	offset_y: f64,
) -> (f64, f64) {

    let aspect_ratio = width as f64 / height as f64;
    let scale = 3.0 / zoom;

    let scaled_x = (x as f64 / width as f64 - 0.5) * scale * aspect_ratio + offset_x;
    let scaled_y = (y as f64 / height as f64 - 0.5) * scale + offset_y;

    (scaled_x, scaled_y)
}


