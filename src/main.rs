use macroquad::prelude::*;

#[derive(Clone, Copy)]
enum Tile {
    Mine,
    Clear(u8),
}

#[derive(Clone, Copy)]
enum TileData {
    Unknown,
    Known,
    Flagged,
}

struct Minefield {
    mines: Vec<Vec<(Tile, TileData)>>,
}
impl Minefield {
    fn new(size: usize, amount: usize) -> Self {
        let row = vec![(Tile::Clear(0), TileData::Unknown); size];
        let mut field = vec![row; size];
        let rng = rand::RandGenerator::new();
        for _ in 0..amount {
            let x = rng.gen_range(0, size);
            let y = rng.gen_range(0, size);
            field[x][y].0 = Tile::Mine;
            // update neighbours
            for neighbour_y in y.saturating_sub(1)..(y + 2).min(size) {
                for neighbour_x in x.saturating_sub(1)..(x + 2).min(size) {
                    match &mut field[neighbour_x][neighbour_y].0 {
                        Tile::Clear(value) => {
                            *value += 1;
                        }
                        _ => {}
                    }
                }
            }
        }
        Self { mines: field }
    }
    fn draw(&self, scaling: f32) {
        for (x, column) in self.mines.iter().enumerate() {
            for (y, tile) in column.iter().enumerate() {
                let tile_x = x as f32 * scaling;
                let tile_y = y as f32 * scaling;
                let text_x = tile_x + scaling * 0.25;
                let text_y = tile_y + scaling * 0.75;
                match tile.1 {
                    TileData::Unknown => {
                        draw_rectangle(
                            tile_x,
                            tile_y,
                            scaling - 1.0,
                            scaling - 1.0,
                            UNKNOWN_TILE_COLOR,
                        );
                    }
                    TileData::Flagged => {
                        draw_rectangle(
                            tile_x,
                            tile_y,
                            scaling - 1.0,
                            scaling - 1.0,
                            UNKNOWN_TILE_COLOR,
                        );
                        draw_text("P", text_x, text_y, scaling, RED);
                    }
                    TileData::Known => match tile.0 {
                        Tile::Mine => {
                            draw_rectangle(tile_x, tile_y, scaling - 1.0, scaling - 1.0, RED);
                            draw_text("X", text_x, text_y, scaling, BLACK);
                        }
                        Tile::Clear(value) => {
                            draw_rectangle(
                                tile_x,
                                tile_y,
                                scaling - 1.0,
                                scaling - 1.0,
                                TILE_COLOR,
                            );
                            if value == 0 {
                                continue;
                            }
                            draw_text(
                                &value.to_string(),
                                text_x,
                                text_y,
                                scaling,
                                COLORS[value as usize - 1],
                            );
                        }
                    },
                }
            }
        }
    }
}

const COLORS: [Color; 8] = [BLUE, GREEN, RED, PURPLE, RED, ORANGE, YELLOW, YELLOW];
const BACKGROUND_COLOR: Color = Color::from_hex(0x0f0f0f);
const TILE_COLOR: Color = Color::from_hex(0x1d2021);
const UNKNOWN_TILE_COLOR: Color = Color::from_hex(0x2D3031);

#[macroquad::main("minesweeper")]
async fn main() {
    let field_size = 16;
    let mut minefield = Minefield::new(field_size, field_size * field_size / 4);
    let scaling = 30.0;
    loop {
        clear_background(BACKGROUND_COLOR);
        minefield.draw(scaling);
        let (mouse_x, mouse_y) = mouse_position();
        if is_mouse_button_pressed(MouseButton::Left) {
            let tile_x = (mouse_x / scaling) as usize;
            let tile_y = (mouse_y / scaling) as usize;
            if tile_x > 0 && tile_x < field_size {
                if tile_y > 0 && tile_y < field_size {
                    minefield.mines[tile_x][tile_y].1 = TileData::Known;
                }
            }
        }
        next_frame().await
    }
}
