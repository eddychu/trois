mod camera;
mod framebuffer;
mod loader;
mod matrix4;
mod mesh;
mod math;
mod raster;
mod renderer;
mod texture;
mod vector2;
mod vector3;
mod vector4;
mod quat;
mod transform;
use framebuffer::FrameBuffer;
use matrix4::Matrix4;
use mesh::{Mesh, Vertex};
use minifb::{Key, Window, WindowOptions};
use texture::Texture;
use vector2::Vector2;
use vector3::Vector3;
use vector4::Vector4;
use quat::Quat;
use transform::Transform;
const WIDTH: usize = 640;
const HEIGHT: usize = 480;

pub struct Uniform {
    model: Matrix4,
    view: Matrix4,
    projection: Matrix4,
    diffuse_texture: Texture,
}

#[derive(Copy, Clone)]
pub struct Varying {
    tex_coord: Vector2,
    normal: Vector3,
}

pub struct VertexOutput {
    position: Vector4,
    varying: Varying,
}

#[derive(Copy, Clone, Debug)]
pub struct Box2D {
    pub min: Vector2,
    pub max: Vector2,
}

pub fn vertex_shader(vertex: &Vertex, uniform: &Uniform) -> VertexOutput {
    let position = uniform.projection
        * uniform.view
        * uniform.model
        * Vector4::new(vertex.position.x, vertex.position.y, vertex.position.z, 1.0);
    let varying = Varying {
        tex_coord: vertex.tex_coord,
        normal: vertex.normal,
    };
    VertexOutput { position, varying }
}

pub fn fragment_shader(varying: &Varying, uniform: &Uniform) -> Vector4 {
    let tex_coord = varying.tex_coord;
    let tex_color = uniform
        .diffuse_texture
        .sample(tex_coord.x, tex_coord.y);
    tex_color
}

pub fn get_box2d(vertices: &[Vector4]) -> Box2D {
    let mut min = Vector2::new(std::f32::MAX, std::f32::MAX);
    let mut max = Vector2::new(std::f32::MIN, std::f32::MIN);
    for vertex in vertices {
        min.x = min.x.min(vertex.x);
        min.y = min.y.min(vertex.y);
        max.x = max.x.max(vertex.x);
        max.y = max.y.max(vertex.y);
    }
    Box2D {
        min: Vector2::new(min.x, min.y),
        max: Vector2::new(max.x, max.y),
    }
}

pub fn is_back_face(vertices: &[Vector4]) -> bool {
    let edge1 = (vertices[1] - vertices[0]).xyz();
    let edge2 = (vertices[2] - vertices[0]).xyz();
    let normal = edge1.cross(&edge2);
    return normal.z < 0.0;
}

pub fn draw_triangle(framebuffer: &mut FrameBuffer, vertices: &[Vertex], uniform: &Uniform) {
    let mut varyings = vec![
        Varying {
            tex_coord: vertices[0].tex_coord,
            normal: vertices[0].normal,
        };
        3
    ];
    let mut gl_positions = vec![
        Vector4::new(
            vertices[0].position.x,
            vertices[0].position.y,
            vertices[0].position.z,
            1.0
        );
        3
    ];
    for i in 0..3 {
        let vertex = &vertices[i];
        let vertex_output = vertex_shader(&vertex, uniform);
        gl_positions[i] = vertex_output.position;       
        varyings[i] = vertex_output.varying;
    }

    if is_back_face(&gl_positions) {
        return;
    }

    for i in 0..3 {
        gl_positions[i] = perspective_divide(gl_positions[i]);
        gl_positions[i] =
        viewport_transform(gl_positions[i], (WIDTH - 1) as f32, (HEIGHT - 1) as f32);
    }

    

    let mut bbox = get_box2d(&gl_positions);
    bbox.min.x = bbox.min.x.max(0.0);
    bbox.min.y = bbox.min.y.max(0.0);
    bbox.max.x = bbox.max.x.min((WIDTH - 1) as f32);
    bbox.max.y = bbox.max.y.min((HEIGHT - 1) as f32);

    for y in bbox.min.y as u32..(bbox.max.y + 1.0) as u32 {
        for x in bbox.min.x as u32..(bbox.max.x + 1.0) as u32 {
            let mut frag_pos = Vector4::new(x as f32 + 0.5, y as f32 + 0.5, 0.0, 1.0);
            let bary = barycentric(gl_positions[0], gl_positions[1], gl_positions[2], frag_pos);
            if bary.x >= 0.0 && bary.y >= 0.0 && bary.z >= 0.0 {
                frag_pos.z = gl_positions[0].z * bary.x
                    + gl_positions[1].z * bary.y
                    + gl_positions[2].z * bary.z;

                if frag_pos.z >= 0.0 && frag_pos.z <= 1.0 {
                    if frag_pos.z <= framebuffer.get_depth(x, y) {
                        framebuffer.set_depth(x, y, frag_pos.z);
                        frag_pos.w = gl_positions[0].w * bary.x
                            + gl_positions[1].w * bary.y
                            + gl_positions[2].w * bary.z;

                        let bary_correct = bary
                            * Vector3::new(gl_positions[0].w, gl_positions[1].w, gl_positions[2].w)
                            * (1.0 / frag_pos.w);
                        let tex_coord = Vector2::new(
                            bary_correct.x * varyings[0].tex_coord.x
                                + bary_correct.y * varyings[1].tex_coord.x
                                + bary_correct.z * varyings[2].tex_coord.x,
                            bary_correct.x * varyings[0].tex_coord.y
                                + bary_correct.y * varyings[1].tex_coord.y
                                + bary_correct.z * varyings[2].tex_coord.y,
                        );

                        let normal = varyings[0].normal * bary_correct.x
                            + varyings[1].normal * bary_correct.y
                            + varyings[2].normal * bary_correct.z;
                        let varying = Varying { tex_coord, normal };
                        let frag_color = fragment_shader(&varying, uniform);
                        framebuffer.set_color(x, y, frag_color.to_u32());
                    }
                }
            }
        }
    }
}

fn barycentric(v0: Vector4, v1: Vector4, v2: Vector4, p: Vector4) -> Vector3 {
    let e0 = Vector2::new(v1.x - v0.x, v1.y - v0.y);
    let e1 = Vector2::new(v2.x - v0.x, v2.y - v0.y);
    let e2 = Vector2::new(p.x - v0.x, p.y - v0.y);
    let d00 = e0.dot(e0);
    let d01 = e0.dot(e1);
    let d11 = e1.dot(e1);
    let d20 = e2.dot(e0);
    let d21 = e2.dot(e1);

    let denom = d00 * d11 - d01 * d01;
    if denom.abs() < std::f32::EPSILON {
        return Vector3::new(-1.0, -1.0, -1.0);
    }
    let v = (d11 * d20 - d01 * d21) / denom;
    let w = (d00 * d21 - d01 * d20) / denom;

    return Vector3::new(1.0 - v - w, v, w);
}

fn viewport_transform(vertex: Vector4, width: f32, height: f32) -> Vector4 {
    let mut vertex = vertex;
    vertex.x = vertex.x * (width / 2.0) + width / 2.0;
    vertex.y = height / 2.0 - vertex.y * (height / 2.0);
    return vertex;
}

fn perspective_divide(vertex: Vector4) -> Vector4 {
    let mut vertex = vertex;
    vertex.x /= vertex.w;
    vertex.y /= vertex.w;
    vertex.z /= vertex.w;
    vertex.w = 1.0 / vertex.w;
    return vertex;
}

fn main() {
    let camera = camera::Camera::new(
        Vector3::new(0.0, 0.0, 4.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        45.0 / 180.0 * std::f32::consts::PI,
        WIDTH as f32 / HEIGHT as f32,
        0.1,
        100.0,
    );

    let mut mesh = Mesh::from_obj_file("assets/helmet/helmet.obj");
    
    let diffuse_texture = Texture::load("assets/helmet/helmet_basecolor.tga");


    // let mesh = Mesh::from_obj_file("assets/crab/crab.obj");
    // let diffuse_texture = Texture::load("assets/crab/crab_diffuse.tga");

    let mut uniform = Uniform {
        model: mesh.transform.to_mat4(),
        view: camera.get_view_matrix(),
        projection: camera.get_projection_matrix(),
        diffuse_texture,
    };

    let mut framebuffer = FrameBuffer::new(WIDTH as u32, HEIGHT as u32);

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut angle = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {

        window.get_keys_released().iter().for_each(|key| match key {
            Key::Right => println!("Right"),
            _ => (),
        });

        framebuffer.clear(0xFF000000);
        let start = std::time::Instant::now();
        angle += 0.1;
        let rotation = Quat::angle_axis(angle, &Vector3::new(0.0, 1.0, 0.0));

        mesh.transform.rotation = rotation;
        uniform.model = mesh.transform.to_mat4();

        // println!("{:?}", uniform.model);
        for i in (0..mesh.indices.len()).step_by(3) {
            let i0 = mesh.indices[i];
            let i1 = mesh.indices[i + 1];
            let i2 = mesh.indices[i + 2];
            let v0 = mesh.vertices[i0];
            let v1 = mesh.vertices[i1];
            let v2 = mesh.vertices[i2];
            let vertices = [v0, v1, v2];
            draw_triangle(&mut framebuffer, &vertices, &uniform);
        }
        // draw_triangle(&mut framebuffer, &vertices, &uniform);
        let frame_time = start.elapsed();
        println!("{}", frame_time.as_secs_f32());



        window
            .update_with_buffer(framebuffer.get_colors(), WIDTH, HEIGHT)
            .unwrap();
    }
}
