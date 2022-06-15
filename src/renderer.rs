use crate::camera::Camera;
use crate::canvas::Canvas;
use crate::mesh::Mesh;
use crate::vector3::Vector3;
use crate::vector4::Vector4;
pub fn render(mesh: &Mesh, camera: &Camera, canvas: &mut Canvas) {
    let view_matrix = camera.get_view_matrix();
    let projection_matrix = camera.get_projection_matrix();
    let view_projection_matrix = projection_matrix * view_matrix;
    println!("{:?}", view_projection_matrix);
    println!("{:?}", mesh.vertices.len());
    println!("{:?}", mesh.indices.len());
    for i in 0..mesh.indices.len() {
        let index = mesh.indices[i];
        let vertex = Vector4::new(
            mesh.vertices[index as usize].x,
            mesh.vertices[index as usize].y,
            mesh.vertices[index as usize].z,
            1.0,
        );
        let vertex = view_projection_matrix * vertex;
        let vertex = vertex * (1.0f64 / vertex.w);
        let x = vertex.x * (canvas.width as f64 / 2.0f64) + canvas.width as f64 / 2.0f64;
        let y = canvas.height as f64 / 2.0f64 - vertex.y * canvas.height as f64 / 2.0f64;
        canvas.set_pixel_rgb(x as u32, y as u32, Vector3::new(1.0, 0.0, 0.0));
    }
}
