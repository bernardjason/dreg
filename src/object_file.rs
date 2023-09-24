use std::io::BufReader;
use obj::{load_obj,Obj, TexturedVertex};
use macroquad::prelude::*;
use macroquad::{models::Vertex, prelude::Mesh};
use std::io::Cursor;
use crate::common::get_file_name_across_platforms;

pub(crate) async fn load_object_file(
    object_filename: &str,
    image_filename: &str,
    scale: f32,
) -> Result<Mesh, String> {

    let actual_image_filename = get_file_name_across_platforms(image_filename);
    let actual_object_filename = get_file_name_across_platforms(object_filename);

    let read_file = load_file(&actual_object_filename).await.unwrap();

    let c = Cursor::new(read_file);

    let input: BufReader<Cursor<Vec<u8>>> = BufReader::new(c);
                
    let mut my_obj_mesh = Mesh {
        vertices: vec![],
        indices: vec![],
        texture: Some(load_texture(actual_image_filename.as_str()).await.unwrap()),
    };

    load_object(input, scale, &mut my_obj_mesh);


    return Ok(my_obj_mesh);
}

pub(crate) fn load_object(input: BufReader<Cursor<Vec<u8>>>, scale: f32, my_obj_mesh: &mut Mesh) {
    let mut kk = 0;
    let parsed_object: Obj<TexturedVertex> = load_obj(input).unwrap();

    for i in 0..parsed_object.vertices.len() {

        let xyz = &parsed_object.vertices[i].position;
        let tex = &parsed_object.vertices[i].texture;
    
        let v = Vertex {
            position: vec3(
                xyz[0] * scale,
                xyz[1] * scale,
                xyz[2] * scale,
            ),
            uv: vec2(tex[0],1.0-tex[1]),
            color: Color {r: 1.0,g: 1.0, b: 1.0,a: 1.0,},
        };
        //println!("{}   {}",v.position,v.uv);
        my_obj_mesh.vertices.push(v);
        my_obj_mesh.indices.push(kk); 
                                     
        kk = kk + 1;
    
    }
}


/*
pub(crate) async fn xxload_obj(
    object_filename: &str,
    image_filename: &str,
    scale: f32,
) -> Result<Mesh, String> {

    let actual_image_filename = get_file_name_across_platforms(image_filename);
    let actual_object_filename = get_file_name_across_platforms(object_filename);

    let cornell_box = tobj::load_obj(
        &actual_object_filename,
        &tobj::LoadOptions {
            single_index: true,
            ..Default::default()
        },
    );


    assert!(cornell_box.is_ok());
    let (models, _materials) = cornell_box.expect("Failed to load OBJ file");

    for (i, m) in models.iter().enumerate() {
        let mesh = &m.mesh;

        println!("model[{}].name = \'{}\'", i, m.name);

        // Normals and texture coordinates are also loaded, but not printed in this example
        println!("model[{}].vertices: {}", i, mesh.positions.len() / 3);

        assert!(mesh.positions.len() % 3 == 0);
    }

    let mut my_obj_mesh = Mesh {
        vertices: vec![],
        indices: vec![],
        texture: Some(load_texture(actual_image_filename.as_str()).await.unwrap()),
    };

    let mut kk = 0;

    for m in 0..models.len() {
        let i = &models[m];

        for ii in 0..i.mesh.positions.len() / 3 {
            let texcoords = if i.mesh.texcoords.len() > 0 {
                vec2(
                    i.mesh.texcoords[ii * 2 + 0],
                    1.0 - i.mesh.texcoords[ii * 2 + 1],
                )
            } else {
                vec2(0.0,0.0)                
            };
            let v = Vertex {
                position: vec3(
                    i.mesh.positions[ii * 3 + 0] * scale,
                    i.mesh.positions[ii * 3 + 1] * scale,
                    i.mesh.positions[ii * 3 + 2] * scale,
                ),
                uv: texcoords,
                color: Color {r: 1.0,g: 1.0, b: 1.0,a: 1.0,},
            };

            my_obj_mesh.vertices.push(v);

            my_obj_mesh.indices.push(kk); 
                                         
            kk = kk + 1;
        }
    }

    return Ok(my_obj_mesh);
}
*/