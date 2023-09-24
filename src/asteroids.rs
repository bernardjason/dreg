use macroquad::prelude::*;

use crate::object_file::load_object_file;

use crate::common::{ObjectInSpace, SpaceObject, MAX_X, MAX_Y, MAX_Z};

pub(crate) struct Asteroid {
    pub translation:Vec3,
    pub direction:Vec3,
    rotation:Vec3,
    pub scale:usize,
    pub ttl:f32
}

pub(crate) struct Asteroids {
    mesh:Vec<Mesh>,
    pub instances:Vec<Asteroid>
}

const SPEED:f32 = 50.0;

impl ObjectInSpace for Asteroid{
    fn get_position_and_type(&self) -> (Vec3,SpaceObject) {
        return (self.translation,SpaceObject::ASTEROID);
    }

    fn get_radius(&self) ->f32  {
        SCALES[self.scale] * 7.0
    }
    fn get_rotation_translation(&self) -> (&Vec3,&Vec3) {
        return (&self.rotation,&self.translation)
    }

    fn do_updates(&mut self,delta:f32) {
        let future = self.translation + self.direction * delta * SPEED * 2.0;
        if future.x.abs() >= MAX_X {
            self.translation.x = self.translation.x * -1.0;
        }
        if future.y.abs() >= MAX_Y {
            self.translation.y = self.translation.y * -1.0;
        }
        if future.z.abs() >= MAX_Z {
            self.translation.z = self.translation.z * -1.0;
        }
        self.translation = self.translation + self.direction * delta * SPEED;
        self.rotation = self.rotation + self.direction * delta;


    }

    fn dead(&mut self) {
        self.ttl=0.0;           
    }

    fn get_index(&self) -> usize {
        self.scale
    }
}

fn pos_neg() ->f32 {    
    if crate::rand::gen_range(1,3) == 1 {
        -1.0
    } else {
        1.0
    }
}

const SCALES:[f32;3] = [1.0 as f32 , 0.6 ,0.4];

impl Asteroids{


    pub(crate) async fn new() -> Asteroids {
                
        let mut asteroids =  Asteroids { 
             mesh:vec![],
             instances: vec![] }       ;


        let mesh = load_object_file("block_rock.obj","asteroid_render1.png",0.2).await.unwrap();
        

        for scale in SCALES.iter() {
            
            let mut my_obj_mesh = Mesh {
                vertices: vec![],
                indices: mesh.indices.clone(),
                texture: mesh.texture.clone(),
            };
            for i in 0..mesh.vertices.len() {
                let mut s = mesh.vertices[i];
                s.position = s.position * *scale;
                my_obj_mesh.vertices.push(s);
            }
            asteroids.mesh.push(my_obj_mesh);
        }

 
    
        asteroids
    }
    pub(crate) fn asteroid_hit(&mut self,position:Vec3,mut scale: usize)  {
        scale = scale +1;
        if scale < self.mesh.len()  {
            for _i in 0..2 {
                self.new_asteroid(position, scale );
            }
        }
    }

    pub(crate) fn randoms(&mut self) {
        for _i in 0..20 {
            let x:f32 = crate::rand::gen_range(20.0,MAX_X * 0.85) as f32 * pos_neg();
            let y:f32 = crate::rand::gen_range(20.0,MAX_Y * 0.85) as f32* pos_neg();
            let z:f32 = crate::rand::gen_range(20.0,MAX_Z * 0.85) as f32* pos_neg();
            self.new_asteroid(vec3(x, y, z), 0);
            if self.instances.len() > 20 {
                break
            }
        }
    }

    fn new_asteroid(&mut self, position: Vec3, scale: usize) {
        let x:f32 = 1.0 / crate::rand::gen_range(80.,200.)  * pos_neg();
        let y:f32 = 1.0 / crate::rand::gen_range(80.,200.) * pos_neg();
        let z:f32 = 1.0 / crate::rand::gen_range(80.,200.) * pos_neg();
        let direction = vec3(x, y, z);
        self.instances.push( Asteroid{
            translation:position + (direction * 100.),
            direction,
            rotation:vec3(x,y,z,),
            ttl:1.0,
            scale:scale,
        });
    }

    pub(crate) fn get_all_asteroids(&self) -> Vec<&Asteroid> {
        let mut all = vec![];
        for i in self.instances.iter() {
            all.push(i)
        }
        return all;
    }

    pub(crate) fn render(&mut self,material:&Material,delta:f32) {
        for i in (0..self.instances.len()).rev() {
            let mesh = self.instances[i].scale;
            
            self.instances[i].render(material,&self.mesh[mesh],delta);
            if self.instances[i].ttl <= 0.0 {
                self.instances.remove(i);
            }
        }
    }
    
}





