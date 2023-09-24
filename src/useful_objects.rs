use macroquad::prelude::*;
use macroquad::{models::Vertex, prelude::Mesh};

use crate::common::{self};

pub(crate) async fn make_cube_in_code(texture_fn:&str,size:f32) -> Mesh {
    
    let file_name = common::get_file_name_across_platforms(texture_fn);
    
    let mut my_cube_mesh = Mesh { vertices: vec![], indices: vec![], texture: 
        Some(load_texture(&file_name).await.unwrap()) 
    };

    let vertices: [f32; 180] = [
        // positions       // texture coords
                        -size, -size, -size, 0.0, 0.0,
                        size, -size, -size, 0.5, 0.0,
                        size, size, -size, 0.5, 0.5,
                        size, size, -size, 0.5, 0.5,
                        -size, size, -size, 0.0, 0.5,
                        -size, -size, -size, 0.0, 0.0,

                        -size, -size, size, 0.0, 0.5,
                        size, -size, size, 0.5, 0.5,
                        size, size, size, 0.5, 1.0,
                        size, size, size, 0.5, 1.0,
                        -size, size, size, 0.0, 0.5,
                        -size, -size, size, 0.0, 0.5,
                        
                        -size, size, size, 1.0, 0.0,
                        -size, size, -size, 1.0, 0.5,
                        -size, -size, -size, 0.5, 0.5,
                        -size, -size, -size, 0.5, 0.5,
                        -size, -size, size, 0.5, 0.0,
                        -size, size, size, 1.0, 0.0,

                        size, size, size, 1.0, 0.5,
                        size, size, -size, 1.0, 1.0,
                        size, -size, -size, 0.5, 1.0,
                        size, -size, -size, 0.5, 1.0,
                        size, -size, size, 0.5, 0.5,
                        size, size, size, 1.0, 0.5,

                        -size, -size, -size, 0.0, 0.5,
                        size, -size, -size, 0.5, 0.5,
                        size, -size, size, 0.5, 0.0,
                        size, -size, size, 0.5, 0.0,
                        -size, -size, size, 0.0, 0.0,
                        -size, -size, -size, 0.0, 0.5,

                        -size, size, -size, 0.5, 0.5,
                        size, size, -size, 1.0, 0.5,
                        size, size, size, 1.0, 0.0,
                        size, size, size, 1.0, 0.0,
                        -size, size, size, 0.5, 0.0,
                        -size, size, -size, 0.5, 0.5,
                    ];
                    for ii in 0..vertices.len()/5 {
                        
                            
                            let v = Vertex{ 
                                position: vec3(
                                    vertices[ii*5+0] ,
                                    vertices[ii*5+1] ,
                                    vertices[ii*5+2] ),
                                uv: vec2(
                                     vertices[ii*5+3],
                                     vertices[ii*5+4]) ,
                                color: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }
                            };
                                                       
                            
                                        
                            my_cube_mesh.vertices.push(v);
                            my_cube_mesh.indices.push( ii as u16);

                        }

    
    return my_cube_mesh                 
    
}



#[allow(dead_code)]
pub(crate) fn draw_polygon(x: f32, y: f32, z: f32 ,  points: Vec<Vec2>, color: Color) {
    let points_length = points.len();
    let mut vertices = Vec::<macroquad::models::Vertex>::with_capacity(points_length as usize + 2);
    let mut indices = Vec::<u16>::with_capacity(points_length as usize * 3);

    for (i, point) in points.iter().enumerate() {
        let vertex = macroquad::models::Vertex {
            position: Vec3::new(x + point.x, y + point.y, z),
            uv: Vec2::default(),
            color
        };

        vertices.push(vertex);
        indices.extend_from_slice(&[0, i as u16 + 1, i as u16 + 2]);
    }

    let mesh = Mesh {
        vertices,
        indices,
        texture: None,
    };

    draw_mesh(&mesh);
}