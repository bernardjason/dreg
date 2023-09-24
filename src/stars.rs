use macroquad::{prelude::{Mesh, Material, gl_use_material, draw_mesh, load_material, ShaderSource, MaterialParams, PipelineParams, Comparison}, window::screen_height};

use crate::{useful_objects::make_cube_in_code, shaders};


pub(crate) struct Stars {
    y:f32,
    roll:f32,
    pitch:f32,    
    mesh:Mesh,
    material:Material,
}

impl Stars {

    pub async fn new() -> Stars {

        
        let mesh = make_cube_in_code("stars.png", 200.0).await;

        let pipeline_params = PipelineParams {
            depth_write: true,
            depth_test: Comparison::LessOrEqual,
            ..Default::default()
        };
        let fragment_shader = shaders::STARS_FRAGMENT_SHADER.to_string();
        let vertex_shader = shaders::STARS_VERTEX_SHADER.to_string();
        
        let material = load_material(
            ShaderSource {
                glsl_vertex: Some(&vertex_shader),
                glsl_fragment: Some(&fragment_shader),
                metal_shader: None,
            },
            MaterialParams {
                pipeline_params,
                ..Default::default()
            },
        )
        .unwrap();
    
        Stars {
            y:0.0,
            roll:0.0,
            pitch:0.0,
            mesh,
            material
        }

    }

    pub fn render(&mut self,roll:f32,pitch:f32,) {
        self.roll = self.roll - roll;
        self.pitch = self.pitch - pitch;
        self.y=self.y + pitch* 10.0;

        if self.y < -screen_height() {
            self.y  = self.y + screen_height()*2.0;
        }
        if self.y > screen_height() {
            self.y  = self.y - screen_height()*2.0;
        }

        gl_use_material(&self.material);
     

        draw_mesh(&self.mesh);

    }
    
}
