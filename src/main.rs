use macroquad::prelude::*;
use std::f32::consts::PI;
use std::env;
use std::fs;
use std::path::PathBuf;

static SIN_60: f32 = 0.866;

#[derive(Clone, Copy, PartialEq, Default)]
enum ShapeType {
    #[default]
    Hexagon,
    Heart,
}

#[derive(Clone, Copy, Default)]
struct Config {
    shape: ShapeType,
    threshold: f32,
}

impl Config {
    fn load() -> Self {
	let config_path = Self::get_config_path();
	if let Ok(content) = fs::read_to_string(&config_path) {
	    let lines: Vec<&str> = content.lines().collect();
	    let mut config = Config::default();

	    for line in lines {
		let parts: Vec<&str> = line.split('=').collect();
		if parts.len() == 2 {
		    let key = parts[0].trim();
		    let value = parts[1].trim();

		    match key {
			"shape" => {
			    config.shape = if value == "heart" {
				ShapeType::Heart
			    } else {
				ShapeType::Hexagon
			    };
			}
			"threshold" => {
			    if let Ok(val) = value.parse::<f32>() {
				config.threshold = val.clamp(0.0, 1.0);
			    }
			}
			_ => {}
		    }
		}
	    }
	    config
	} else {
	    Config::default()
	}
    }

    fn save(&self) {
	let config_path = Self::get_config_path();
	let shape_str = match self.shape {
	    ShapeType::Hexagon => "hexagon",
	    ShapeType::Heart => "heart",
	};
	let content = format!(
	    "shape={}\nthreshold={}\n",
	    shape_str, self.threshold
	);
	let _ = fs::write(&config_path, content);
    }

    fn get_config_path() -> PathBuf {
	if let Ok(appdata) = env::var("APPDATA") {
	    let mut path = PathBuf::from(appdata);
	    path.push("HeartScreensaver");
	    let _ = fs::create_dir_all(&path);
	    path.push("config.txt");
	    path
	} else {
	    PathBuf::from("screensaver_config.txt")
	}
    }
}
struct Shape {
    x: f32,
    y: f32,
    radius: f32,
    color: Color,
    next_color: Color,
    transition_progress: f32,
    phase_offset: f32,
}

impl Shape {
    fn new(x: f32, y: f32, radius: f32) -> Self {
	Self {
	    x,
	    y,
	    radius,
	    color: random_color(),
	    next_color: random_color(),
	    transition_progress: 0.0,
	    phase_offset: rand::gen_range(0.0, 2. * PI),
	}
    }

    fn update(&mut self, dt: f32, _time: f32) {
	self.transition_progress += dt * 0.3;

	if self.transition_progress >= 1.0 {
	    self.color = self.next_color;
	    self.next_color = random_color();
	    self.transition_progress = 0.0;
	}
    }
    fn draw(&self, time: f32, shape_type: ShapeType, threshold: f32) {
	let phase_speed = (1. - threshold) * 10.;
	let raw_value = (time * phase_speed + self.phase_offset).sin();

	let opacity = if raw_value > threshold {
	    ((raw_value - threshold) / (1.0 - threshold)).powf(2.0)
	} else {
	    0.0
	};

	if opacity <= 0.01 {
	    return;
	}

	let current_color = Color::new(
	    self.color.r + (self.next_color.r - self.color.r) * self.transition_progress,
	    self.color.g + (self.next_color.g - self.color.g) * self.transition_progress,
	    self.color.b + (self.next_color.b - self.color.b) * self.transition_progress,
	    opacity,
	);

	match shape_type {
	    ShapeType::Hexagon => draw_hexagon(self.x, self.y, self.radius, 0.0, true, current_color),
	    ShapeType::Heart => draw_heart(self.x, self.y, self.radius, current_color),
	}
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

fn window_conf() -> Conf {
    let args: Vec<String> = env::args().collect();

    let (fullscreen, width, height) = if args.len() > 1 {
	let arg = &args[1];
	let arg_lower = arg.to_lowercase();

	if arg_lower.starts_with("/c") || arg_lower.starts_with("-c") {
	    // Configuration mode - handle both /c and /c:hwnd formats
	    (false, 500, 350)
	} else if arg_lower.starts_with("/s") || arg_lower.starts_with("-s") {
	    // Screensaver mode
	    (true, 0, 0)
	} else if arg_lower.starts_with("/p") || arg_lower.starts_with("-p") {
	    // Preview mode - just exit for now
	    std::process::exit(0);
	} else {
	    // Unknown or no argument - windowed mode
	    (false, 800, 600)
	}
    } else {
	// No arguments - windowed mode for testing
	(false, 800, 600)
    };

    Conf {
	window_title: "Heart Screensaver".to_owned(),
	fullscreen,
	window_width: width,
	window_height: height,
	..Default::default()
    }
}


#[macroquad::main(window_conf)]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let is_config_mode = args.len() > 1 && (args[1].to_lowercase() == "/c" || args[1].to_lowercase() == "-c");

    if is_config_mode {
	run_config_ui().await;
    } else {
	run_screensaver().await;
    }
}

async fn run_config_ui() {
    let mut config = Config::load();
    let mut selected_hexagon = config.shape == ShapeType::Hexagon;
    let mut selected_heart = config.shape == ShapeType::Heart;
    let mut threshold_slider_dragging = false;

    loop {
	clear_background(Color::from_rgba(240, 240, 240, 255));

	// Title
	draw_text("Screensaver Configuration", 20.0, 40.0, 30.0, BLACK);

	// Shape selection
	draw_text("Choose Shape:", 20.0, 90.0, 25.0, BLACK);

	// Hexagon radio button
	let hexagon_box = Rect::new(40.0, 110.0, 20.0, 20.0);
	draw_rectangle(hexagon_box.x, hexagon_box.y, hexagon_box.w, hexagon_box.h, WHITE);
	draw_rectangle_lines(hexagon_box.x, hexagon_box.y, hexagon_box.w, hexagon_box.h, 2.0, BLACK);
	if selected_hexagon {
	    draw_rectangle(hexagon_box.x + 4.0, hexagon_box.y + 4.0, 12.0, 12.0, DARKBLUE);
	}
	draw_text("Hexagons", 70.0, 128.0, 20.0, BLACK);

	// Heart radio button
	let heart_box = Rect::new(40.0, 150.0, 20.0, 20.0);
	draw_rectangle(heart_box.x, heart_box.y, heart_box.w, heart_box.h, WHITE);
	draw_rectangle_lines(heart_box.x, heart_box.y, heart_box.w, heart_box.h, 2.0, BLACK);
	if selected_heart {
	    draw_rectangle(heart_box.x + 4.0, heart_box.y + 4.0, 12.0, 12.0, DARKBLUE);
	}
	draw_text("Hearts", 70.0, 168.0, 20.0, BLACK);

	// Density slider (threshold - inverted for UX)
	draw_text("Density (fewer <- -> more):", 20.0, 220.0, 20.0, BLACK);
	let density_slider_rect = Rect::new(40.0, 240.0, 420.0, 10.0);
	draw_rectangle(density_slider_rect.x, density_slider_rect.y, density_slider_rect.w, density_slider_rect.h, LIGHTGRAY);

	// Convert threshold to density (invert: lower threshold = more shapes)
	let normalized_thresh = (config.threshold - 0.9) * 10.;
	let density = 1.0 - normalized_thresh;
	let density_handle_x = density_slider_rect.x + density * density_slider_rect.w;
	let density_handle = Rect::new(density_handle_x - 8.0, density_slider_rect.y - 5.0, 16.0, 20.0);
	draw_rectangle(density_handle.x, density_handle.y, density_handle.w, density_handle.h, DARKBLUE);

	let density_text = format!("{:.0}%", density * 100.0);
	draw_text(&density_text, 40.0, 275.0, 18.0, BLACK);


	// OK button
	let ok_button = Rect::new(200.0, 390.0, 100.0, 40.0);
	let mouse_pos = mouse_position();
	let is_hovering = ok_button.contains(Vec2::new(mouse_pos.0, mouse_pos.1));

	draw_rectangle(ok_button.x, ok_button.y, ok_button.w, ok_button.h,
		      if is_hovering { DARKGRAY } else { GRAY });
	draw_rectangle_lines(ok_button.x, ok_button.y, ok_button.w, ok_button.h, 2.0, BLACK);
	draw_text("OK", ok_button.x + 35.0, ok_button.y + 27.0, 25.0, WHITE);

	// Handle mouse input
	let mouse_down = is_mouse_button_down(MouseButton::Left);
	let mouse_clicked = is_mouse_button_pressed(MouseButton::Left);

	// Density slider interaction
	if mouse_clicked && density_handle.contains(Vec2::new(mouse_pos.0, mouse_pos.1)) {
	    threshold_slider_dragging = true;
	}
	if !mouse_down {
	    threshold_slider_dragging = false;
	}
	if threshold_slider_dragging {
	    let normalized = ((mouse_pos.0 - density_slider_rect.x) / density_slider_rect.w).clamp(0.0, 1.0);
	    let density_val = normalized;
	    config.threshold = 1.0 - (density_val / 10.); // Invert back to threshold
	    config.threshold = config.threshold.clamp(0.0, 1.);
	}


	// Radio button clicks
	if mouse_clicked {
	    if hexagon_box.contains(Vec2::new(mouse_pos.0, mouse_pos.1)) {
		selected_hexagon = true;
		selected_heart = false;
		config.shape = ShapeType::Hexagon;
	    } else if heart_box.contains(Vec2::new(mouse_pos.0, mouse_pos.1)) {
		selected_hexagon = false;
		selected_heart = true;
		config.shape = ShapeType::Heart;
	    } else if ok_button.contains(Vec2::new(mouse_pos.0, mouse_pos.1)) {
		config.save();
		break;
	    }
	}

	if is_key_pressed(KeyCode::Escape) {
	    break;
	}

	next_frame().await
    }
}

async fn run_screensaver() {
    let config = Config::load();
    let shape_radius = 40.0;

    let mut shapes = Vec::new();
    for cell in create_hexgrid(shape_radius, screen_width(), screen_height()) {
	shapes.push(Shape::new(cell.x, cell.y, shape_radius));
    }

    let mut time = 0.0;
    let mut mouse_moved = false;
    let mut last_mouse_pos = mouse_position();

    loop {
	clear_background(BLACK);

	let dt = get_frame_time();
	time += dt;

	let current_mouse_pos = mouse_position();
	if current_mouse_pos != last_mouse_pos {
	    if mouse_moved {
		break;
	    }
	    mouse_moved = true;
	    last_mouse_pos = current_mouse_pos;
	}

	if is_key_pressed(KeyCode::Escape) || is_mouse_button_pressed(MouseButton::Left) {
	    break;
	}

	for shape in &mut shapes {
	    shape.update(dt, time);
	    shape.draw(time, config.shape, config.threshold);
	}

	next_frame().await
    }
}
