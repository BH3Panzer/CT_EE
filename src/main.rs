use raylib::prelude::*;
mod ressources;
mod unit;
mod map;
mod camera;
mod ground_tile;
use crate::camera::handle_camera;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(0, 0)
        .title("ChronoTech: Evolution of Empires")
        .vsync()
        .build();

    if !rl.is_window_fullscreen() {
        rl.toggle_fullscreen();
    }

    let screen_width: i32 = rl.get_screen_width();
    let screen_height: i32 = rl.get_screen_height();

    let mut dt: f32;

    let mut debug_mode: bool = false;
    let mut camera: Camera3D = Camera3D::perspective(
        Vector3::new(0.0, 40.0, 10.0),
        Vector3::new(0.0, 0.0, -20.0),
        Vector3::new(0.0, 1.0, 0.0),
        80.0,
    );

    let mut dev_map: map::Map = map::Map::new(5.0);

    dev_map.dev_gen(&mut rl, &thread);
    
    while !rl.window_should_close() {
        dt = rl.get_frame_time();
        if rl.is_key_pressed(KeyboardKey::KEY_F1) {
            debug_mode = !debug_mode;
        }

        handle_camera(&mut camera, dt, &mut rl);

        let mut d = rl.begin_drawing(&thread);

        

        d.clear_background(Color::BLACK);

        let mut d3d: RaylibMode3D<'_, RaylibDrawHandle<'_>> = d.begin_mode3D(camera);

        dev_map.draw(&mut d3d, &camera, screen_width, screen_height);

        if debug_mode {
            d3d.draw_grid(50, 1.0);
            
            
        }

        std::mem::drop(d3d);

        if debug_mode {
            d.draw_fps(5, 0);
            d.draw_text(dt.to_string().as_str(), 5, 20, 20, Color::GRAY);
        }

        std::mem::drop(d);
        
        
    }
}
