use nalgebra::{ Vector3, vector };
use nannou::prelude::*;

mod renderer;
use renderer::{ Camera, Polygon, Model as Model3D };

struct Model {
	camera: Camera,
	models: Vec<Model3D>,
	points: Vec<Vector3<f64>>,
}

fn main() {
	nannou::app(model)
		.loop_mode(LoopMode::Wait)
		.event(event)
		.simple_window(view)
        .run();
}

fn model(_app: &App) -> Model {

	let camera = renderer::Camera::from(
		[1.0, 1.0], 1.0,
		vector![0.0, 0.0, 0.0],
	);

	let a = vec![
		vector![0.0, 0.0, 0.0],
		vector![0.0, 2.0, 0.0],
		vector![0.0, 2.0, 1.0],
		vector![0.0, 1.0, 1.0],
		vector![0.0, 1.0, 3.0],
		vector![0.0, 0.0, 3.0],
	];

	let c = vec![
		vector![0.0, 0.0, 0.0],
		vector![1.0, 0.0, 0.0],
		vector![1.0, 0.0, 3.0],
		vector![0.0, 0.0, 3.0],
	];

	let ground = vec![
		vector![0.0, 0.0, 0.0],
		vector![10.0, 0.0, 0.0],
		vector![10.0, 10.0, 0.0],
		vector![0.0, 10.0, 0.0],
	];

	let x_plane = vec![
		vector![0.0, 0.0, 0.0],
		vector![0.0, 10.0, 0.0],
		vector![0.0, 10.0, 10.0],
		vector![0.0, 0.0, 10.0],
	];

	let y_plane = vec![
		vector![0.0, 0.0, 0.0],
		vector![10.0, 0.0, 0.0],
		vector![10.0, 0.0, 10.0],
		vector![0.0, 0.0, 10.0],
	];

    Model {
		camera,
		models: vec![
			vec![a, c],
			// vec![ground, y_plane, x_plane],
		],
		points: vec![
			vector![0.0, 0.0, 0.0],
			vector![1.0, 0.0, 0.0],
			vector![0.0, 1.0, 0.0],
			vector![0.0, 0.0, 1.0],
		]
	}
}

fn event(_app: &App, model: &mut Model, event: Event) {
	match event {
        Event::WindowEvent {
			id: _,
			simple: event,
		} => {
			match event {
				Some(event) => window_event(_app, model, event),
				None => ()
			}
		}
		_ => ()
    }
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
	let mut cam = &mut model.camera;
	match key {
		Key::Up => cam.zoom *= 2.0,
		Key::Down => cam.zoom *= 0.5,
		Key::Left => cam.angles[0] += 0.1,
		Key::Right => cam.angles[0] -= 0.1,

		Key::Q => cam.angles[1] += 0.1,
		Key::R => cam.angles[1] -= 0.1,
		_ => ()

	}

	cam.update();
}

fn window_event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(key) => key_pressed(_app, model, key),
		_ => ()
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
	// get canvas to draw on
    let draw = app.draw();

    // set background to blue
    draw.background().color(BLACK);

	// let polies = model.camera.project_model(&model.models[0]);
	for model3d in &model.models {
		let polies = model.camera.project_model(model3d);

		for poly in polies {
			let points = poly.points.iter().map(|p| {
				(pt2(p.x as f32, p.y as f32), rgb(poly.brightness, poly.brightness, poly.brightness))
			});
			
			draw.polygon().points_colored(points);
		}
	}

	for point in &model.points {
		let point = model.camera.orthogonal_projetion(point).0;
		let point = model.camera.screen_coords(&point);
		draw.ellipse().xy(pt2(point.x as f32, point.y as f32)).radius(1.0);
	}

	// let points = model.camera.orthogonal_projetion(point)

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}