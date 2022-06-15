use crate::camera::Camera;
use crate::canvas::Canvas;
use crate::scene::Scene;
use crate::vector3::Vector3;
use crate::vector4::Vector4;
pub fn render(scene: &Scene, camera: &Camera, canvas: &mut Canvas) {
    let view_matrix = camera.get_view_matrix();
    let projection_matrix = camera.get_projection_matrix();
    let view_projection_matrix = projection_matrix * view_matrix;

    let root = scene.root();
    println!("{:?}", root);
    for child in &root.children {
        println!("{}", child);
        let node = scene.node(*child);
        let mesh = scene.mesh(node.mesh.unwrap());

        for i in 0..mesh.num_indices {
            let index = mesh.first_index + i;
            let vertex = scene.position(scene.indice(index));
            let position = view_projection_matrix * Vector4::new(vertex.x, vertex.y, vertex.z, 1.0);
            let position = position * (1.0f64 / position.w);
            let x = position.x * (canvas.width as f64 / 2.0f64) + canvas.width as f64 / 2.0f64;
            let y = canvas.height as f64 / 2.0f64 - position.y * canvas.height as f64 / 2.0f64;
            println!("{} {}", x, y);
            canvas.set_pixel_rgb(x as u32, y as u32, Vector3::new(1.0, 0.0, 0.0));
        }
    }
}
