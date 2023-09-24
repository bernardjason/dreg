
use std::time::{SystemTime, UNIX_EPOCH};

use game::Game;
use macroquad::prelude::*;

use crate::common::norm;
mod useful_objects;
mod shaders;
mod asteroids;
mod stars;
mod instruments;
mod common;
mod object_file;
mod aliens;
mod bullets;
mod explosions;
mod game;

const DOUBLE_TAP:i32 = 40;

#[derive(Default)]
pub(crate) struct ScreenTouch {
    x:f32,
    y:f32,
    change_x :f32,
    change_y :f32,
    frame:i128,
    fire:bool
}

#[cfg(target_arch = "wasm32")]
fn gameconf() -> Conf {
    Conf {
        window_title: String::from("Macroquad"),
        window_width: 700,//1260,
        window_height: 700,//768,
        fullscreen: false,
        ..Default::default()
    }
}

#[cfg(target_os = "android")]
fn gameconf() -> Conf {
    Conf {
        window_width: 1000,//1260,
        window_height: 1000,//768,
        high_dpi: true,
        window_resizable: true,
        fullscreen: false,
        ..Default::default()
    }
}
#[cfg(not(any(target_arch = "wasm32", target_os = "android",)))]
fn gameconf() -> Conf {
    Conf {
        window_title: String::from("Macroquad"),
        window_width: 1080,
        window_height: 800,
        fullscreen: false,
        ..Default::default()
    }      
}

fn touch(mut screen_touch:ScreenTouch,frame:i128) -> ScreenTouch{

    let sensitivity = 150;
    let t = touches();
    
    
    #[cfg(not (target_arch = "wasm32"))]
    let now = now();
    #[cfg(target_arch = "wasm32")]
    let now:u128=0;
    
    if t.len() > 0 {
        
        for touch in t {
            
            let t_x = (touch.position.x as i32 / sensitivity) * sensitivity;
            let t_y = (touch.position.y as i32 / sensitivity) * sensitivity;
                        
            if touch.phase == TouchPhase::Started {
                screen_touch.x = t_x as f32;
                screen_touch.y = t_y as f32;

                if frame -  screen_touch.frame < DOUBLE_TAP as i128 {
                    screen_touch.fire=true;    
                }
                debug!("!!!! TAP screen_touch.frame={} frame={}",screen_touch.frame,frame);
                screen_touch.frame = frame;
            }
            if touch.phase == TouchPhase::Cancelled || touch.phase == TouchPhase::Ended {
                screen_touch.change_x = 0.0;
                screen_touch.change_y = 0.0;
                debug!("{} TOUCHES CANCEL/ENDED !!!!!!!!!!!!!!!",now);  
            }
            if touch.phase == TouchPhase::Moved && (screen_touch.x as i32 != t_x || screen_touch.y as i32 != t_y)  {
                screen_touch.change_x = norm( screen_touch.x - t_x as f32 );
                screen_touch.change_y = norm( screen_touch.y - t_y as f32  );
                screen_touch.x = t_x as f32;
                screen_touch.y = t_y as f32;
            }
            debug!("{} TOUCHES t_x={}, t_y={}   screen_touch.change_x={}  screen_touch.change_y={}",now,t_x,t_y,screen_touch.change_x,screen_touch.change_y)
        }
        
    } else {
        screen_touch.change_x = 0.0;
        screen_touch.change_y = 0.0;   
    }
    
    return screen_touch;

}

fn now() -> u128 {
    let since_the_epoch = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .expect("Time went backwards");
    let now = since_the_epoch.as_millis();
    now
}



#[macroquad::main(gameconf)]
async fn main() {

  
    // want to ugnore config value
    #[cfg(target_os = "android")]
    set_fullscreen(true);
    

    set_cursor_grab(false);
    show_mouse(false);


    let mut game = Game::new().await;
  
    let mut frame: i128 = 0;
    let mut screen_touch :ScreenTouch= Default::default();

    let mut screen = 0;

    let mut fps: i128 = 0;

    let background = common::get_file_name_across_platforms("background.png");
    let background_texture = load_texture(&background).await.unwrap();

    let welcome = vec![ "Press space or", "double tap", "to play", "", "Arrow keys &","Space fire", "or touch screen and","double tap mobile"];
    let end = vec!["Game over","Score was"];

    loop {

        frame=frame+1;
        fps = fps + get_fps() as i128;
        screen_touch.fire = false;

        let delta = get_frame_time();
     

        screen_touch = touch(screen_touch,frame);
    
        if is_key_pressed(KeyCode::Escape) {
            if screen == 2 {
                #[cfg(not(target_arch = "wasm32"))]
                break
            }
            screen = 2;
        }
 
        if screen == 0 {
            
            if is_key_down(KeyCode::Space) || screen_touch.fire {
                screen=screen+1;
                game.last_shoot=0.5;
            }

            clear_background(BLACK);

            gl_use_default_material();

            draw_texture(&background_texture,0.,100.,WHITE);
            
            let mut y = 356.0;
            for i in welcome.iter() {
                draw_text(
                    i,
                    20.0,
                    y,
                    screen_height()* 0.05,
                    WHITE,
                );
                y = y + screen_height()* 0.06;
            }            
        } else if screen == 2 {
            clear_background(BLACK);

            gl_use_default_material();

            draw_texture(&background_texture,0.,100.,WHITE);
            
            let mut y = screen_height()*0.4;

            for i in end.iter() {
                draw_text(
                    i,
                    20.0,
                    y,
                    screen_height()* 0.05,
                    WHITE,
                );
                y = y + screen_height()* 0.08;
            }
            draw_text(
                format!("{}",game.score).as_str(),
                20.0,
                y,
                screen_height()* 0.05,
                WHITE,
            );
            
        } else {
            clear_background(BLACK);

            let (do_roll,do_pitch) = game.player_input_and_move(delta,&screen_touch);

            game.render(delta, fps, frame,do_roll,do_pitch);

            if game.sheilds <= 0.0 {
                screen = 2
            }
        }

        next_frame().await
    }
}

