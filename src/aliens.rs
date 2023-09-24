use macroquad::prelude::*;
use crate::asteroids::Asteroids;
use crate::bullets::Hit;
use crate::object_file::load_object_file;
use crate::common::{ObjectInSpace, SpaceObject, MAX_X, MAX_Z};

pub(crate) struct Alien {
    pub translation:Vec3,
    pub direction:Vec3,
    rotation:Vec3,
    pub ttl:f32,
}


pub(crate) struct Aliens {
    mesh:Mesh,
    pub instances:Vec<Alien>
}

impl ObjectInSpace for Alien{
    fn get_position_and_type(&self) -> (Vec3,SpaceObject) {
        return (self.translation,SpaceObject::ALIEN);
    }

    fn get_radius(&self) ->f32  {
        2.5
    }

    fn get_rotation_translation(&self) -> (&Vec3,&Vec3) {
        return (&self.rotation,&self.translation)
    }

    fn do_updates(&mut self,delta:f32) {
        self.translation = self.translation + self.direction * delta * 10.0;
        self.rotation.y = self.rotation.y + 1.0 * delta ;

    }
    
    fn dead(&mut self) {
        self.ttl=0.0;        
    }

    fn get_index(&self) -> usize {
        1
    }


}


impl Aliens{


    pub(crate) async fn new() -> Aliens {

        return Aliens { 
            mesh:load_object_file("anotheralien.obj","anotheralien.png",0.1).await.unwrap() ,
             instances: vec![] }
       
       
    }

    pub(crate) fn lots_of_aliens(&mut self) {
        let space = 40;
        let between = 25;
        for y in (-space..space).step_by(between) {                      
            for x in (-space..space-20).step_by(between) {
                for z in (-space..space-20).step_by(between) {
                
                    self.instances.push( Alien{
                        translation:vec3(x as f32,y as f32,z as f32),
                        rotation:vec3(0.0, 0.0, 0.0),
                        direction:vec3(1.0,0.0,0.0),
                        ttl:1.0
                    });
           
                }
            }
        }
    }
    pub(crate) fn change_direction(&mut self) {
        if self.instances.len() == 0 {
            return
        }

        let mut direction_change = self.instances[0].direction;
        let mut changed = false;
        
        for i in self.instances.iter() {
            let new_pos = i.translation+direction_change;
            if new_pos.x > MAX_X {
                direction_change.x = 0.0;
                direction_change.z = 1.0;
                changed=true;
                break;
            }
            if new_pos.z > MAX_Z {
                direction_change.x = -1.0;
                direction_change.z = 0.0;
                changed=true;
                break;
            }
            if new_pos.x < -MAX_X {
                direction_change.x = 0.0;
                direction_change.z = -1.0;
                changed=true;
                break;
            }
            if new_pos.z < -MAX_Z {
                direction_change.x = 1.0;
                direction_change.z = 0.0;
                changed=true;
                break;
            }
        }
        if changed {
            for i in self.instances.iter_mut() {
                i.direction = direction_change;
                
            }
        }
    }

    pub(crate) fn get_all_aliens(&self) -> Vec<&Alien> {
        let mut all = vec![];
        for i in self.instances.iter() {
            all.push(i)
        }
        return all;
    }
    pub(crate) fn alien_collide_asteroid(&mut self, asteroids:&mut Asteroids) ->Vec<Hit>  {
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
                    b.dead();
                }
            }
            
        }
        return explosion_here  
    }


    pub(crate) fn render(&mut self,material:&Material,delta:f32) {
        for i in (0..self.instances.len()).rev() {
            self.instances[i].render(material,&self.mesh,delta);
            if self.instances[i].ttl <= 0.0 {
                self.instances.remove(i);
            }
        }
    }
}




