mod canvas;
mod loader;
mod scene;
mod vector2;
mod vector3;
mod vector4;
use canvas::Canvas;
use loader::load;
use scene::Scene;
use vector3::Vector3;
use vector4::Vector4;
fn main() {
    let color = Vector3::new(1.0, 0.0, 0.0);
    let mut canvas = Canvas::new(256, 256, 4);
    canvas.draw_line(0.0, 0.0, 250.0, 250.0, color);
    canvas.save_image("test.png");
    // let scene = load("resource/DamagedHelmet/glTF/DamagedHelmet.gltf");
    // println!("{:?}", scene);
}
