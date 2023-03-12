use crate::{camera::ProjectedVec, objects::Tri, Color, Vec2, Viewport};

impl Tri<ProjectedVec> {
    fn vertices_to_vector(&self, point: Vec2) -> [Vec2; 3] {
        let a = point - self.vertices[0].position;
        let b = point - self.vertices[1].position;
        let c = point - self.vertices[2].position;

        [a, b, c]
    }

    pub fn signed_distance(&self, point: Vec2) -> f32 {
        let v = self.vertices_to_vector(point);

        let edges =
            [0, 1, 2].map(|i| self.vertices[(i + 1) % 3].position - self.vertices[i].position);

        let distances_squared = [0, 1, 2].map(|i| {
            let projection_factor = v[i].dot(&edges[i]) / edges[i].norm_squared();
            let projection = edges[i] * projection_factor.clamp(0.0, 1.0);
            (v[i] - projection).norm_squared()
        });

        let min_distance = distances_squared
            .into_iter()
            .reduce(f32::min)
            .unwrap()
            .sqrt();

        // Make clockwise and counterclockwise triangles have the same sign
        let handedness_corrector = edges[0].perp(&edges[2]).signum();

        let perps = [0, 1, 2].map(|i| v[i].perp(&edges[i]) * handedness_corrector);

        // Technically the reduction is kind of unecessary
        let sign = perps.into_iter().reduce(f32::min).unwrap().signum();

        -min_distance * sign
    }

    pub fn projection_distance(&self, point: Vec2) -> f32 {
        let weights = self.vertices_to_vector(point).map(|v| v.norm_squared());
        let total_weight: f32 = weights.iter().sum();

        [0, 1, 2]
            .map(|i| weights[i] * self.vertices[i].distance / total_weight)
            .iter()
            .sum()
    }
}

// #[test]
// fn test_tri_sdf() {
//     let tri = Tri {
//         vertices: [
//             Vec2::new(0.0, 0.0),
//             Vec2::new(1.0, 0.0),
//             Vec2::new(0.0, 1.0),
//         ],
//     };

//     assert_eq!(0.0, tri.signed_distance(Vec2::new(0.5, 0.0)));
//     assert_eq!(0.0, tri.signed_distance(Vec2::new(0.0, 0.0)));
//     assert_eq!(0.1, tri.signed_distance(Vec2::new(0.1, 0.1)));
//     assert_eq!(-1.0, tri.signed_distance(Vec2::new(2.0, 0.0)));

//     assert_eq!(-1.0, tri.signed_distance(Vec2::new(-1.0, 0.0)));
// }

pub fn render_buffer(
    objects: &[Tri<ProjectedVec>],
    background: Color,
    viewport: &Viewport,
) -> Buffer {
    let mut buffer = vec![vec![background; viewport.width]; viewport.height];

    (0..viewport.height).for_each(|y| {
        let y_fraction = (y as f32 * 2.0) / viewport.height as f32 - 1.0;

        (0..viewport.width).for_each(|x| {
            let x_fraction = (x as f32 * 2.0) / viewport.width as f32 - 1.0;

            let point = Vec2::new(x_fraction, y_fraction);

            for tri in objects {
                let projection_distance = tri.projection_distance(point);

                if projection_distance < 0.0 {
                    continue;
                }

                let tri_distance = tri.signed_distance(point);
                // buffer[y][x] = Color::opaque(distance, -distance, 0.0)
                if tri_distance < 0.0 {
                    buffer[y][x] = Color::opaque(tri_distance * 5.0, 1.0, projection_distance);
                } 
                // else {
                //     buffer[y][x] = Color::opaque(0.0, 0.0, projection_distance);
                // }
            }
        });
    });

    buffer
}

type Buffer = Vec<Vec<Color>>;

// fn buffer_to_string(buffer: Buffer) -> String {
//     buffer
//         .into_iter()
//         .map(|row| {
//             row.into_iter()
//                 .map(|color| color.to_ansi())
//                 .collect::<String>()
//         })
//         .collect::<Vec<String>>()
//         .join("\n")
// }

// #[test]
// fn test_render_buffer() {
//     let zoom = 1.0;
//     let tri = Tri {
//         vertices: [
//             Vec2::new(0.0, -0.196) * zoom,
//             Vec2::new(0.70014, -0.140028) * zoom,
//             Vec2::new(0.0, 0.62469506) * zoom,
//         ],
//     };

//     let viewport = Viewport {
//         width: 100,
//         height: 30,
//     };

//     let buffer = render_buffer(&[tri], Color::new(0.0, 0.0, 0.0, 1.0), &viewport);

//     println!("{}", buffer_to_string(buffer));
// }
