use macroquad::{shapes::{ draw_line, draw_rectangle}, prelude::{ YELLOW,RED, Mat4, Vec2, Color},   };
use crate::{asteroids::Asteroid, common::ObjectInSpace, aliens::Alien};
use crate::glam::f32::Vec3;
use macroquad::prelude::*;

struct Blip {
    xy:Vec2,
    colour:Color,
    height:f32,
}
pub(crate)struct Instruments {
    x : f32,
    y:f32,
    radius:f32,
    blips:Vec<Blip>,
    instrument_height:f32,

}

impl Instruments {

    pub(crate) fn new() -> Instruments {
        
        Instruments{
            x:screen_width()/2.0,
            y:screen_height()-screen_width()/4.0 ,
            radius:screen_width()/4.0,
            blips:vec![],
            instrument_height:screen_height(),
        }
    }

    pub(crate) fn update(&mut self,player_position:Vec3,applied_camera:Mat4,space_objects:Vec<&Asteroid>,alien_objects:Vec<&Alien>) {
        self.instrument_height = screen_height() / 4.0;
        if self.instrument_height > 450.0 {
            self.instrument_height = 450.0;
        }

        self.x = screen_width()/2.0;
        self.y = screen_height() - self.instrument_height/2.0;
        self.radius = screen_width()/2.0;

        self.blips.clear();

        let scale_xy = 2.0;
        let off_screen = 50.0;

        let up = applied_camera.transform_vector3(vec3(0.0, 1.0, 0.0));
        let (_out_vec,out_quat,_out_what) = applied_camera.to_scale_rotation_translation();
            
        for o in space_objects.iter() {
            self.plot_object(o, player_position, out_quat, scale_xy, up, off_screen,YELLOW,GREEN);                       
        }
        for o in alien_objects.iter() {
            self.plot_object(o, player_position, out_quat, scale_xy, up, off_screen,PURPLE,PINK);                       
        }


    }

    fn plot_object<T:ObjectInSpace>(&mut self, o: &&T, player_position: Vec3, out_quat: Quat, scale_xy: f32, up: Vec3, off_screen: f32,up_colour:Color,down_colour:Color) {
        let (position,_what) = o.get_position_and_type();
        let distance = position.distance(player_position);
            
        let position_diff= position - player_position;
            
        let rotated_by_camera = out_quat.inverse().mul_vec3(position_diff) * scale_xy;
            
        let mut height = (position_diff.angle_between(up).to_degrees() -90.0).abs();

        if height <= 2.0 {
            height = 2.0;
        }
                        
        let a:f32 = -90.0f32.to_radians();
        let xx = rotated_by_camera.x * a.cos() + rotated_by_camera.z*a.sin();
        let zz = rotated_by_camera.z * a.cos() + rotated_by_camera.x*a.sin();
          
        let blip = Blip{
            xy:Vec2 { x: xx, y: zz },
            colour:if position.y < player_position.y {
                up_colour
            } else {
                down_colour
            },
            height:height,
        };
        if distance < off_screen {
            self.blips.push(blip);
        }
    }

    pub(crate) fn render(&mut self) {

        let (centre_x,centre_y) = (screen_width()/2.0,screen_height()-self.instrument_height/2.0);

        let background = Color::new(0.1, 0.1, 0.1, 1.0);
              
        let squash = self.radius / self.instrument_height*2.0;
        
        draw_rectangle(0.0, screen_height()- self.instrument_height, screen_width(), self.instrument_height, background);

        let sides = 20;
        let rot = 0.0;
        
        let mut y1= self.y;
        let mut x1 = -999999.0;
        for i in 0..=sides {
            let x2 = self.x + (i as f32 / sides as f32 * std::f32::consts::PI * 2. + rot).cos()* self.radius;
            let y2 = self.y + (i as f32 / sides as f32 * std::f32::consts::PI * 2. + rot).sin() *self.radius / squash;
            
            if x1 >= -screen_width() {
                draw_line(x1, y1, x2, y2, 4.0, RED);
            }
            
            x1 = x2;
            y1=y2;
        }
        draw_line(0.0, screen_height(), screen_width(), screen_height(), 3.0, YELLOW);
        draw_line(screen_width() , screen_height(), screen_width(), screen_height() - self.instrument_height , 3.0, YELLOW);
        draw_line(0.0, screen_height() - self.instrument_height, screen_width(), screen_height() - self.instrument_height , 3.0, YELLOW);
        draw_line(0.0 , screen_height(), 0.0, screen_height() - self.instrument_height , 3.0, YELLOW);

        for blip in self.blips.iter(){
            draw_line(blip.xy.x+centre_x, blip.xy.y+centre_y,
                      blip.xy.x+centre_x, blip.xy.y+blip.height+centre_y, 
                      3.0, blip.colour);
        }

        draw_rectangle(centre_x-2.0, centre_y - 2.0, 4.0,4.0, WHITE);


    }

   }
