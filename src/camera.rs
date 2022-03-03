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
	pub transform: Transform,
	screen_base: Base,
}

impl Directable for Camera {
	fn normal(&self) -> Vector3<f64> { self.transform.normal() }
}

impl Camera {
	fn projection_distance(&self, vector: &Vector3<f64>) -> f64 {
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

	fn get_screen_base(transform: Transform) {
		
	}
}

// type Poly3D = Vec<Vector3<f64>>;
// pub type Model = Vec<Poly3D>;

// impl Camera {

// 	fn screen_j(normal: &Vector3<f64>) -> Vector3<f64> {
// 		let n = normal;

// 		if n.x == 0.0 && n.y == 0.0 {
// 			return AXES[1];
// 		}

// 		let d = n.x.powf(2.0) + n.y.powf(2.0);
// 		let d = d.sqrt();
// 		vector!(
// 			-n.x * n.z / d,
// 			-n.y * n.z / d,
// 			d
// 		)
// 	}

// 	fn screen_i(normal: &Vector3<f64>, screen_j: &Vector3<f64>) -> Vector3<f64> {
// 		screen_j.cross(&normal)
// 	}

// 	pub fn update(&mut self) {
// 		self.j = Self::screen_j(&self.normal);
// 		self.i = Self::screen_i(&self.normal, &self.j);
// 		self.normal = Self::get_normal(self.angles, self.zoom)
// 	}

// fn projection_distance(&self, vector: &Vector3<f64>) -> f64 {
// 	let n = self.normal;
// 	let p = self.position;

// 	let top = n.x * (vector.x - p.x) +
// 			  n.y * (vector.y - p.y) +
// 			  n.z * (vector.z - p.z);
				

	
// 	let bottom = n[0].powf(2.0) + n[1].powf(2.0) + n[2].powf(2.0);
// 	top/bottom
// }

// 	pub fn orthogonal_projetion(&self, vector: &Vector3<f64>) -> (Vector3<f64>, f64) {
// 		let dist = self.projection_distance(vector);
// 		(vector + &self.normal * dist, dist)
// 	}

// 	pub fn screen_coords(&self, vector: &Vector3<f64>) -> Vector2<f64> {
// 		let dif = vector - self.position;
// 		let i = self.i;
// 		let j = self.j;

// 		// if i.x == 0.0 {
// 		// 	let y = j.x / dif.x;
// 		// 	let x = (dif.y - y*j.y) / i.y;
// 		// 	return vector!(x, y);
// 		// } else if i.y == 0.0 {
// 		// 	let x = j.y / dif.y;
// 		// 	let y = (dif.x - x*j.x) / i.x;
// 		// 	return vector!(x, y);
// 		// } else if j.x == 0.0 {
// 		// 	let x = i.x / dif.x;
// 		// 	let y = (dif.y - x*j.y) / i.y;
// 		// 	return vector!(x, y)
// 		// } else if j.y == 0.0 {
// 		// 	let x = dif.y / i.y;
// 		// 	let y = (dif.x - x*j.x) / i.x;
// 		// 	return vector!(x, y);
// 		// }

// 		if i.y == 0.0 || i.x == 0.0 || j.x == 0.0 || j.y == 0.0 {
// 			panic!("Screen coordinates are 0 and you can't simply multiply. \n i: {}", i)
// 		}

// 		let x = ( j.x * dif.x - j.y * dif.y ) / (i.x * j.x - i.y * j.y);
// 		// dif = x*i + y*j

// 		let y = (dif.x - x*i.x) / j.x;
// 		// let y = ( i.y * dif.x - i.y * dif.y ) / (j.x * i.y - j.y * i.x);

// 		vector!(x, y)
// 	}

// 	pub fn from(angles: [f64; 2], zoom: f64, position: Vector3<f64>) -> Self {
		
// 		let normal = Self::get_normal(angles, zoom);

// 		let j = Camera::screen_j(&normal);
// 		let i = Camera::screen_i(&normal, &j);

// 		let i = Camera::screen_coords()

// 		Camera { position, zoom, angles, normal, i, j }
// 	}

// 	pub fn project_polygon(&self, points: &Poly3D) -> Polygon { 
// 		let normal = (points[1] - points[0]).cross(& (points[2] - points[0]));
// 		let brightness = self.normal.normalize().dot(&normal.normalize());

// 		if brightness.is_nan() {
// 			panic!("Brigtness is nan. calculated with n: {} p0: {} p1: {}", normal, points[0], points[1])
// 		}

// 		let length = points.len();

// 		let mut projected_points: Vec<Vector2<f64>> = Vec::with_capacity(length);
// 		let mut distances: Vec<f64> = Vec::with_capacity(length);

// 		for point in points {
// 			let (projection, distance) = self.orthogonal_projetion(point);
// 			let coords = self.screen_coords(&projection);

// 			projected_points.push(coords);
// 			distances.push(distance);
// 		}
		
// 		let distance: f64 = distances.iter().sum::<f64>() / length as f64;

// 		Polygon {
// 			points: projected_points,
// 			brightness,
// 			distance,
// 		}
// 	}

// 	pub fn project_model(&self, model: &Model) -> Vec<Polygon> {
// 		model.iter().map(|poly| self.project_polygon(poly)).collect()
// 	}
// }



// #[derive(Debug)]
// pub struct Polygon {
// 	pub points: Vec<Vector2<f64>>,
// 	pub distance: f64,
// 	pub brightness: f64,
// }