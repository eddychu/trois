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
mod light;
mod quat;
mod transform;
mod material;
mod matrix3;
use std::{f32::consts::PI, fs::metadata};

use framebuffer::FrameBuffer;
use math::{srgb_to_linear, linear_to_srgb};
use matrix4::Matrix4;
use mesh::{Mesh, Vertex};
use minifb::{Key, Window, WindowOptions};
use texture::Texture;
use vector2::Vector2;
use vector3::Vector3;
use vector4::Vector4;
use matrix3::Matrix3;
use quat::Quat;
use light::Light;
use transform::Transform;
use material::Material;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

pub struct Uniform {
    mv: Matrix4,
    normal_matrix: Matrix3,
    projection: Matrix4,
    light: Light,
    ao_tex : Texture,
    emissive_tex: Texture,
    albedo_tex: Texture,
    metal_roughness_tex: Texture,
    normal_tex: Texture,
}

#[derive(Copy, Clone)]
pub struct Varying {
    tex_coord: Vector2,
    normal: Vector3,
    position: Vector3,
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
    let view_pos = uniform.mv * Vector4::new(
        vertex.position.x,
        vertex.position.y,
        vertex.position.z,
        1.0,
    );

    let normal = (uniform.mv * Vector4::new(vertex.normal.x, vertex.normal.y, vertex.normal.z, 0.0)).xyz().normalize();

    // let normal = (uniform.normal_matrix * vertex.normal).normalize();
    

    let position = uniform.projection * uniform.mv
        * Vector4::new(vertex.position.x, vertex.position.y, vertex.position.z, 1.0);
    let varying = Varying {
        tex_coord: vertex.tex_coord,
        normal,
        position: view_pos.xyz(),
    };
    VertexOutput { position, varying }
}


fn distribution_factor(n_dot_h: f32, alpha2: f32) -> f32 {
    let n_dot_h_2 = n_dot_h * n_dot_h;
    let factor = n_dot_h_2 * (alpha2 - 1.0) + 1.0;
    alpha2 / (PI * factor * factor)
}

fn geom_smith_factor(dot_product: f32, roughness: f32) -> f32 {
    let k = (roughness + 1.0) * (roughness + 1.0) / 8.0;
    let mut denom = dot_product * (1.0 - k) + k;
    if denom <= 0.0 {
        denom = 0.0001;
    }
    return 1.0 / denom;
} 



fn fresnel(v_dot_h: f32, f0: Vector3) -> Vector3 {
    return f0 + (Vector3::new(1.0, 1.0, 1.0) - f0) * ((1.0 - v_dot_h).clamp(0.0, 1.0).powf(5.0));
}

pub fn fragment_shader(varying: &Varying, uniform: &Uniform) -> Vector4 {
    let normal = varying.normal.normalize();
    let uv = varying.tex_coord;
    let ao = uniform.ao_tex.sample(uv).x;
    let emission = uniform.emissive_tex.sample(uv).xyz();

    let albedo = uniform.albedo_tex.sample(uv).xyz();
    let mr = uniform.metal_roughness_tex.sample(uv);
    let roughness = mr.y;
    let metallic = mr.z;
    let v = -varying.position;
    let mut f0 = Vector3::new(0.04, 0.04, 0.04);
    f0 = f0 * (1.0 - metallic) + albedo * metallic;
    let mut l = uniform.light.transform.position - varying.position;
    let distance_sqr = l.length_squared();
    l = l.normalize();
    let h = (v + l).normalize();
    let n_dot_v  = normal.dot(v).max(0.0);
    
    let n_dot_l = normal.dot(l);
    if n_dot_l <= 0.0 {
        return Vector4::new(0.0, 0.0, 0.0, 1.0);
    }
    let n_dot_h = normal.dot(h).max(0.0);
    let v_dot_h = v.dot(h).max(0.0);
    let attenuation = 1.0 / distance_sqr;
    let radiance = uniform.light.intensity * attenuation;
    let alpha_roughness = roughness * roughness;
    let alpha2 = alpha_roughness * alpha_roughness;
    let d_term = distribution_factor(n_dot_h, alpha2);
    let v_term: f32 = geom_smith_factor(n_dot_l, roughness) * geom_smith_factor(n_dot_v, roughness);
    let f_term = fresnel(v_dot_h, f0);
    // println!("{:?}", f_term);
    let diffuse = albedo * (1.0 / PI);
    let specular = f_term * v_term * d_term * 0.25;

    let mut color = diffuse + specular + emission; 
    let ambient = Vector3::new(0.1, 0.1, 0.1) * ao;
    // println!("{:?}", color);
    color = color * n_dot_l * radiance;
    color = color + emission + ambient;
    color.x = color.x.clamp(0.0, 1.0);
    color.y = color.y.clamp(0.0, 1.0);
    color.z = color.z.clamp(0.0, 1.0);
    // color = Vector3::new(linear_to_srgb(color.x), linear_to_srgb(color.y), linear_to_srgb(color.z));

    Vector4::new(color.x, color.y, color.z, 1.0)

    // let light_dir = uniform.light.transform.position - varying.position;

    // let n_dot_l = normal.dot(light_dir.normalize()).max(0.0);

    // Vector4::from_vector3(k_ao * k_d * n_dot_l + k_e.xyz())
    // Vector4::from_vector3(normal)
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
    let normal = edge1.cross(edge2);
    return normal.z < 0.0;
}

pub fn draw_triangle(framebuffer: &mut FrameBuffer, vertices: &[Vertex], uniform: &Uniform) {
    let mut varyings = vec![
        Varying {
            tex_coord: vertices[0].tex_coord,
            normal: vertices[0].normal,
            position: vertices[0].position,
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
                        let position = varyings[0].position * bary_correct.x
                            + varyings[1].position * bary_correct.y
                            + varyings[2].position * bary_correct.z;
                        let varying = Varying { tex_coord, normal, position };
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

    

    let mut light = Light::new(Vector3::new(0.2, 0.2, 0.2), Vector3::new(5.0, 5.0, 5.0), Transform::identity());
    light.transform.position = (camera.get_view_matrix() * Vector4::from_vector3(Vector3::new(0.0, 0.0, 3.0))).xyz();

    // let mut mesh = Mesh::from_obj_file("assets/common/box.obj");

    let mut mesh = Mesh::from_obj_file("assets/helmet/helmet.obj");
    
    let albedo_tex = Texture::load("assets/helmet/helmet_basecolor.tga");
    let normal_tex = Texture::load("assets/helmet/helmet_normal.tga");
    let metal_roughness_tex = Texture::load("assets/helmet/helmet_metalRoughness.jpg");
    let emissive_tex = Texture::load("assets/helmet/helmet_emission.tga");
    let ao_tex = Texture::load("assets/helmet/helmet_occlusion.tga");
   

    // let mesh = Mesh::from_obj_file("assets/crab/crab.obj");
    // let diffuse_texture = Texture::load("assets/crab/crab_diffuse.tga");
   
    let mut uniform = Uniform {
        mv : Matrix4::identity(),
        normal_matrix: Matrix3::identity(),
        projection: camera.get_projection_matrix(),
        albedo_tex,
        metal_roughness_tex,
        emissive_tex,
        ao_tex,
        normal_tex,
        light,
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
        uniform.mv = camera.get_view_matrix() * mesh.transform.to_mat4();
        let normal = Matrix3::from_mat4(uniform.mv);
        uniform.normal_matrix = normal;
        // let light_pos = camera.get_view_matrix() * Vector4::new(light.transform.position.x, 
        //         light.transform.position.y, light.transform.position.z, 1.0);
        // uniform.light.transform.position = Vector3::new(light_pos.x, light_pos.y, light_pos.z);
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
