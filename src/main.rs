use macroquad::prelude::*;

#[derive(Clone, Copy)]
enum Player {
    X,
    O,
}

enum GameState {
    Playing,
    Won(Vec4),
    Draw,
}

#[derive(PartialEq)]
enum Slot {
    Empty,
    X,
    O,
}

fn new_grid() -> Vec<Vec<Slot>> {
    let mut grid = Vec::new();

    for x in 0..3 {
        grid.push(Vec::new());
        for _y in 0..3 {
            grid[x].push(Slot::Empty);
        }
    }

    grid
}

fn get_rect(x: usize, y: usize, block_size: f32, block_pad: f32) -> Rect {
    let total_block_size = block_size + block_pad;
    Rect::new(
        x as f32 * total_block_size
            + (screen_width() - (total_block_size * 3f32 - block_pad)) * 0.5f32,
        y as f32 * total_block_size
            + (screen_height() - (total_block_size * 3f32 - block_pad)) * 0.5f32,
        block_size,
        block_size,
    )
}

fn draw_outline_rect(x: f32, y: f32, w: f32, h: f32, color: Color, block_pad: f32) {
    draw_circle(x, y, block_pad * 0.5f32, color);
    draw_circle(x, y + h, block_pad * 0.5f32, color);
    draw_circle(x + w, y, block_pad * 0.5f32, color);
    draw_circle(x + w, y + h, block_pad * 0.5f32, color);

    draw_line(x, y, x + w, y, block_pad, color);
    draw_line(x, y, x, y + h, block_pad, color);

    draw_line(x + w, y, x + w, y + h, block_pad, color);
    draw_line(x, y + h, x + w, y + h, block_pad, color);
}

fn draw_grid(grid: &Vec<Vec<Slot>>, turn: Player, block_size: f32, block_pad: f32) {
    let total_block_size = block_size + block_pad;
    for (x, col) in grid.iter().enumerate() {
        for (y, slot) in col.iter().enumerate() {
            match (slot, turn) {
                (Slot::X, Player::X) | (Slot::O, Player::O) => draw_outline_rect(
                    x as f32 * total_block_size
                        + (screen_width() - (total_block_size * grid.len() as f32 - block_pad))
                            * 0.5f32,
                    y as f32 * total_block_size
                        + (screen_height() - (total_block_size * grid.len() as f32 - block_pad))
                            * 0.5f32,
                    block_size,
                    block_size,
                    BLACK,
                    block_pad,
                ),
                _ => (),
            }
            let rect = get_rect(x, y, block_size, block_pad);
            draw_rectangle(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                match slot {
                    Slot::Empty => WHITE,
                    Slot::X => RED,
                    Slot::O => BLUE,
                },
            );
        }
    }
}

fn check_grid(grid: &[Vec<Slot>]) -> Option<Vec4> {
    for i in 0..3 {
        if grid[i]
            .iter()
            .all(|slot| slot != &Slot::Empty && slot == &grid[i][0])
        {
            return Some(vec4(i as f32, 0f32, i as f32, 2f32));
        }
        if grid
            .iter()
            .all(|col| col[i] != Slot::Empty && col[i] == grid[0][i])
        {
            return Some(vec4(0f32, i as f32, 2f32, i as f32));
        }
    }
    if grid[0][0] == grid[1][1] && grid[1][1] == grid[2][2] && grid[1][1] != Slot::Empty {
        return Some(vec4(0f32, 0f32, 2f32, 2f32));
    } else if grid[0][2] == grid[1][1] && grid[1][1] == grid[2][0] && grid[1][1] != Slot::Empty {
        return Some(vec4(0f32, 2f32, 2f32, 0f32));
    }
    if grid
        .iter()
        .all(|col| col.iter().all(|slot| slot != &Slot::Empty))
    {
        return Some(vec4(0f32, 0f32, 0f32, 0f32));
    }
    None
}

fn get_pressed() -> Option<Vec2> {
    if is_mouse_button_pressed(MouseButton::Left) {
        return Some(vec2(mouse_position().0, mouse_position().1));
    }
    for touch in touches() {
        if touch.phase == TouchPhase::Started {
            return Some(touch.position);
        }
    }
    None
}

#[macroquad::main("Tic Tac Toe")]
async fn main() {
    let mut grid = new_grid();
    let mut game_state = GameState::Playing;
    let mut turn = Player::X;

    let mut block_size;
    let mut block_pad;

    loop {
        if screen_width() < screen_height() {
            block_size = screen_width() * 0.5f32 / 3.1f32;
            block_pad = screen_width() * 0.5f32 / 0.05f32;
        } else {
            block_size = screen_height() * 0.5f32 / 3.1f32;
            block_pad = screen_height() * 0.5f32 / 0.05f32;
        }
        match game_state {
            GameState::Playing => {
                clear_background(match turn {
                    Player::X => RED,
                    Player::O => BLUE,
                });
                draw_grid(&grid, turn, block_size, block_pad);

                if let Some(line) = check_grid(&grid) {
                    if line.x == 0f32 && line.y == 0f32 && line.z == 0f32 && line.w == 0f32 {
                        game_state = GameState::Draw;
                    } else {
                        game_state = GameState::Won(line);
                    }
                }

                if let Some(pos) = get_pressed() {
                    'x_loop: for x in 0..3 {
                        for y in 0..3 {
                            let rect = get_rect(x, y, block_size, block_pad);

                            if rect.x <= pos.x
                                && pos.x <= (rect.x + rect.w)
                                && rect.y <= pos.y
                                && pos.y <= (rect.y + rect.h)
                            {
                                if grid[x][y] == Slot::Empty {
                                    grid[x][y] = match turn {
                                        Player::X => Slot::X,
                                        Player::O => Slot::O,
                                    };
                                    turn = match turn {
                                        Player::X => Player::O,
                                        Player::O => Player::X,
                                    };
                                }
                                break 'x_loop;
                            }
                        }
                    }
                }
            }
            GameState::Won(line) => {
                clear_background(BLACK);
                draw_grid(&grid, turn, block_size, block_pad);

                let rect1 = get_rect(line.x as usize, line.y as usize, block_size, block_pad);
                let point1 = vec2(rect1.x + rect1.w * 0.5f32, rect1.y + rect1.h * 0.5f32);

                let rect2 = get_rect(line.z as usize, line.w as usize, block_size, block_pad);
                let point2 = vec2(rect2.x + rect2.w * 0.5f32, rect2.y + rect2.h * 0.5f32);

                draw_line(
                    point1.x,
                    point1.y,
                    point2.x,
                    point2.y,
                    block_size * 0.2f32,
                    WHITE,
                );

                draw_circle(point1.x, point1.y, block_size * 0.1f32, WHITE);
                draw_circle(point2.x, point2.y, block_size * 0.1f32, WHITE);

                if let Some(_pos) = get_pressed() {
                    game_state = GameState::Playing;
                    grid = new_grid();
                    turn = Player::X;
                }
            }
            GameState::Draw => {
                clear_background(BLACK);
                draw_grid(&grid, turn, block_size, block_pad);

                if let Some(_pos) = get_pressed() {
                    game_state = GameState::Playing;
                    grid = new_grid();
                    turn = Player::X;
                }
            }
        }

        next_frame().await;
    }
}
