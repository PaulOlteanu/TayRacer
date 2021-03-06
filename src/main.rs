use cgmath::Vector3;

mod canvas;
mod collision;
mod ray;
mod scene;
mod shading;
mod util;

use self::canvas::Canvas;
use self::scene::{scene_objects::Sphere, Scene};

const CANVAS_WIDTH: u32 = 200;
const CANVAS_HEIGHT: u32 = 100;
const CAMERA_DISTANCE: f64 = 100.0;

const RAYS_PER_PIXEL: u64 = 100;

fn main() {
    let _x = Vector3::new(1.0, 0.0, 0.0);
    let _y = Vector3::new(0.0, 1.0, 0.0);
    let z = Vector3::new(0.0, 0.0, 1.0);

    let camera_origin = Vector3::new(0.0, 0.0, 0.0);
    let camera_direction = -z; // Pointing to the negative z axis

    let mut scene = Scene::new();

    let canvas = Canvas::new(
        CANVAS_WIDTH,
        CANVAS_HEIGHT,
        CAMERA_DISTANCE,
        camera_origin,
        camera_direction,
    );

    let sphere1_origin = Vector3::new(120.0, 0.0, -200.0);
    let sphere1_radius = 100.0;
    let sphere1_shader = shading::Lambertian {
        albedo: Vector3::new(0.8, 0.8, 0.8),
    };
    let sphere1 = Sphere::new(sphere1_origin, sphere1_radius, Box::new(sphere1_shader));
    scene.add_object(sphere1);

    let sphere3_origin = Vector3::new(-120.0, 0.0, -200.0);
    let sphere3_radius = 100.0;
    let sphere3_shader = shading::Metal {
        albedo: Vector3::new(0.8, 0.8, 0.8),
    };
    let sphere3 = Sphere::new(sphere3_origin, sphere3_radius, Box::new(sphere3_shader));
    scene.add_object(sphere3);

    let sphere2_origin = Vector3::new(0.0, -20100.0, -200.0);
    let sphere2_radius = 20000.0;
    let sphere2_shader = shading::Lambertian {
        albedo: Vector3::new(0.8, 0.8, 0.8),
    };
    let sphere2 = Sphere::new(sphere2_origin, sphere2_radius, Box::new(sphere2_shader));
    scene.add_object(sphere2);

    canvas.capture(&scene, RAYS_PER_PIXEL, String::from("asdf"));
}
