use nalgebra::{Vector2, Vector3};

mod camera;
mod objects;
mod renderer;
mod scene;

pub use camera::Camera;
pub use objects::{shape, Color, Tri};
pub use scene::{Scene, Viewport};

pub type Vec3 = Vector3<f32>;
pub type Vec2 = Vector2<f32>;
