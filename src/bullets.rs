use macroquad::{prelude::*, audio::{Sound, self}};
use crate::{common::{ObjectInSpace,  get_file_name_across_platforms, SpaceObject}, useful_objects::make_cube_in_code, asteroids::Asteroids, aliens::Aliens};

pub(crate) struct Bullet {
    translation: Vec3,
    direction: Vec3,
    ttl: f32,
    speed: f32,
}
pub(crate) struct Hit {
    pub position:Vec3,
    pub what:SpaceObject,
    pub hit_info:usize,
}

pub(crate) struct Bullets {
    mesh: Mesh,
    instances: Vec<Bullet>,
    //#[cfg(feature="audio")]
    sound:Sound,
}

impl Bullets {
    pub(crate) async fn new() -> Bullets {
        let cube = make_cube_in_code("rust.png", 1.0).await;

        //#[cfg(feature = "audio")]
        let fire = audio::load_sound(&get_file_name_across_platforms("fire.wav")).await.unwrap();

        return Bullets {
            mesh: cube,
            instances: vec![],
            //#[cfg(all(feature = "audio"))]
            sound:fire
        };
    }

    pub(crate) fn render(&mut self, material: &Material, delta: f32) {
        let mut delete = vec![];
        for i in (0..self.instances.len()).rev() {
            self.instances[i].render(material, &self.mesh, delta);
            self.instances[i].ttl = self.instances[i].ttl - delta;
            if self.instances[i].ttl <= 0.0 {
                delete.push(i);
            }
        }
        for i in delete {
            self.instances.remove(i);
        }
    }

    pub(crate) fn fire(&mut self, mut position: Vec3, direction: Vec3) {
        //#[cfg(feature = "audio")]
        audio::play_sound_once(&self.sound);

        position = position - direction; // in case close by

        self.instances.push(Bullet {
            translation: position,
            direction,
            ttl: 1.0,
            speed: 200.0,
        });
    }
    pub(crate) fn hit(&mut self, asteroids:&mut Asteroids,aliens:&mut Aliens) ->Vec<Hit>  {
        let mut explosion_here = vec![];

        for b in self.instances.iter_mut() {
            for i in 0..asteroids.instances.len() {
                if asteroids.instances[i].translation.distance(b.translation) < asteroids.instances[i].get_radius() {
                    explosion_here.push(Hit{
                        what:SpaceObject::ASTEROID,
                        position:asteroids.instances[i].translation,
                        hit_info:asteroids.instances[i].scale,
                    });
                    asteroids.instances[i].dead();
                    b.ttl = 0.0;
                }
            }
            for i in aliens.instances.iter_mut() {
                if b.ttl > 0.0 {
                    if i.translation.distance(b.translation) < i.get_radius() {
                        explosion_here.push(Hit{
                            what:SpaceObject::ALIEN,
                            position:i.translation,
                            hit_info:1,
                        });
                        i.dead();
                        b.ttl = 0.0;
                    }
                }               
            }
        }
        return explosion_here  
    }
}

impl Bullet {
    fn render(&mut self, material: &Material, mesh: &Mesh, delta: f32) {
        self.translation = self.translation + self.direction * delta * self.speed;

        let combined = glam::Mat4::from_translation(self.translation);

        material.set_uniform("Move", combined);


        gl_use_material(&material);

        
        draw_mesh(mesh);
    }
}
