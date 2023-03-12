use nalgebra::Rotation3;

use super::{Vec2, Vec3};

#[derive(Debug)]
pub struct ProjectedVec {
    pub position: Vec2,
    pub distance: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    position: Vec3,
    normal: Vec3,
    up: Vec3,
}

impl Camera {
    pub fn new(position: Vec3, normal: Vec3, up: Vec3) -> Camera {
        Camera {
            position,
            normal,
            up,
        }
    }

    pub fn upright(position: Vec3, normal: Vec3) -> Camera {
        let up = Vec3::new(0.0, 1.0, 0.0);
        Camera::new(position, normal, up)
    }

    fn side(&self) -> Vec3 {
        self.normal.cross(&self.up).normalize()
    }

    // TODO: Optimize with caching, probably
    pub fn project(&self, point: Vec3) -> ProjectedVec {
        let delta = point - self.position;

        // let factor = 1.0 / delta.dot(&self.normal);
        // let delta = delta * factor;

        let delta = delta.normalize();

        let x = delta.dot(&self.side());
        let y = delta.dot(&self.up);

        ProjectedVec {
            position: Vec2::new(x, y),
            distance: delta.dot(&self.normal),
        }
    }

    pub fn translate(&mut self, delta: Vec3) {
        self.position += delta;
    }

    pub fn plane_translate(&mut self, delta: Vec2) {
        self.position += self.normal * delta.x + self.side() * delta.y;
    }

    pub fn rotate(&mut self, angle: f32) {
        let rotation = Rotation3::new(&self.up * angle);
        self.normal = rotation * self.normal;
        self.up = rotation * self.up;
    }
}
