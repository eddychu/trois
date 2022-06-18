// use crate::camera::Camera;
// use crate::canvas::Canvas;
// use crate::mesh::Mesh;
// use crate::texture::Texture;
// use crate::vector2::Vector2;
// use crate::vector3::Vector3;
// use crate::vector4::Vector4;
// fn clipping(clipping_coords: &Vec<Vector4>) -> bool {
//     let mut count = 0;
//     for i in 0..clipping_coords.len() {
//         let vertex = clipping_coords[i];
//         let inside = (-vertex.w <= vertex.x && vertex.x <= vertex.w)
//             && (-vertex.w <= vertex.y && vertex.y <= vertex.w)
//             && (0.0 <= vertex.z && vertex.z <= vertex.w);
//         if !inside {
//             count += 1;
//         }
//     }
//     return count == 3;
// }

// fn perspective_divide(vertex: &mut Vector4) {
//     vertex.x /= vertex.w;
//     vertex.y /= vertex.w;
//     vertex.z /= vertex.w;
// }

// fn is_back_face(vertex: &Vec<Vector4>) -> bool {
//     let xa = vertex[0].x;
//     let xb = vertex[1].x;
//     let xc = vertex[2].x;

//     let ya = vertex[0].y;
//     let yb = vertex[1].y;
//     let yc = vertex[2].y;

//     return xa * (yb - yc) + xb * (yc - ya) + xc * (ya - yb) <= 0.0;
// }

// fn viewport_transform(vertex: &mut Vector4, width: f64, height: f64) {
//     vertex.x = vertex.x * (width / 2.0f64) + width / 2.0f64;
//     vertex.y = height / 2.0f64 - vertex.y * (height / 2.0f64);
// }

// fn barycentric(vertex: &Vec<Vector4>, x: f64, y: f64) -> Vector3 {
//     let mut v0 = Vector2::new(vertex[1].x - vertex[0].x, vertex[1].y - vertex[0].y);
//     let mut v1 = Vector2::new(vertex[2].x - vertex[0].x, vertex[2].y - vertex[0].y);
//     let mut v2 = Vector2::new(x - vertex[0].x, y - vertex[0].y);
//     let d00 = v0.dot(&v0);
//     let d01 = v0.dot(&v1);
//     let d11 = v1.dot(&v1);
//     let d20 = v2.dot(&v0);
//     let d21 = v2.dot(&v1);

//     let denom = d00 * d11 - d01 * d01;
//     let v = (d11 * d20 - d01 * d21) / denom;
//     let w = (d00 * d21 - d01 * d20) / denom;
//     let u = 1.0 - v - w;
//     return Vector3::new(u, v, w);
// }

// pub fn render(mesh: &Mesh, camera: &Camera, canvas: &mut Canvas) {
//     let red = Vector3::new(1.0, 0.0, 0.0);
//     let green = Vector3::new(0.0, 1.0, 0.0);
//     let blue = Vector3::new(0.0, 0.0, 1.0);
//     let view_matrix = camera.get_view_matrix();
//     let projection_matrix = camera.get_projection_matrix();
//     let view_projection_matrix = projection_matrix * view_matrix;
//     let mut primitive: Vec<Vector4> = vec![Vector4::new(0.0, 0.0, 0.0, 0.0); 3];
//     let mut uvs: Vec<Vector2> = vec![Vector2::new(0.0, 0.0); 3];
//     let mut normals: Vec<Vector3> = vec![Vector3::new(0.0, 0.0, 0.0); 3];
//     let mut diffuse_texture: Option<Texture> = None;
//     match mesh.diffuse_texture {
//         Some(ref texture) => {
//             diffuse_texture = Some(Texture::load(texture));
//         }
//         None => {}
//     }
//     for i in 0..mesh.indices.len() / 3 {
//         for j in 0..3 {
//             let index = mesh.indices[i * 3 + j] as usize;
//             let position = mesh.vertices[index];
//             uvs[j] = mesh.uvs[index];
//             primitive[j] =
//                 view_projection_matrix * Vector4::new(position.x, position.y, position.z, 1.0);
//         }
//         if !clipping(&primitive) {
//             // perspective divide
//             for j in 0..3 {
//                 perspective_divide(&mut primitive[j]);
//             }
//             // face culling
//             if !is_back_face(&primitive) {
//                 for j in 0..3 {
//                     viewport_transform(
//                         &mut primitive[j],
//                         canvas.width as f64,
//                         canvas.height as f64,
//                     );
//                 }

//                 let mut x_max = primitive[0]
//                     .x
//                     .max(primitive[1].x)
//                     .max(primitive[2].x)
//                     .min(canvas.width as f64 - 1.0);
//                 let mut x_min = primitive[0]
//                     .x
//                     .min(primitive[1].x)
//                     .min(primitive[2].x)
//                     .max(0.0);
//                 let mut y_max = primitive[0]
//                     .y
//                     .max(primitive[1].y)
//                     .max(primitive[2].y)
//                     .min(canvas.height as f64 - 1.0);
//                 let mut y_min = primitive[0]
//                     .y
//                     .min(primitive[1].y)
//                     .min(primitive[2].y)
//                     .max(0.0);
//                 for x in x_min as usize..x_max as usize {
//                     for y in y_min as usize..y_max as usize {
//                         let mut barycentric =
//                             barycentric(&primitive, x as f64 + 0.5, y as f64 + 0.5);
//                         if barycentric.x < 0.0 || barycentric.y < 0.0 || barycentric.z < 0.0 {
//                             continue;
//                         }

//                         let z = barycentric.x * primitive[0].z
//                             + barycentric.y * primitive[1].z
//                             + barycentric.z * primitive[2].z;
//                         if canvas.get_depth(x as u32, y as u32) >= z {
//                             barycentric = Vector3::new(
//                                 barycentric.x / primitive[0].w,
//                                 barycentric.y / primitive[1].w,
//                                 barycentric.z / primitive[2].w,
//                             );

//                             barycentric = barycentric
//                                 * (1.0 / (barycentric.x + barycentric.y + barycentric.z));

//                             println!("{:?}", barycentric);

//                             let uv = uvs[0] * barycentric.x
//                                 + uvs[1] * barycentric.y
//                                 + uvs[2] * barycentric.z;
//                             println!("{:?}", uvs);
//                             println!("{:?}", uv);
//                             let mut color = Vector4::new(1.0, 0.0, 0.0, 1.0);

//                             if diffuse_texture.is_some() {
//                                 let texture = diffuse_texture.as_ref().unwrap();
//                                 println!("{:?}", texture.width);
//                                 let u = (uv.x * (texture.width - 1) as f64).floor() as u32;
//                                 let v = ((1.0 - uv.y) * (texture.height - 1) as f64).floor() as u32;
//                                 println!("{} {}", u, v);
//                                 color = texture.get_pixel(u, v);
//                             } else {
//                                 let inter_color = red * barycentric.x
//                                     + green * barycentric.y
//                                     + blue * barycentric.z;
//                                 color =
//                                     Vector4::new(inter_color.x, inter_color.y, inter_color.z, 1.0);
//                             }
//                             canvas.set_pixel(x as u32, y as u32, color);
//                             canvas.set_depth(x as u32, y as u32, z);
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }
