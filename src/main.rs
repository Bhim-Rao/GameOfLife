#![windows_subsystem = "windows"]
use macroquad::prelude::*;
use std::time::Instant;

const STEP: f32 = 5.0;

#[macroquad::main(window_conf)]
async fn main() {
    let w_i = (screen_width()/STEP) as i32;
    let h_i = (screen_height()/STEP) as i32;
    let mut g = grid();
    let mut mouse_x;
    let mut mouse_y;
    let mut m_x;
    let mut m_y;
    let mut y_u;
    let mut x_u;
    let mut state;
    let mut n_c;
    let mut simulate = false;
    loop {
        let now = Instant::now();
        let mut ng = grid();
        let background = Color::from_hex(0x0f0f0f);
        clear_background(background);
        if is_mouse_button_down(MouseButton::Left) {
            (mouse_x,mouse_y) = mouse_position();
            if mouse_x < screen_width()-STEP && mouse_y < screen_height()-STEP && mouse_x > 0.0 && mouse_y > 0.0 {
                m_x = ((mouse_x/STEP)) as usize;
                m_y = ((mouse_y/STEP)) as usize;
                g[m_y as usize][m_x as usize] = 1;
            }
        }
        if is_mouse_button_down(MouseButton::Right) {
            (mouse_x,mouse_y) = mouse_position();
            if mouse_x < screen_width()-STEP && mouse_y < screen_height()-STEP && mouse_x > 0.0 && mouse_y > 0.0 {
                m_x = ((mouse_x/STEP)) as usize;
                m_y = ((mouse_y/STEP)) as usize;
                g[m_y as usize][m_x as usize] = 0;
            }
        }
        if is_key_pressed(KeyCode::Space) {
            if simulate {
                simulate = false;
            } else {
                simulate = true;
            }
        }
        for x in 0..w_i {
            for y in 0..h_i {
                y_u = y as usize;
                x_u = x as usize;
                state = g[y_u][x_u];
                if y == h_i-1 || x == w_i-1 || y == 0 || x == 0 {

                } else if state == 1 {
                    draw_rectangle((x as f32)*STEP, (y as f32)*STEP, STEP, STEP, GREEN);
                    if simulate {
                        n_c = 0;
                        if g[y_u-1][x_u-1] == 1 {
                            n_c += 1;
                        }
                        if g[y_u-1][x_u] == 1 {
                            n_c += 1;
                        }
                        if g[y_u-1][x_u+1] == 1 {
                            n_c += 1;
                        }
                        if g[y_u][x_u-1] == 1 {
                            n_c += 1;
                        }
                        if g[y_u][x_u+1] == 1 {
                            n_c += 1;
                        }
                        if g[y_u+1][x_u-1] == 1 {
                            n_c += 1;
                        }
                        if g[y_u+1][x_u] == 1 {
                            n_c += 1;
                        }
                        if g[y_u+1][x_u+1] == 1 {
                            n_c += 1;
                        }

                        if n_c < 2 {
                            ng[y_u][x_u] = 0;
                        }
                        if n_c > 3 {
                            ng[y_u][x_u] = 0;
                        }
                        if n_c == 2 || n_c == 3 {
                            ng[y_u][x_u] = 1;
                        }
                    }
                } else if simulate {
                    n_c = 0;
                    if g[y_u-1][x_u-1] == 1 {
                        n_c += 1;
                    }
                    if g[y_u-1][x_u] == 1 {
                        n_c += 1;
                    }
                    if g[y_u-1][x_u+1] == 1 {
                        n_c += 1;
                    }
                    if g[y_u][x_u-1] == 1 {
                        n_c += 1;
                    }
                    if g[y_u][x_u+1] == 1 {
                        n_c += 1;
                    }
                    if g[y_u+1][x_u-1] == 1 {
                        n_c += 1;
                    }
                    if g[y_u+1][x_u] == 1 {
                        n_c += 1;
                    }
                    if g[y_u+1][x_u+1] == 1 {
                        n_c += 1;
                    }
                    if n_c == 3 {
                        ng[y_u][x_u] = 1;
                    }
                }
            }
        }
        if simulate {g = ng;}
        let fps = (1.0/((now.elapsed().as_millis() as f32)/1000.0)) as i32;
        draw_text(&(fps.to_string() + "fps"), 20.0, 20.0, 30.0, WHITE);
        next_frame().await
    }
}

fn grid() -> Vec<Vec<i32>> {
    let x_values = vec![0i32; (screen_width()/STEP) as usize];
    let y_values = vec![x_values.clone(); (screen_height()/STEP) as usize];
    return y_values;
}

fn window_conf() -> Conf {
    Conf {
        window_resizable: false,
        window_title: "Conway's Game of Life".to_string(),
        ..Default::default()
    }
}
