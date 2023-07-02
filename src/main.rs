use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::f64::consts::PI;
use std::time::Duration;

const WIN_X: u32 = 960;
const WIN_Y: u32 = 540;

const MOVE_AMOUNT: f64 = 0.02;
const FOV: f64 = PI / 3.0;

const MAP: [[u32; 10]; 10] = [
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 2, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 2, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 2, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Ray Casting", WIN_X, WIN_Y)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut player_pos_x = 4.0;
    let mut player_pos_y = 4.0;
    let mut player_facing_angle: f64 = 0.0;

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'running;
            }
        }

        let player_movement_x_offset = MOVE_AMOUNT * player_facing_angle.cos();
        let player_movement_y_offset = MOVE_AMOUNT * player_facing_angle.sin();

        for key in event_pump.keyboard_state().pressed_scancodes() {
            if key == Scancode::W {
                player_pos_x += player_movement_x_offset;
                player_pos_y += player_movement_y_offset;
            }
            if key == Scancode::S {
                player_pos_x -= player_movement_x_offset;
                player_pos_y -= player_movement_y_offset;
            }

            if key == Scancode::A {
                player_facing_angle -= 0.05;
            }

            if key == Scancode::D {
                player_facing_angle += 0.05;
            }

            if key == Scancode::Q {
                player_facing_angle -= 0.01;
            }

            if key == Scancode::E {
                player_facing_angle += 0.01;
            }

            if key == Scancode::Escape {
                break 'running;
            }
        }

        let mut ray_angle = player_facing_angle - FOV / 2.0;
        let angle_increment = FOV / WIN_X as f64;
        let mut x_draw_pos = 0;

        while ray_angle < player_facing_angle + FOV / 2.0 {
            let horizontal_ray_angle = ray_angle;

            let x_step = if horizontal_ray_angle.cos().is_sign_positive() {
                let offset = player_pos_x.ceil() - player_pos_x;
                if offset == 0.0 {
                    offset + 1.0
                } else {
                    offset
                }
            } else {
                let offset = player_pos_x.floor() - player_pos_x;
                if offset == 0.0 {
                    offset - 1.0
                } else {
                    offset
                }
            };

            let y_step = x_step * horizontal_ray_angle.tan();

            let mut horizontal_map_x = player_pos_x + x_step;
            let mut horizontal_map_y = player_pos_y + y_step;
            let mut horizontal_ray_length: f64 = -1.0;

            let y_step = y_step * (1.0 / x_step);

            while horizontal_map_x < MAP.len() as f64
                && horizontal_map_x > 0.0
                && horizontal_map_y < MAP.len() as f64
                && horizontal_map_y > 0.0
            {
                if horizontal_ray_angle.cos().is_sign_positive() {
                    if MAP[horizontal_map_x as usize][horizontal_map_y as usize] > 0 {
                        horizontal_ray_length = ((horizontal_map_x - player_pos_x).powf(2.0)
                            + (horizontal_map_y - player_pos_y).powf(2.0))
                        .sqrt();

                        break;
                    }

                    horizontal_map_x += 1.0;
                    horizontal_map_y += y_step;
                } else {
                    if MAP[(horizontal_map_x - 1.0) as usize][horizontal_map_y as usize] > 0 {
                        horizontal_ray_length = ((horizontal_map_x - player_pos_x).powf(2.0)
                            + (horizontal_map_y - player_pos_y).powf(2.0))
                        .sqrt();

                        break;
                    }

                    horizontal_map_x -= 1.0;
                    horizontal_map_y -= y_step;
                }
            }

            if horizontal_ray_length == -1.0 {
                horizontal_ray_length = std::f64::MAX;
            }

            let vertical_ray_angle = PI / 2.0 - ray_angle;

            let y_step = if vertical_ray_angle.cos().is_sign_positive() {
                let offset = player_pos_y.ceil() - player_pos_y;
                if offset == 0.0 {
                    offset + 1.0
                } else {
                    offset
                }
            } else {
                let offset = player_pos_y.floor() - player_pos_y;
                if offset == 0.0 {
                    offset - 1.0
                } else {
                    offset
                }
            };

            let x_step = y_step * vertical_ray_angle.tan();

            let mut vertical_map_x = player_pos_x + x_step;
            let mut vertical_map_y = player_pos_y + y_step;
            let mut vertical_ray_length: f64 = -1.0;

            let x_step = x_step * (1.0 / y_step);

            while vertical_map_x < MAP.len() as f64
                && vertical_map_x > 0.0
                && vertical_map_y < MAP.len() as f64
                && vertical_map_y > 0.0
            {
                if vertical_ray_angle.cos().is_sign_positive() {
                    if MAP[vertical_map_x as usize][vertical_map_y as usize] > 0 {
                        vertical_ray_length = ((vertical_map_x - player_pos_x).powf(2.0)
                            + (vertical_map_y - player_pos_y).powf(2.0))
                        .sqrt();

                        break;
                    }

                    vertical_map_x += x_step;
                    vertical_map_y += 1.0;
                } else {
                    if MAP[vertical_map_x as usize][(vertical_map_y - 1.0) as usize] > 0 {
                        vertical_ray_length = ((vertical_map_x - player_pos_x).powf(2.0)
                            + (vertical_map_y - player_pos_y).powf(2.0))
                        .sqrt();

                        break;
                    }

                    vertical_map_x -= x_step;
                    vertical_map_y -= 1.0;
                }
            }

            if vertical_ray_length == -1.0 {
                vertical_ray_length = std::f64::MAX;
            }

            let ray_length: f64;

            if horizontal_ray_length < vertical_ray_length {
                ray_length = horizontal_ray_length;

                canvas.set_draw_color(Color::RGB(255, 0, 0));
            } else {
                ray_length = vertical_ray_length;

                canvas.set_draw_color(Color::RGB(205, 0, 0));
            }

            let corrected_ray = ray_length * (ray_angle - player_facing_angle).cos();

            let ray_height = WIN_Y as f64 / corrected_ray;
            let window_offset = (WIN_Y as f64 - ray_height) / 2.0;

            canvas.draw_line(
                Point::new(x_draw_pos, window_offset as i32),
                Point::new(x_draw_pos, ((WIN_Y as f64) - window_offset) as i32),
            )?;

            x_draw_pos += 1;
            ray_angle += angle_increment;
        }

        canvas.present();
        std::thread::sleep(Duration::from_millis(10));
    }
    Ok(())
}
