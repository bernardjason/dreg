use std::path::Path;
use macroquad::prelude::*;

use macroquad::prelude::{Vec3, Material, Mesh};


pub(crate) const MAX_X:f32 = 60.0;
pub(crate) const MAX_Y:f32 = 60.0;
pub(crate) const MAX_Z:f32 = 60.0;


#[derive(PartialEq)]
pub(crate) enum SpaceObject {
    ASTEROID,
    ALIEN
}
pub(crate) trait ObjectInSpace {
    fn get_position_and_type(&self) -> (Vec3,SpaceObject);

    fn get_radius(&self) ->f32 ;

    fn get_rotation_translation(&self) -> (&Vec3,&Vec3);

    fn do_updates(&mut self,delta:f32);

    fn dead(&mut self);

    fn get_index(&self) -> usize;

    fn render(&mut self,material:&Material,mesh:&Mesh,delta:f32) {
        
        self.do_updates(delta);

        let (rotation,translation) = self.get_rotation_translation();
        let rotation_x = glam::Quat::from_rotation_x(rotation.x);
        let rotation_y = glam::Quat::from_rotation_y(rotation.y);
        let rotation_z = glam::Quat::from_rotation_z(rotation.z);
        
        
        let combined =  glam::Mat4::from_translation(*translation )  * glam::Mat4::from_quat(rotation_x) 
                             *glam::Mat4::from_quat(rotation_y) *glam::Mat4::from_quat(rotation_z);
                                        
        
        material.set_uniform("Move", combined );
       
        gl_use_material(&material);    

        /*
        if self.get_scale() == 1.0 {
            draw_mesh(mesh)
        } else {
            let mut my_obj_mesh = Mesh {
                vertices: vec![],
                indices: mesh.indices.clone(),
                texture: mesh.texture.clone()
            };
            for i in 0..mesh.vertices.len() {
                let mut s = mesh.vertices[i];
                s.position = s.position * self.get_scale();
                my_obj_mesh.vertices.push(s);
            }
            draw_mesh(&my_obj_mesh)
        }
        */
        draw_mesh(mesh);
        
        
    }

}

pub (crate) fn get_file_name_across_platforms(texture_fn: &str) -> String {
    let file_name = if Path::new("assets").exists() {
        format!("assets/{}",texture_fn)
    } else {
        format!("{}",texture_fn)
    };
    file_name
}

pub(crate) fn norm(x:f32) ->f32 {
    x * 1.0/x.abs()
}