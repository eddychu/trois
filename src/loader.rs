use gltf::{Document, Gltf};

pub fn load(path: &str) {
    let (gltf, buffers, _) = gltf::import(path).unwrap();
    for scene in gltf.scenes() {
        for node in scene.nodes() {
            let mesh = node.mesh().unwrap();
            let primitives = mesh.primitives();
            for primitive in primitives {
                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
                if let Some(iter) = reader.read_positions() {
                    for vertex_position in iter {
                        println!("{:?}", vertex_position);
                    }
                }
                if let Some(iter) = reader.read_normals() {
                    for vertex_normal in iter {
                        println!("{:?}", vertex_normal);
                    }
                }
                if let Some(iter) = reader.read_tex_coords(0) {
                    for vertex_uv in iter.into_f32() {
                        println!("{:?}", vertex_uv);
                    }
                }
            }
        }
    }
}
