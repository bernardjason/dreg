use crate::common::{SpaceObject, MAX_X, MAX_Y, MAX_Z, ObjectInSpace};
use crate::shaders;
use crate::{
    aliens, asteroids,
    bullets::{self, Hit},
    explosions, instruments, stars, ScreenTouch,
};
use macroquad::prelude::*;

const MOVE_SPEED: f32 = 5.0;
#[cfg(target_os = "android")]
const LOOK_SPEED: f32 = 50.0;
#[cfg(not(target_os = "android",))]
const LOOK_SPEED: f32 = 100.0;

pub(crate) struct Game {
    pitch: f32,
    roll: f32,
    applied_camera: glam::Mat4,
    front: Vec3,
    up: Vec3,
    position: Vec3,
    material: Material,
    stars: stars::Stars,
    asteroids: asteroids::Asteroids,
    aliens: aliens::Aliens,
    instruments: instruments::Instruments,
    bullets: bullets::Bullets,
    explosions: explosions::Explosions,
    pub last_shoot: f32,
    pub score:i32,
    pub sheilds:f32,
}

impl Game {
    pub(crate) async fn new() -> Game {
        let pipeline_params = PipelineParams {
            depth_write: true,
            depth_test: Comparison::LessOrEqual,
            ..Default::default()
        };

        let stars= stars::Stars::new();
        let asteroids= asteroids::Asteroids::new();
        let aliens= aliens::Aliens::new();        
        let bullets = bullets::Bullets::new();
        let explosions= explosions::Explosions::new();

        Game {
            score:0,
            sheilds:100.0,
            pitch: 0.0,
            roll: 0.0,
            applied_camera: glam::Mat4::from_translation(vec3(0.0, 0.0, 0.0)),
            front: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 0.0, 0.0),
            position: vec3(-15.0, 1.0, 2.0),
            material: load_material(
                ShaderSource {
                    glsl_vertex: Some(&shaders::DEFAULT_VERTEX_SHADER.to_string()),
                    glsl_fragment: Some(&shaders::DEFAULT_FRAGMENT_SHADER.to_string()),
                    metal_shader: None,
                },
                MaterialParams {
                    uniforms: vec![("Move".to_string(), UniformType::Mat4) , ],
                    pipeline_params,
                    ..Default::default()
                },
            ).unwrap(),
            stars: stars.await,
            asteroids: asteroids.await,
            aliens: aliens.await,
            instruments: instruments::Instruments::new(),
            bullets: bullets.await,
            explosions: explosions.await,
            last_shoot: 0.0,
        }
    }

    pub(crate) fn render(
        &mut self,
        delta: f32,
        fps: i128,
        frame: i128,
        do_roll: f32,
        do_pitch: f32,
    ) {
        clear_background(BLACK);

        if self.asteroids.instances.len() == 0 {
            self.asteroids.randoms();
        }
        if self.aliens.instances.len() == 0 {
            self.aliens.lots_of_aliens();
        }

        set_camera(&Camera3D {
            position: self.position,
            up: self.up,
            target: self.position + self.front,
            ..Default::default()
        });

        self.stars.render(do_roll, do_pitch);

        gl_use_default_material();

        draw_grid( MAX_X as u32 /2, 5.0, GRAY, GREEN);

        
        draw_cube_wires(vec3(0., 1., -MAX_Z/3.), vec3(2., 2., 2.), PINK);
        draw_cube_wires(vec3(0., 1., -MAX_Z/8.), vec3(2., 2., 2.), GREEN);
        draw_cube_wires(vec3(0., 1., MAX_Z/5.), vec3(2., 2., 2.), BLUE);
        draw_cube_wires(vec3(0., 1., MAX_Z/8.), vec3(2., 2., 2.), YELLOW);

        draw_cube_wires(vec3(0., MAX_Y/10.0, 0.0), vec3(2., 2., 2.), PINK);
        draw_cube_wires(vec3(0., MAX_Y/4., 0.0), vec3(2., 2., 2.), GREEN);
        draw_cube_wires(vec3(0., -MAX_Y/5., 0.0), vec3(2., 2., 2.), BLUE);
        draw_cube_wires(vec3(0., -MAX_Y/7., 0.0), vec3(2., 2., 2.), YELLOW);
        

        gl_use_material(&self.material);

        self.aliens.render(&self.material,delta);

        self.asteroids.render(&self.material,delta);

        self.bullets.render(&self.material, delta);
        self.explosions.render(&self.material, delta);

        let explosion_here: Vec<Hit>;
        (explosion_here) = self.bullets.hit(&mut self.asteroids, &mut self.aliens);

        for i in explosion_here {
            self.explosions.create(i.position);
            if i.what == SpaceObject::ASTEROID {
                self.asteroids.asteroid_hit(i.position, i.hit_info);
                self.score=self.score+1
            } else {
                self.score=self.score+10
            }
        }

        let explosion_here: Vec<Hit>;
        (explosion_here) = self.aliens.alien_collide_asteroid(&mut self.asteroids);

        for i in explosion_here {
            self.explosions.create(i.position);
            self.asteroids.asteroid_hit(i.position, i.hit_info);            
        }

        for i in self.aliens.instances.iter_mut() {
            if i.translation.distance(self.position) < i.get_radius() * 2.0  {
                self.sheilds=self.sheilds - 5.;
                self.explosions.create(i.translation);
                i.dead();
            }
        }
        for i in self.asteroids.instances.iter_mut() {
            if i.translation.distance(self.position) < i.get_radius() * 1.5 {
                self.sheilds=self.sheilds - 20.;
                self.explosions.create(i.translation);
                i.dead();
            }
        }
        

        self.aliens.change_direction();

        gl_use_default_material();

        set_default_camera();

        let height = screen_height()* 0.04;
        draw_text(
            format!("Score:{}",self.score,).as_str(),
            10.0,
            height*2.0,
            height,
            WHITE,
        );
        draw_text(
            format!("Shields:{:.1}",self.sheilds,).as_str(),
            10.0,
            height*3.0,
            height,
            WHITE,
        );
        draw_text(
            format!(
                "fps:{} aliens:{}",fps / frame,self.aliens.instances.len()
            )
            .as_str(),
            10.0,
            height*4.0,
            height,
            WHITE,
        );
 

        self.instruments.update(
            self.position,
            self.applied_camera,
            self.asteroids.get_all_asteroids(),
            self.aliens.get_all_aliens(),
        );
        self.instruments.render();
    }

    pub fn player_input_and_move(&mut self, delta: f32, screen_touch: &ScreenTouch) -> (f32, f32) {
        self.last_shoot = self.last_shoot - delta;
        let mut do_roll: f32 = 0.0;
        let mut do_pitch: f32 = 0.0;

        if is_key_down(KeyCode::Space) && self.last_shoot <= 0.0 || screen_touch.fire == true {
            self.bullets.fire(self.position, self.front);
            self.last_shoot = 0.5;
        }

        if screen_touch.change_y > 0.0
            || is_key_down(KeyCode::Up) && !is_key_down(KeyCode::LeftShift)
        {
            do_pitch = -LOOK_SPEED * delta;
        }
        if screen_touch.change_y < 0.0
            || is_key_down(KeyCode::Down) && !is_key_down(KeyCode::LeftShift)
        {
            do_pitch = LOOK_SPEED * delta;
        }
        if is_key_down(KeyCode::Left) || screen_touch.change_x > 0.0 {
            do_roll = -LOOK_SPEED * delta;
        }
        if is_key_down(KeyCode::Right) || screen_touch.change_x < 0.0 {
            do_roll = LOOK_SPEED * delta;
        }

        if is_key_down(KeyCode::Up) && is_key_down(KeyCode::LeftShift) {
            self.position += self.front * MOVE_SPEED * delta;
        }
        if is_key_down(KeyCode::Down) && is_key_down(KeyCode::LeftShift) {
            self.position -= self.front * MOVE_SPEED * delta;
        }

        self.position += self.front * MOVE_SPEED * 0.5 * delta;

        if do_pitch != 0.0 {
            self.applied_camera = self.applied_camera
                * glam::Mat4::from_axis_angle(vec3(0.0, 0.0, 1.0), do_pitch.to_radians());
        }

        if do_roll != 0.0 {
            self.applied_camera = self.applied_camera
                * glam::Mat4::from_axis_angle(vec3(1.0, 0.0, 0.0), do_roll.to_radians());
        }

        self.pitch = add_degrees(self.pitch, do_pitch);
        self.roll = add_degrees(self.roll, do_roll);

        self.up = self.applied_camera.transform_vector3(vec3(0.0, 1.0, 0.0));
        self.front = self.applied_camera.transform_vector3(vec3(1.0, 0.0, 0.0));


        let future = self.position + self.front * MOVE_SPEED * delta;
        if future.x >= MAX_X {
            self.position.x = self.position.x * -1.0;
            self.position =self.position + self.front * MOVE_SPEED ;
        }
        if future.x <= -MAX_X {
            self.position.x = self.position.x * -1.0;
            self.position =self.position - self.front * MOVE_SPEED ;
        }
        if future.y >= MAX_Y {
            self.position.y = self.position.y * -1.0;
            self.position =self.position + self.front * MOVE_SPEED ;
        }
        if future.y <= -MAX_Y {
            self.position.y = self.position.y * -1.0;
            self.position =self.position - self.front * MOVE_SPEED ;
        }
        if future.z >= MAX_Z {
            self.position.z = self.position.z * -1.0;
            self.position =self.position + self.front * MOVE_SPEED ;
        }
        if future.z <= -MAX_Z {
            self.position.z = self.position.z * -1.0;
            self.position =self.position - self.front * MOVE_SPEED ;
        }

        (do_roll, do_pitch)
    }
}

fn add_degrees(original: f32, add: f32) -> f32 {
    let mut original: f32 = original + add;
    if original >= 360.0 {
        original = original - 360.0
    }
    if original < 0.0 {
        original = original + 360.0
    }

    return original;
}
