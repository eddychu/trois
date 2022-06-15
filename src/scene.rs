use gltf::buffer::Data;
use gltf::{Document, Gltf};

use crate::vector2::Vector2;
use crate::vector3::Vector3;
use crate::vector4::Vector4;
#[derive(Debug)]
pub struct Node {
    pub mesh: Option<u32>,
    pub children: Vec<u32>,
    pub parent: Option<u32>,
    pub index: u32,
}

pub struct Mesh {
    pub first_index: u32,
    pub num_indices: u32,
    pub first_vertex: u32,
    pub num_vertices: u32,
}

pub struct Scene {
    nodes: Vec<Node>,
    meshes: Vec<Mesh>,
    root: Option<u32>,
    vertices: Vec<Vector3>,
    normals: Vec<Vector3>,
    tex_coords: Vec<Vector2>,
    indices: Vec<u32>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            nodes: Vec::new(),
            meshes: Vec::new(),
            root: None,
            vertices: Vec::new(),
            normals: Vec::new(),
            tex_coords: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn node(&self, index: u32) -> &Node {
        &self.nodes[index as usize]
    }

    pub fn mesh(&self, index: u32) -> &Mesh {
        &self.meshes[index as usize]
    }

    pub fn root(&self) -> &Node {
        &self.nodes[self.root.unwrap() as usize]
    }

    pub fn position(&self, index: u32) -> &Vector3 {
        &self.vertices[index as usize]
    }

    pub fn normal(&self, index: u32) -> &Vector3 {
        &self.normals[index as usize]
    }

    pub fn indice(&self, index: u32) -> u32 {
        self.indices[index as usize]
    }

    pub fn load(path: &str) -> Scene {
        let mut result = Scene::new();
        let (gltf, buffers, _) = gltf::import(path).unwrap();
        let scene = gltf.scenes().nth(0).unwrap();
        let root = Node {
            mesh: None,
            children: Vec::new(),
            parent: None,
            index: 0,
        };
        result.nodes.push(root);
        result.root = Some(0);
        for node in scene.nodes() {
            result.process_node(&node, 0, &buffers);
        }

        return result;
    }

    fn process_node(&mut self, node: &gltf::Node, parent: u32, buffers: &Vec<Data>) {
        let mesh = node.mesh().unwrap();

        let mut new_mesh = Mesh {
            first_index: self.indices.len() as u32,
            num_indices: 0,
            first_vertex: self.vertices.len() as u32,
            num_vertices: 0,
        };
        let new_node = Node {
            mesh: Some(self.meshes.len() as u32),
            children: Vec::new(),
            parent: Some(parent),
            index: self.nodes.len() as u32,
        };
        self.nodes[parent as usize].children.push(new_node.index);
        self.nodes.push(new_node);

        let primitives = mesh.primitives();

        for primitive in primitives {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            if let Some(iter) = reader.read_positions() {
                let num_vertices = iter.len();
                new_mesh.num_vertices = num_vertices as u32;
                for vertex_position in iter {
                    let position = Vector3::new(
                        vertex_position[0] as f64,
                        vertex_position[1] as f64,
                        vertex_position[2] as f64,
                    );
                    self.vertices.push(position);
                }
            }
            if let Some(iter) = reader.read_normals() {
                for vertex_normal in iter {
                    let normal = Vector3::new(
                        vertex_normal[0] as f64,
                        vertex_normal[1] as f64,
                        vertex_normal[2] as f64,
                    );
                    self.normals.push(normal);
                }
            }
            if let Some(iter) = reader.read_tex_coords(0) {
                for vertex_uv in iter.into_f32() {
                    let uv = Vector2::new(vertex_uv[0] as f64, vertex_uv[1] as f64);
                    self.tex_coords.push(uv);
                }
            }
            if let Some(iter) = reader.read_indices() {
                let iter = iter.into_u32();
                new_mesh.num_indices = iter.len() as u32;
                for index in iter {
                    self.indices.push(index);
                }
            }
        }
        self.meshes.push(new_mesh);
    }
}
