extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::f64::consts::PI;
use std::time::Duration;

const WIN_X: u32 = 1000;
const WIN_Y: u32 = 500;

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

    let mut player_pos_x = 400.0;
    let mut player_pos_y = 400.0;
    let mut player_facing_angle: f64 = 0.0;

    let move_amount = 10.0;

    'running: loop {
        //Clear previous rendering, (clean canvas)
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        //Work out player facing dir

        let player_movement_x_offset = move_amount * player_facing_angle.cos();

        let player_movement_y_offset = move_amount * player_facing_angle.sin();

        //Event handling
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    player_pos_x = player_pos_x + player_movement_x_offset;
                    player_pos_y = player_pos_y + player_movement_y_offset;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    player_pos_x = player_pos_x - player_movement_x_offset;
                    player_pos_y = player_pos_y - player_movement_y_offset;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    player_facing_angle -= 0.25;
                    if player_facing_angle < 0.0 {
                        player_facing_angle = PI * 2.0 - 0.25;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    player_facing_angle += 0.25;
                    if player_facing_angle >= 2.0 * PI {
                        player_facing_angle = 0.0;
                    }
                }

                _ => {}
            }
        }

        //Player Icon

        canvas.set_draw_color(Color::RGB(0, 255, 0));

        let mut horizontal_ray_end_loc = (0.0, 0.0);
        let mut horizontal_ray_distance = 0.0;
        let mut vertical_ray_end_loc = (0.0, 0.0);
        let mut vertical_ray_distance = 0.0;

        let mut angle = player_facing_angle - (PI / 4.0);

        let mut fov_end = angle + (PI / 2.0);

        let mut x_render: f64 = 1.0;

        while angle < fov_end {
            let mut ray_angle = angle;

            if ray_angle.is_sign_negative() {
                let amount = angle * -1.0;
                ray_angle = (2.0 * PI) - amount;
            }

            if ray_angle > (2.0 * PI) {
                ray_angle = ray_angle - (2.0 * PI)
            }

            //Horizontal Ray

            if ray_angle > PI && ray_angle < 2.0 * PI {
                let angle_at_top = (PI / 2.0) - (2.0 * PI - ray_angle);

                let mut player_facing_wall_y_pos = (player_pos_y as i32 / 100) * 100;

                let wall_y_offset = player_pos_y - player_facing_wall_y_pos as f64 - 0.0000001;
                let wall_x_offset = wall_y_offset * angle_at_top.tan();

                let mut player_facing_wall_x_pos = player_pos_x + wall_x_offset;

                while player_facing_wall_y_pos / 100 >= 0 {
                    let map_array_loc = (
                        player_facing_wall_x_pos as i32 / 100,
                        player_facing_wall_y_pos as i32 / 100 - 1,
                    );

                    if map_array_loc.0 > -1 && map_array_loc.0 < 10 {
                        if MAP[map_array_loc.1 as usize][map_array_loc.0 as usize] >= 1 {
                            horizontal_ray_end_loc =
                                (player_facing_wall_x_pos, player_facing_wall_y_pos as f64);

                            let y = player_pos_y - player_facing_wall_y_pos as f64;
                            let mut x = player_pos_x - player_facing_wall_x_pos as f64;

                            if x.is_sign_negative() {
                                x = x * -1.0
                            }

                            horizontal_ray_distance = x.hypot(y);

                            break;
                        } else {
                            let multiplier = 100.0 / wall_y_offset;
                            player_facing_wall_x_pos =
                                player_facing_wall_x_pos + wall_x_offset * multiplier;
                        }
                    }
                    player_facing_wall_y_pos -= 100;
                }
            } else if ray_angle < PI || ray_angle > PI {
                {
                    let angle_at_top = (PI / 2.0) - ray_angle;

                    let mut player_facing_wall_y_pos = (player_pos_y as i32 / 100) * 100 + 100;

                    let wall_y_offset = player_facing_wall_y_pos as f64 - player_pos_y;
                    let wall_x_offset = wall_y_offset * angle_at_top.tan();

                    let mut player_facing_wall_x_pos = player_pos_x + wall_x_offset;

                    while player_facing_wall_y_pos / 100 <= 1000 {
                        let map_array_loc = (
                            player_facing_wall_x_pos as i32 / 100,
                            player_facing_wall_y_pos as i32 / 100,
                        );

                        if map_array_loc.0 > -1 && map_array_loc.0 < 10 {
                            if MAP[map_array_loc.1 as usize][map_array_loc.0 as usize] >= 1 {
                                horizontal_ray_end_loc =
                                    (player_facing_wall_x_pos, player_facing_wall_y_pos as f64);

                                let y = player_facing_wall_y_pos as f64 - player_pos_y;
                                let mut x = player_pos_x - player_facing_wall_x_pos as f64;

                                if x.is_sign_negative() {
                                    x = x * -1.0
                                }

                                horizontal_ray_distance = x.hypot(y);

                                break;
                            } else {
                                let multiplier = 100.0 / wall_y_offset;
                                player_facing_wall_x_pos =
                                    player_facing_wall_x_pos + wall_x_offset * multiplier;
                            }
                        }
                        player_facing_wall_y_pos += 100;
                    }
                }
            }

            // Vertical Ray

            if ray_angle > PI / 2.0 && ray_angle < (3.0 * PI) / 2.0 {
                let angle_at_top = PI - ray_angle;

                let mut player_facing_wall_x_pos = (player_pos_x as i32 / 100) * 100;

                let wall_x_offset = player_pos_x - player_facing_wall_x_pos as f64 + 0.0000001;
                let wall_y_offset = wall_x_offset * angle_at_top.tan();

                let mut player_facing_wall_y_pos = player_pos_y + wall_y_offset;

                while player_facing_wall_x_pos / 100 >= 0 {
                    let map_array_loc = (
                        player_facing_wall_x_pos as i32 / 100 - 1,
                        player_facing_wall_y_pos as i32 / 100,
                    );
                    if map_array_loc.1 > -1 && map_array_loc.1 < 10 {
                        if MAP[map_array_loc.1 as usize][map_array_loc.0 as usize] >= 1 {
                            vertical_ray_end_loc =
                                (player_facing_wall_x_pos as f64, player_facing_wall_y_pos);

                            let mut y = player_facing_wall_y_pos as f64 - player_pos_y;
                            let x = player_pos_x - player_facing_wall_x_pos as f64;

                            if y.is_sign_negative() {
                                y = y * -1.0
                            }

                            vertical_ray_distance = x.hypot(y);

                            break;
                        } else {
                            let multiplier = 100.0 / wall_x_offset;
                            player_facing_wall_y_pos =
                                player_facing_wall_y_pos + wall_y_offset * multiplier;
                        }
                    }
                    player_facing_wall_x_pos -= 100;
                }
            } else if ray_angle < PI / 2.0 || ray_angle > (3.0 * PI) / 2.0 {
                let angle_at_top = ray_angle;

                let mut player_facing_wall_x_pos = (player_pos_x as i32 / 100) * 100 + 100;

                let wall_x_offset = player_facing_wall_x_pos as f64 - player_pos_x;
                let wall_y_offset = wall_x_offset * angle_at_top.tan();

                let mut player_facing_wall_y_pos = player_pos_y + wall_y_offset;

                while player_facing_wall_x_pos / 100 <= 1000 {
                    let map_array_loc = (
                        player_facing_wall_x_pos as i32 / 100,
                        player_facing_wall_y_pos as i32 / 100,
                    );

                    if map_array_loc.1 > -1 && map_array_loc.1 < 10 {
                        if MAP[map_array_loc.1 as usize][map_array_loc.0 as usize] >= 1 {
                            vertical_ray_end_loc =
                                (player_facing_wall_x_pos as f64, player_facing_wall_y_pos);

                            let mut y = player_pos_y - player_facing_wall_y_pos as f64;
                            let x = player_facing_wall_x_pos as f64 - player_pos_x;

                            if y.is_sign_negative() {
                                y = y * -1.0
                            }

                            vertical_ray_distance = x.hypot(y);

                            break;
                        } else {
                            let multiplier = 100.0 / wall_x_offset;
                            player_facing_wall_y_pos =
                                player_facing_wall_y_pos + wall_y_offset * multiplier;
                        }
                    }
                    player_facing_wall_x_pos += 100;
                }
            }

            if horizontal_ray_distance == 0.0 {
                let vertical_line_y = (500.0 / vertical_ray_distance) * 100.0;
                let draw_start_y = vertical_line_y / 4.0;

                canvas.set_draw_color(Color::RGB(0, 255, 0));
                canvas.fill_rect(Rect::new(x_render as i32, draw_start_y as i32, 13, vertical_line_y as u32)).unwrap();
            } else if vertical_ray_distance == 0.0 {
                let vertical_line_y = (500.0 / horizontal_ray_distance) * 100.0;
                let draw_start_y = vertical_line_y / 4.0;

                canvas.set_draw_color(Color::RGB(0, 200, 0));
                canvas.fill_rect(Rect::new(x_render as i32, draw_start_y as i32, 13, vertical_line_y as u32)).unwrap();
            } else if horizontal_ray_distance > vertical_ray_distance {
                let vertical_line_y = (500.0 / vertical_ray_distance) * 100.0;
                let draw_start_y = vertical_line_y / 4.0;

                canvas.set_draw_color(Color::RGB(0, 255, 0));
                canvas.fill_rect(Rect::new(x_render as i32, draw_start_y as i32, 13, vertical_line_y as u32)).unwrap();
            } else {
                let vertical_line_y = (500.0 / horizontal_ray_distance) * 100.0;
                let draw_start_y = vertical_line_y / 4.0;

                canvas.set_draw_color(Color::RGB(0, 200, 0));
                canvas.fill_rect(Rect::new(x_render as i32, draw_start_y as i32, 13, vertical_line_y as u32)).unwrap();
            }

            

            x_render += 12.5;
            angle += 0.02;
        }

        canvas.present(); //Draw buffer onto window
        std::thread::sleep(Duration::new(0, 10000000)); //Delay for capped framerate / update rate
    }

    Ok(())
}
