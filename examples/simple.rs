use std::io::{stdin, stdout, Write};

use clap::Parser;
use claras_renderer::{Camera, Scene, Tri, Vec2, Vec3, Viewport};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

fn setup() -> Scene {
    // Camera looking at (0, 0, 0) from +z
    let camera = Camera::upright(
		Vec3::new(-0.1, 0.2, 1.0), 
		Vec3::new(0.09983342, 0.0,  -0.9950042)
	);

    let viewport = Viewport::new(150, 50);

    Scene::new(camera, viewport)
        // .add_shape(vec![Tri {
        //     vertices: [
        //         Vec3::new(0.0, 0.0, 1.0),
        //         Vec3::new(1.0, 0.0, 0.0),
        //         Vec3::new(0.0, 1.0, 0.0),
        //     ],
        // }])
        .add_shape(vec![Tri {
            vertices: [
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(0.0, 1.0, 0.0),
            ],
        }])
        // .add_shape(vec![Tri {
        //     vertices: [
        //         Vec3::new(0.0, 0.0, 0.0),
        //         Vec3::new(1.0, 0.0, 0.0),
        //         Vec3::new(0.0, 1.0, 0.0),
        //     ],
        // }])
}

fn termion_render(scene: &Scene) {
    let output = scene.render();

    for (i, line) in output.into_iter().rev().enumerate() {
        print!("{}", line);
        print!("{}", termion::cursor::Goto(1, i as u16 + 1));
    }
}

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    playground: bool,
}

fn main() {
    let args = Args::parse();

    if args.playground {
        playground();
    } else {
        let scene = setup();
        let mut output = scene.render();
        output.reverse();

        print!("{}", output.join("\n"));
    }
}

fn playground() {
    let mut scene = setup();

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide
    )
    .unwrap();
    stdout.flush().unwrap();

    termion_render(&scene);

    for c in stdin.keys() {
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All
        )
        .unwrap();

        match c.unwrap() {
            Key::Char('q') => break,

            Key::Char('d') => scene.camera.plane_translate(Vec2::new(0.0, 0.1)),
            Key::Char('a') => scene.camera.plane_translate(Vec2::new(0.0, -0.1)),
            Key::Char('w') => scene.camera.plane_translate(Vec2::new(0.1, 0.0)),
            Key::Char('s') => scene.camera.plane_translate(Vec2::new(-0.1, 0.0)),

            Key::Left => scene.camera.rotate(0.1),
            Key::Right => scene.camera.rotate(-0.1),

            Key::Up => scene.camera.translate(Vec3::new(0.0, 0.1, 0.0)),
            Key::Down => scene.camera.translate(Vec3::new(0.0, -0.1, 0.0)),

            _ => (),
        }

        termion_render(&scene);
    }

    write!(stdout, "{}{}", termion::cursor::Show, termion::style::Reset).unwrap();
}
