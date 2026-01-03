use macroquad::prelude::*;
use std::f32::consts::PI;


const THRESHOLD: f32 = 0.999; // Only visible when sine > 0.95
static SIN_60: f32 = 0.866;
const PHASE_SPEED: f32 = 0.01; // Adjust this to control how fast the wave moves
struct Hexagon {
    x: f32,
    y: f32,
    radius: f32,
    color: Color,
    next_color: Color,
    transition_progress: f32,
    phase_offset: f32, // Unique phase offset for this hexagon
}

impl Hexagon {
    fn new(x: f32, y: f32, radius: f32) -> Self {
	Self {
	    x,
	    y,
	    radius,
	    color: random_color(),
	    next_color: random_color(),
	    transition_progress: 0.0,
	    phase_offset: rand::gen_range(0.0, PI * 2.), // Random phase 0 to 2π
	}
    }

    fn update(&mut self, dt: f32) {
	// Handle color transition
	self.transition_progress += dt * 0.3;

	if self.transition_progress >= 1.0 {
	    self.color = self.next_color;
	    self.next_color = random_color();
	    self.transition_progress = 0.0;
	}
    }

    fn draw(&self, time: f32) {
	// Calculate opacity based on time and phase offset
	// Use sine wave for smooth pulsing: 0.0 to 1.0
	let raw_value = (time * PHASE_SPEED + self.phase_offset).sin();

	let opacity = if raw_value > THRESHOLD {
	    ((raw_value - THRESHOLD) / (1.0 - THRESHOLD)).powf(2.0) // Smooth fade near peak
	} else {
	    0.0
	};
	if opacity <= 0.01 {
	    return; // Don't draw nearly invisible hexagons
	}

	// Interpolate between current and next color
	let current_color = Color::new(
	    self.color.r + (self.next_color.r - self.color.r) * self.transition_progress,
	    self.color.g + (self.next_color.g - self.color.g) * self.transition_progress,
	    self.color.b + (self.next_color.b - self.color.b) * self.transition_progress,
	    opacity,
	);

	// draw_hexagon(self.x, self.y, self.radius, 0.0, true, current_color);
	draw_heart(self.x, self.y, self.radius, current_color);
    }
}

fn random_color() -> Color {
    Color::new(
	rand::gen_range(0.0, 1.0),
	rand::gen_range(0.0, 1.0),
	rand::gen_range(0.0, 1.0),
	1.0,
    )
}

fn draw_heart(x: f32, y: f32, size: f32, color: Color) {
    // Heart shape using parametric equations
    // We'll draw it as a series of triangles from the center
    let segments = 100;
    let mut points = Vec::new();

    for i in 0..=segments {
	let t = (i as f32 / segments as f32) * 2.0 * std::f32::consts::PI;

	// Parametric heart equation

	let heart_x = 16.0 * t.sin().powi(3);
	let heart_y = -(13.0 * t.cos() - 5.0 * (2.0 * t).cos() - 2.0 * (3.0 * t).cos() - (4.0 * t).cos());

	// Scale and translate
	let scale = size / 20.0;
	points.push(Vec2::new(
	    x + heart_x * scale,
	    y + heart_y * scale,
	));
    }

    // Draw heart as triangles from center
    for i in 0..segments {
	draw_triangle(
	    Vec2::new(x, y),
	    points[i],
	    points[i + 1],
	    color,
	);
    }
}
fn draw_hexagon(x: f32, y: f32, radius: f32, rotation: f32, filled: bool, color: Color) {
    let mut points = Vec::new();
    for i in 0..6 {
	let angle = rotation + (i as f32 * 60.0).to_radians();
	points.push(Vec2::new(
	    x + radius * angle.cos(),
	    y + radius * angle.sin(),
	));
    }

    if filled {
	// Draw filled hexagon as triangles from center
	for i in 0..6 {
	    let next = (i + 1) % 6;
	    draw_triangle(
		Vec2::new(x, y),
		points[i],
		points[next],
		color,
	    );
	}
    }
}

fn create_hexgrid(hex_radius: f32, width: f32, height: f32) -> Vec<Vec2>{
    let hex_height = SIN_60 * hex_radius * 2.; // sin(60°) for hexagon height
    let num_cols = (width / hex_radius * 2.) as i32 + 2;
    let num_rows = (height / hex_radius * 2.) as i32 + 2;
    let mut hexagons = Vec::new();
    for row in 0..num_rows {
	for col in 0..num_cols {
	    let x = col as f32 * hex_radius * 3.;
	    let y = row as f32 * hex_height;
	    hexagons.push(Vec2::new(x, y));
	    let x2 = x + hex_radius * 1.5;
	    let y2 = y + hex_height * 0.5;
	    hexagons.push(Vec2::new(x2, y2));
	}
    }
    return hexagons
}

#[macroquad::main("Hexagon Screensaver")]
async fn main() {
    let hex_radius = 40.0;

    let mut hexagons = Vec::new();
    for cell in create_hexgrid(hex_radius, screen_width(), screen_height()) {
	hexagons.push(Hexagon::new(cell.x, cell.y, hex_radius));
    }

    let mut time = 0.0;
    loop {
	clear_background(BLACK);

	let dt = get_frame_time();
	time += dt;

	// Update and draw all hexagons
	for hex in &mut hexagons {
	    hex.update(dt);
	    hex.draw(time);
	}

	next_frame().await
    }
}
