use nalgebra::{ Vector3, Vector2, vector };

const AXES: [Vector3<f64>; 3] = [
	vector!(1.0, 0.0, 0.0),
	vector!(0.0, 1.0, 0.0),
	vector!(0.0, 0.0, 1.0),
];

#[derive(Debug)]
struct Polar { 
	angles: [f64; 2],
	zoom: f64,
}

#[derive(Debug)]
enum Direction {
	Linear(Vector3<f64>),
	Polar(Polar)
}

#[derive(Debug)]
struct Transform {
	direction: Direction,
	position: Vector3<f64>,
}

impl Transform {
	fn default() -> Transform {
		Transform {
			direction: Direction::Linear(vector![1.0, 0.0, 0.0]),
			position: vector![0.0, 0.0, 0.0],
		}
	}
}

trait Directable {
	fn normal(&self) -> Vector3<f64>;
}

impl Directable for Polar {
	fn normal(&self) -> Vector3<f64> {
		let angles = self.angles;
		let r = 1.0/ self.zoom;
		let normal = vector![
			angles[1].cos() * angles[0].cos(),
			angles[1].cos() * angles[0].sin(),
			angles[1].sin()
		] * r;

		normal
	}
}

impl Directable for Direction {
	fn normal(&self) -> Vector3<f64> {
		match self {
			Direction::Linear(normal) => *normal,
			Direction::Polar(polar) => polar.normal(),
		}
	}
}

impl Directable for Transform {
	fn normal(&self) -> Vector3<f64> { self.direction.normal() }
}

#[derive(Debug)]
struct Base {
	i: Vector2<f64>,
	j: Vector2<f64>,
	k: Vector2<f64>,
}

#[derive(Debug)]
pub struct Camera {
	transform: Transform,
	screen_base: Option<Base>,
}

impl Directable for Camera {
	fn normal(&self) -> Vector3<f64> { self.transform.normal() }
}

impl Camera {
	pub fn default() -> Self {
		Camera {
			transform: Transform::default(),
			screen_base: None,
		}
	}

	pub fn projection_distance(&self, vector: &Vector3<f64>) -> f64 {
		let n = self.transform.normal();
		let p = self.transform.position;

		let top = n.x * (vector.x - p.x) +
				  n.y * (vector.y - p.y) +
				  n.z * (vector.z - p.z);
		
		let bottom = n.x.powf(2.0) + n.y.powf(2.0) + n.z.powf(2.0);
		top/bottom
	}

	fn project_point(&self, point: &Vector3<f64>, distance: f64) -> Vector3<f64> {
		point + self.transform.normal() * distance
	}

	fn screen_j(normal: &Vector3<f64>) -> Vector3<f64> {
		let n = normal;

		if n.x == 0.0 && n.y == 0.0 {
			return AXES[1];
		}

		let d = n.x.powf(2.0) + n.y.powf(2.0);
		let d = d.sqrt();
		vector!(
			-n.x * n.z / d,
			-n.y * n.z / d,
			d
		)
	}

	fn screen_i(normal: &Vector3<f64>, screen_j: &Vector3<f64>) -> Vector3<f64> {
		screen_j.cross(&normal)
	}

	fn screen_coordinates(&self, point: &Vector3<f64>, i: &Vector3<f64>, j: &Vector3<f64>) -> Vector2<f64> {
		let dif = point - self.transform.position;
		let x = (dif.x * j.y - dif.y * j.x) / (i.x * j.y - i.y * j.x);
		let y = (dif.x * i.y - dif.y * i.x) / (j.x * i.y - j.y * i.x); 

		vector![x, y]
	}

	fn get_screen_base(&mut self) {
		let mut screen_base: Vec<Vector2<f64>> = Vec::with_capacity(3);

		let j = Camera::screen_j(&self.normal());
		let i = Camera::screen_i(&self.normal(), &j);

		for axis in AXES {
			let lambda = self.projection_distance(&axis);
			let projection = self.project_point(&axis, lambda);
			let screen_coords = self.screen_coordinates(&projection, &i, &j);

			screen_base.push(screen_coords);
		}

		self.screen_base = Some(Base {
			i: screen_base[0],
			j: screen_base[1],
			k: screen_base[2],
		});
	}
}