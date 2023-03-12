use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::{renderer, Camera, Color, Tri, Vec3};

#[derive(Debug)]
pub struct Viewport {
    pub width: usize,
    pub height: usize,
}

impl Viewport {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

#[derive(Debug)]
pub struct Scene {
    tris: Vec<Tri<Vec3>>,
    pub viewport: Viewport,
    pub camera: Camera,
}

type Buffer = Vec<Vec<Color>>;

impl Scene {
    pub fn new(camera: Camera, viewport: Viewport) -> Self {
        Self {
            tris: Vec::new(),
            viewport,
            camera,
        }
    }

    fn render_buffer(&self) -> Buffer {
        let projected_objects: Vec<_> = self
            .tris
            .iter()
            .map(|tri| tri.projection(&self.camera))
            .collect();

        let background_color = Color::opaque(0.1, 0.0, 0.0);

        renderer::render_buffer(&projected_objects, background_color, &self.viewport)
    }

    pub fn render(&self) -> Vec<String> {
        let buffer = self.render_buffer();

        buffer
            .into_par_iter()
            .map(|row| {
                row.into_par_iter()
                    .map(|color| color.to_ansi())
                    .collect::<Vec<_>>()
                    .join("")
            })
            .collect()
    }

    pub fn add_shape<T: IntoTris>(mut self, shape: T) -> Self {
        self.tris.extend(shape.tris());
        self
    }
}

pub trait IntoTris {
    fn tris(self) -> Vec<Tri<Vec3>>;
}

impl IntoTris for Tri<Vec3> {
    fn tris(self) -> Vec<Tri<Vec3>> {
        vec![self]
    }
}

impl IntoTris for Vec<Tri<Vec3>> {
    fn tris(self) -> Vec<Tri<Vec3>> {
        self
    }
}
