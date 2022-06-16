mod camera;
mod canvas;
mod loader;
mod matrix4;
mod mesh;
mod renderer;
mod texture;
mod vector2;
mod vector3;
mod vector4;
use canvas::Canvas;
use mesh::Mesh;
use renderer::render;
use vector2::Vector2;
use vector3::Vector3;
use vector4::Vector4;

fn main() {
    // let color = Vector3::new(1.0, 0.0, 0.0);
    let mut canvas = Canvas::new(256, 256, 4);
    // canvas.draw_line(0.0, 0.0, 250.0, 250.0, color);
    // canvas.save_image("test.png");
    // let scene = Scene::load("resource/DamagedHelmet/glTF/DamagedHelmet.gltf");
    let mut mesh = Mesh::from_obj_file("assets/helmet/helmet.obj");
    mesh.diffuse_texture = Some("assets/helmet/helmet_basecolor.tga".to_string());

    // let mut mesh = Mesh::new();
    // mesh.vertices = vec![
    //     Vector3::new(-1.0, -1.0, 0.0),
    //     Vector3::new(1.0, -1.0, 0.0),
    //     Vector3::new(1.0, 1.0, 0.0),
    // ];
    // mesh.uvs = vec![
    //     Vector2::new(0.0, 0.0),
    //     Vector2::new(1.0, 0.0),
    //     Vector2::new(1.0, 1.0),
    // ];
    // mesh.normals = vec![
    //     Vector3::new(0.0, 0.0, 1.0),
    //     Vector3::new(0.0, 0.0, 1.0),
    //     Vector3::new(0.0, 0.0, 1.0),
    // ];
    // mesh.indices = vec![0, 1, 2];

    let camera = camera::Camera::new(
        Vector3::new(0.0, 0.0, 3.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        45.0 / 180.0 * std::f64::consts::PI,
        256.0 / 256.0,
        0.1,
        100.0,
    );
    use std::time::Instant;
    let now = Instant::now();
    render(&mesh, &camera, &mut canvas);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    // println!("{:?}", scene);
    canvas.save_image("test.png");
}
