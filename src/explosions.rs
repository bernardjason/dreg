use macroquad::{prelude::*, audio::{self, Sound}};
use crate::{common::get_file_name_across_platforms, useful_objects::make_cube_in_code};

pub(crate) struct Explosion {
    translation: Vec3,
    direction: Vec3,
    ttl: f32,
    speed: f32,
}

pub(crate) struct Explosions {
    mesh: Mesh,
    instances: Vec<Explosion>,
    colours:Vec<Texture2D>,
    current:i128,
    //#[cfg(feature = "audio")]
    sound:Sound,
}

impl Explosions {
    pub(crate) async fn new() -> Explosions {
        let cube = make_cube_in_code("rust.png", 0.10).await;

        //#[cfg(feature="audio")]
        let hit = audio::load_sound(&get_file_name_across_platforms("hit.wav")).await.unwrap();


        let mut colours = vec![];
        for i in vec![ PURPLE,RED,BLUE,ORANGE] {
            let image = Image::gen_image_color(8, 8, i);
            let texture = Texture2D::from_image(&image);
            colours.push(texture);
        }
        return Explosions {
            mesh: cube,
            instances: vec![],
            colours,
            current:0,
            //#[cfg(feature="audio")]
            sound:hit
        };
    }

    pub(crate) fn render(&mut self, material: &Material, delta: f32) {
        let mut delete = vec![];
        for i in (0..self.instances.len()).rev() {
            let colour = &self.colours[ (self.current % self.colours.len() as i128) as usize];
            self.mesh.texture = Some(colour.clone());
            
            self.instances[i].render(material, &self.mesh, delta);
            self.instances[i].ttl = self.instances[i].ttl - delta;
            if self.instances[i].ttl <= 0.0 {
                delete.push(i);
            }
            self.current=self.current+1;
        }
        for i in delete {
            self.instances.remove(i);
        }
    }

    pub(crate) fn create(&mut self, position: Vec3) {
        
        //#[cfg(feature="audio")]
        audio::play_sound_once(&self.sound);


        for x in -1..2 {
            for y in -1..2 {
                for z in -1..2 {
                    let direction = vec3(x as f32, y as f32, z as f32);
                    self.instances.push(Explosion {
                        translation: position,
                        direction,
                        ttl: crate::rand::gen_range(25,100) as f32 / 100.0,
                        speed: crate::rand::gen_range(200,500) as f32 / 100.0,
                    });

                }
            }
        }

    }
   
}

impl Explosion {
    fn render(&mut self, material: &Material, mesh: &Mesh, delta: f32) {
        self.translation = self.translation + self.direction * delta * self.speed;

        let combined = glam::Mat4::from_translation(self.translation);

        material.set_uniform("Move", combined);

        gl_use_material(&material);
        
        draw_mesh(mesh);
    }
}
