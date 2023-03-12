use std::ops::Mul;

use crate::{camera::ProjectedVec, Camera, Vec3};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Color {
        Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a * rhs,
        }
    }
}

impl Color {
    // From https://stackoverflow.com/questions/30097953/ascii-art-sorting-an-array-of-ascii-characters-by-brightness-levels-c-c, array generated with Julia script (but should be a rust macro 🙄)
    const CHAR_OPACITY_SCALE: [char; 70] = [
        '.', '\'', '`', '^', '\"', ',', ':', ';', 'I', 'l', '!', 'i', '>', '<', '~', '+', '_', '-',
        '?', ']', '[', '}', '{', '1', ')', '(', '|', '\\', '/', 't', 'f', 'j', 'r', 'x', 'n', 'u',
        'v', 'c', 'z', 'X', 'Y', 'U', 'J', 'C', 'L', 'Q', '0', 'O', 'Z', 'm', 'w', 'q', 'p', 'd',
        'b', 'k', 'h', 'a', 'o', '*', '#', 'M', 'W', '&', '8', '%', 'B', '@', '$', '█',
    ];

    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { r, g, b, a }
    }

    pub fn opaque(r: f32, g: f32, b: f32) -> Color {
        Color::new(r, g, b, 1.0)
    }

    fn to_char(&self) -> char {
        let length = Self::CHAR_OPACITY_SCALE.len() - 1;
        let index = (self.a * length as f32).round() as usize;
        Self::CHAR_OPACITY_SCALE[index]
    }

    pub fn to_ansi(&self) -> String {
        let r = (self.r.clamp(0.0, 1.0) * 255.0).round() as u8;
        let g = (self.g.clamp(0.0, 1.0) * 255.0).round() as u8;
        let b = (self.b.clamp(0.0, 1.0) * 255.0).round() as u8;

        format!("\x1b[38;2;{r};{g};{b}m{}", self.to_char())
    }

    fn combine_channel(a: f32, b: f32, amount: f32) -> f32 {
        (a + b * amount).clamp(0.0, 1.0)
    }

    pub fn combine(&mut self, other: Color) {
        self.r = Self::combine_channel(self.r, other.r, other.a);
        self.g = Self::combine_channel(self.g, other.g, other.a);
        self.b = Self::combine_channel(self.b, other.b, other.a);
        self.a = Self::combine_channel(self.a, other.a, other.a);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Tri<T> {
    pub vertices: [T; 3],
}

impl<T> Tri<T> {
    pub fn new(vertices: [T; 3]) -> Tri<T> {
        Tri { vertices }
    }
}

impl Tri<Vec3> {
    pub fn projection(&self, camera: &Camera) -> Tri<ProjectedVec> {
        let projected_vertices = self.vertices.map(|v| camera.project(v));
        Tri { vertices: projected_vertices }
    }
}

pub mod shape {
    use crate::{Tri, Vec3};

    pub fn equilateral_triangle(center: Vec3, normal: Vec3) -> Tri<Vec3> {
        let mut vertices = [Vec3::new(0.0, 0.0, 0.0); 3];

        let normal = normal.normalize();
        let mut tangent = normal.cross(&Vec3::new(0.0, 0.0, 1.0)).normalize();
        let mut bitangent = tangent.cross(&normal).normalize();

        if normal.z < 0.0 {
            tangent = tangent * -1.0;
            bitangent = bitangent * -1.0;
        }

        let radius = 1.0;
        let height = (3.0_f32.sqrt() / 2.0) * radius;

        vertices[0] = center + tangent * radius + bitangent * height;
        vertices[1] = center + tangent * radius - bitangent * height;
        vertices[2] = center - tangent * radius;

        Tri { vertices }
    }

    pub fn square(center: Vec3, normal: Vec3, up: Vec3) -> Vec<Tri<Vec3>> {
        let mut vertices = [Vec3::new(0.0, 0.0, 0.0); 4];

        let normal = normal.normalize();
        let up = up.normalize();
        let side = normal.cross(&up).normalize();

        let radius = 1.0;

        vertices[0] = center + side * radius + up * radius;
        vertices[1] = center + side * radius - up * radius;
        vertices[2] = center - side * radius - up * radius;
        vertices[3] = center - side * radius + up * radius;

        vec![
            Tri::new([vertices[0], vertices[1], vertices[2]]),
            Tri::new([vertices[0], vertices[2], vertices[3]]),
        ]
    }
}
