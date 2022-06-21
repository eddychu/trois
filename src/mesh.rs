use crate::matrix4::Matrix4;
use crate::vector2::Vector2;
use crate::vector3::Vector3;
use crate::transform::Transform;
use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
};

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: Vector3,
    pub tex_coord: Vector2,
    pub normal: Vector3,
}

impl Vertex {
    pub fn new(position: Vector3, tex_coord: Vector2, normal: Vector3) -> Vertex {
        Vertex {
            position,
            tex_coord,
            normal,
        }
    }
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<usize>,
    pub diffuse_texture: Option<String>,
    pub transform: Transform,
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
            diffuse_texture: None,
            transform: Transform::identity(),
        }
    }

    pub fn from_obj_file(path: &str) -> Mesh {
        let mut indices: Vec<usize> = Vec::new();
        let mut positions: Vec<Vector3> = Vec::new();
        let mut uvs: Vec<Vector2> = Vec::new();
        let mut normals: Vec<Vector3> = Vec::new();
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line.unwrap();
            let mut parts = line.split_whitespace();
            let first = parts.next();
            match first {
                Some(first) => {
                    match first {
                        "#" => continue,
                        "vt" => {
                            let x = parts.next().unwrap().parse::<f32>().unwrap();
                            let y = parts.next().unwrap().parse::<f32>().unwrap();
                            // println!("{:?}", Vector2::new(x, y));
                            uvs.push(Vector2::new(x, 1.0 - y));
                        }
                        "vn" => {
                            let x = parts.next().unwrap().parse::<f32>().unwrap();
                            let y = parts.next().unwrap().parse::<f32>().unwrap();
                            let z = parts.next().unwrap().parse::<f32>().unwrap();
                            // println!("{:?}", Vector3::new(x, y, z));
                            normals.push(Vector3::new(x, y, z));
                        }
                        "v" => {
                            let x = parts.next().unwrap().parse::<f32>().unwrap();
                            let y = parts.next().unwrap().parse::<f32>().unwrap();
                            let z = parts.next().unwrap().parse::<f32>().unwrap();
                            let vertex = Vector3::new(x, y, z);
                            positions.push(vertex);
                        }
                        "f" => {
                            let f = parts.next().unwrap();
                            let mut first = f.split('/');
                            let v1 = first.next().unwrap().parse::<usize>().unwrap() - 1;
                            // let t1 = first.next().unwrap().parse::<u32>().unwrap() - 1;
                            // let n1 = first.next().unwrap().parse::<u32>().unwrap() - 1;

                            let f = parts.next().unwrap();
                            let mut second = f.split('/');
                            let v2 = second.next().unwrap().parse::<usize>().unwrap() - 1;
                            // let t2 = second.next().unwrap().parse::<u32>().unwrap() - 1;
                            // let n2 = second.next().unwrap().parse::<u32>().unwrap() - 1;

                            let f = parts.next().unwrap();
                            let mut third = f.split('/');
                            let v3 = third.next().unwrap().parse::<usize>().unwrap() - 1;
                            // let t3 = third.next().unwrap().parse::<u32>().unwrap() - 1;
                            // let n3 = third.next().unwrap().parse::<u32>().unwrap() - 1;
                            indices.push(v1);
                            indices.push(v2);
                            indices.push(v3);
                        }

                        _ => {}
                    }
                }
                None => continue,
            };
        }
        let mut vertices: Vec<Vertex> = Vec::new();
        for i in 0..positions.len() {
            let position = positions[i];
            let tex_coord = uvs[i];
            let normal = normals[i];
            let vertex = Vertex::new(position, tex_coord, normal);
            vertices.push(vertex);
        }
        Mesh {
            vertices,
            indices,
            diffuse_texture: None,
            transform: Transform::identity(),
        }
    }
}
