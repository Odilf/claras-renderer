use nalgebra::{ Vector3, vector };
use nannou::prelude::*;

pub mod camera;
use camera::Camera;

type Polygon = Vec<Vector3<f64>>;

pub enum Renderable {
	Point(Vector3<f64>),
	Line { start: Vector3<f64>, end: Vector3<f64> },
	Polygon(Polygon),
	Model(Vec<Polygon>),
}

type Queue = Vec<Renderable>;

impl Renderable {
	fn distance(&self, camera: &Camera) -> f64 {
		match self {
			Renderable::Point(point) =>  camera.projection_distance(point),
			Renderable::Line { start, end } => 2.0,
			Renderable::Polygon(polygon) => {
				let length = polygon.len() as f64;
				let sum: f64 = polygon.iter().map(|v| Self::distance(&Self::Point(*v), camera)).sum();
				sum/length
			},
			Renderable::Model(model) => {
				let length = model.len() as f64;
				let sum: f64 = model.iter().map(|poly| Self::distance(&Self::Polygon( poly.to_vec() ), camera)).sum();
				sum/length
			}
		}
	}
}

pub struct Model {
	camera: Camera,
	queue: Vec<Renderable>,
}

pub fn model(camera: Camera, queue: Queue) -> Model {
	Model { camera, queue }
}

fn draw_object(camera: &Camera, object: &Renderable, draw: &nannou::draw::Draw) {
	match object {
		Renderable::Point(position) => { draw.ellipse().radius(5.0).color(GREEN).x(position.x as f32).y(position.y as f32); },
		_ => ()
	};

	// draw.ellipse().radius(5.0);
}

// fn draw_queue(camera: &Camera, queue: &Vec<Renderable>) -> Vec<RenderTarget> {
// 	queue.iter().map(|object| draw(camera, object)).collect()
// }

pub fn render(app: &App, model: &Model, frame: Frame) {
	let draw = app.draw();
	let cam = &model.camera;

	draw.background().color(TOMATO);

	for object in &model.queue {
		draw_object(cam, object, &draw)
	}

	draw.to_frame(app, &frame).unwrap();
}