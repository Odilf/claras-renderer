use nalgebra::vector;
use nannou::prelude::*;

mod renderer;
use renderer::camera::Camera;
use renderer::{ Renderable, Model };

fn main() {
	nannou::app(model).simple_window(view).run();
}

fn view(app: &App, model: &Model, frame: Frame) {
	renderer::render(app, model, frame)
}

fn model(_app: &App) -> renderer::Model {
	let queue = vec![
		Renderable::Point(vector![10.0, 0.0, 0.0])
	];

	let camera = Camera::default();

	renderer::model(camera, queue)
}