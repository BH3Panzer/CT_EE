use raylib::prelude::*;

pub fn handle_camera(camera: &mut Camera3D, dt: f32, rl: &mut RaylibHandle) {
    if rl.is_key_down(KeyboardKey::KEY_UP) {
        camera.position.z -= 20.0 * dt;
        camera.target.z -= 20.0 * dt;
    }

    if rl.is_key_down(KeyboardKey::KEY_DOWN) {
        camera.position.z += 20.0 * dt;
        camera.target.z += 20.0 * dt;
    }

    if rl.is_key_down(KeyboardKey::KEY_LEFT) {
        camera.position.x -= 20.0 * dt;
        camera.target.x -= 20.0 * dt;
    }

    if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
        camera.position.x += 20.0 * dt;
        camera.target.x += 20.0 * dt;
    }

    if rl.get_mouse_position().x < 50.0 {
        camera.position.x -= 25.0 * dt;
        camera.target.x -= 25.0 * dt;
    }

    if rl.get_mouse_position().x > rl.get_screen_width() as f32 - 50.0 {
        camera.position.x += 25.0 * dt;
        camera.target.x += 25.0 * dt;
    }

    if rl.get_mouse_position().y < 50.0 {
        camera.position.z -= 25.0 * dt;
        camera.target.z -= 25.0 * dt;
    }

    if rl.get_mouse_position().y > rl.get_screen_height() as f32 - 50.0 {
        camera.position.z += 25.0 * dt;
        camera.target.z += 25.0 * dt;
    }

    if rl.get_mouse_wheel_move() > 0.0 {
        camera.position.y -= 25.0 * dt;
        if camera.position.y < 0.0 {
            camera.position.y = 0.0;
        }
    } else if rl.get_mouse_wheel_move() < 0.0 {
        camera.position.y += 25.0 * dt;
        
    }
}