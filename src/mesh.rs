use crate::matrix4::Matrix4;
use crate::vector2::Vector2;
use crate::vector3::Vector3;
use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
};
pub struct Mesh {
    pub vertices: Vec<Vector3>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn from_obj_file(path: &str) -> Mesh {
        let mut indices: Vec<u32> = Vec::new();
        let mut vertices: Vec<Vector3> = Vec::new();
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line.unwrap();
            let mut parts = line.split_whitespace();
            let first = parts.next().unwrap();
            match first {
                "vt" => {
                    let x = parts.next().unwrap().parse::<f64>().unwrap();
                    let y = parts.next().unwrap().parse::<f64>().unwrap();
                    println!("{:?}", Vector2::new(x, y));
                }
                "vn" => {
                    let x = parts.next().unwrap().parse::<f64>().unwrap();
                    let y = parts.next().unwrap().parse::<f64>().unwrap();
                    let z = parts.next().unwrap().parse::<f64>().unwrap();
                    println!("{:?}", Vector3::new(x, y, z));
                }
                "v" => {
                    let x = parts.next().unwrap().parse::<f64>().unwrap();
                    let y = parts.next().unwrap().parse::<f64>().unwrap();
                    let z = parts.next().unwrap().parse::<f64>().unwrap();
                    let vertex = Vector3::new(x, y, z);
                    vertices.push(vertex);
                }
                "f" => {
                    let f = parts.next().unwrap();
                    let mut first = f.split('/');
                    let v1 = first.next().unwrap().parse::<u32>().unwrap() - 1;
                    // let t1 = first.next().unwrap().parse::<u32>().unwrap() - 1;
                    // let n1 = first.next().unwrap().parse::<u32>().unwrap() - 1;

                    let f = parts.next().unwrap();
                    let mut second = f.split('/');
                    let v2 = second.next().unwrap().parse::<u32>().unwrap() - 1;
                    // let t2 = second.next().unwrap().parse::<u32>().unwrap() - 1;
                    // let n2 = second.next().unwrap().parse::<u32>().unwrap() - 1;

                    let f = parts.next().unwrap();
                    let mut third = f.split('/');
                    let v3 = third.next().unwrap().parse::<u32>().unwrap() - 1;
                    // let t3 = third.next().unwrap().parse::<u32>().unwrap() - 1;
                    // let n3 = third.next().unwrap().parse::<u32>().unwrap() - 1;
                    indices.push(v1);
                    indices.push(v2);
                    indices.push(v3);
                }

                _ => {}
            }
        }
        Mesh { vertices, indices }
    }
}
