use macroquad::{miniquad::window::screen_size, prelude::*};

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
    WrongFlag,
}

struct Minefield {
    field: Vec<Vec<(Tile, TileData)>>,
    size: usize,
    has_lost: bool,
    has_won: bool,
    remaining_flags: usize,
}
impl Minefield {
    fn empty(size: usize) -> Self {
        let row = vec![(Tile::Clear(0), TileData::Unknown); size];
        let field = vec![row; size];
        Self {
            field,
            size,
            has_lost: false,
            has_won: false,
            remaining_flags: 0,
        }
    }
    /// Generate a new minefield "around" a mouse click,
    /// ensuring that the clicked tile is safe, with a value of 0
    fn new_around_click(size: usize, mines_amount: usize, click_x: usize, click_y: usize) -> Self {
        let rng = rand::RandGenerator::new();
        rng.srand(macroquad::miniquad::date::now() as u64);

        // Keep regenerating minefields until one fills the conditions
        loop {
            let mut field = Self::empty(size);
            field.remaining_flags = mines_amount;
            let mut remaining_mines = mines_amount;

            while remaining_mines > 0 {
                let x = rng.gen_range(0, size);
                let y = rng.gen_range(0, size);
                if let Tile::Mine = field.field[x][y].0 {
                    continue;
                }
                remaining_mines -= 1;
                field.field[x][y].0 = Tile::Mine;
                // update neighbours
                for neighbour_y in y.saturating_sub(1)..(y + 2).min(size) {
                    for neighbour_x in x.saturating_sub(1)..(x + 2).min(size) {
                        if let Tile::Clear(value) = &mut field.field[neighbour_x][neighbour_y].0 {
                            *value += 1;
                        }
                    }
                }
            }
            if let Tile::Clear(value) = field.field[click_x][click_y].0 {
                if value == 0 {
                    return field;
                }
            }
        }
    }
    fn draw(&self, scaling: f32, offset_x: f32, offset_y: f32) {
        for (x, column) in self.field.iter().enumerate() {
            for (y, tile) in column.iter().enumerate() {
                let tile_x = x as f32 * scaling + offset_x;
                let tile_y = y as f32 * scaling + offset_y;
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
                    TileData::WrongFlag => {
                        draw_rectangle(
                            tile_x,
                            tile_y,
                            scaling - 1.0,
                            scaling - 1.0,
                            UNKNOWN_TILE_COLOR,
                        );
                        draw_text("P", text_x, text_y, scaling, BLACK);
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
                                TILE_VALUE_COLORS[value as usize - 1],
                            );
                        }
                    },
                }
            }
        }
    }
    fn game_over(&self) -> bool {
        self.has_lost || self.has_won
    }
    /// Update self.has_won by checking if all flags are placed and correct and no tiles are unknown
    fn check_win(&mut self) {
        if self.has_lost {
            self.has_won = false;
            return;
        }
        if self.remaining_flags != 0 {
            self.has_won = false;
            return;
        }
        for row in &self.field {
            for tile in row {
                match tile.1 {
                    TileData::Flagged => {
                        if let Tile::Clear(_) = tile.0 {
                            self.has_won = false;
                            return;
                        }
                    }
                    TileData::Unknown => {
                        self.has_won = false;
                        return;
                    }
                    _ => {}
                }
            }
        }
        self.has_won = true;
    }
    /// Make a tile flagged
    fn flag_tile(&mut self, x: usize, y: usize) {
        let tile = &mut self.field[x][y].1;
        match tile {
            TileData::Flagged => {
                *tile = TileData::Unknown;
                self.remaining_flags += 1;
            }
            TileData::Unknown => {
                if self.remaining_flags > 0 {
                    *tile = TileData::Flagged;
                    self.remaining_flags -= 1;
                    if self.remaining_flags == 0 {
                        self.check_win();
                    }
                }
            }
            _ => {}
        }
    }
    /// Handles a click on the minefield. If clicked tile is unknown, it is revealed.
    /// If it is known, it is expanded
    fn handle_click(&mut self, x: usize, y: usize) {
        let tile_data = self.field[x][y].1;
        match tile_data {
            // if tile is unknown, make it known
            TileData::Unknown => {
                self.reveal_tile(x, y);
            }
            TileData::Known => {
                self.try_expand_tile(x, y);
            }
            _ => {}
        }
        self.check_win();
    }
    /// Reveals a tile
    fn reveal_tile(&mut self, x: usize, y: usize) {
        let tile = &mut self.field[x][y];
        if let Tile::Mine = tile.0 {
            self.has_lost = true;
            self.reveal_all_mines();
            return;
        }
        tile.1 = TileData::Known;
    }
    /// "Expands" a tile at position, i.e. clicking its neighbouring tiles that arent flagged.
    /// If too few of its neighbours are known, it does nothing
    fn try_expand_tile(&mut self, x: usize, y: usize) {
        let tile = self.field[x][y].0;
        if let Tile::Clear(value) = tile {
            // count unknown neighbours and flagged neighbours
            let mut flagged_neighbours = 0;
            let mut unknown_neighbours = Vec::new();

            for neighbour_y in y.saturating_sub(1)..(y + 2).min(self.size) {
                for neighbour_x in x.saturating_sub(1)..(x + 2).min(self.size) {
                    match self.field[neighbour_x][neighbour_y].1 {
                        TileData::Unknown => {
                            unknown_neighbours.push((neighbour_x, neighbour_y));
                        }
                        TileData::Flagged => {
                            flagged_neighbours += 1;
                        }
                        _ => {}
                    }
                }
            }
            if flagged_neighbours >= value {
                for (x, y) in unknown_neighbours {
                    self.reveal_tile(x, y);

                    // if the revealed tile is a clear tile with value 0, also expand it, for a recursive expansion
                    if let Tile::Clear(value) = self.field[x][y].0 {
                        if value == 0 {
                            self.try_expand_tile(x, y);
                        }
                    }
                }
            }
        }
    }
    /// Reveals all mines.
    /// Called on defeat.
    fn reveal_all_mines(&mut self) {
        for row in &mut self.field {
            for tile in row {
                match tile.0 {
                    Tile::Mine => {
                        if let TileData::Unknown = tile.1 {
                            tile.1 = TileData::Known;
                        }
                    }
                    Tile::Clear(_) => {
                        if let TileData::Flagged = tile.1 {
                            tile.1 = TileData::WrongFlag;
                        }
                    }
                }
            }
        }
    }
}

const TILE_VALUE_COLORS: [Color; 8] = [BLUE, GREEN, RED, PURPLE, RED, ORANGE, YELLOW, WHITE];
const BACKGROUND_COLOR: Color = Color::from_hex(0x0f0f0f);
const TILE_COLOR: Color = Color::from_hex(0x1d2021);
const UNKNOWN_TILE_COLOR: Color = Color::from_hex(0x2D3031);

fn calculate_offset(scaling: f32, field_size: usize) -> (f32, f32) {
    let (width, _) = screen_size();
    let x = ((width - field_size as f32 * scaling) / 2.0).round();
    (x, scaling * 2.0)
}

#[macroquad::main("minesweeper")]
async fn main() {
    let field_size = 16;
    let mut minefield = Minefield::empty(field_size);
    let scaling = 32.0;
    let (mut offset_x, mut offset_y);

    let mut started_click_on_button = false;

    let mut first_click = true;

    loop {
        (offset_x, offset_y) = calculate_offset(scaling, field_size);
        clear_background(BACKGROUND_COLOR);

        // draw minefield
        minefield.draw(scaling, offset_x, offset_y);

        // handle clicking tiles
        let (mouse_x, mouse_y) = mouse_position();
        let mouse_tile_x = ((mouse_x - offset_x) / scaling) as usize;
        let mouse_tile_y = ((mouse_y - offset_y) / scaling) as usize;
        let mouse_tile_in_bounds = mouse_x > offset_x
            && mouse_tile_x < field_size
            && mouse_y > offset_y
            && mouse_tile_y < field_size;

        if !minefield.game_over()
            && is_mouse_button_pressed(MouseButton::Left)
            && mouse_tile_in_bounds
        {
            if first_click {
                // on the first click, generate the actual minefield around that click
                // this means that wherever you initially click, that spot will be guaranteed to be safe
                minefield = Minefield::new_around_click(
                    field_size,
                    field_size * field_size / 4,
                    mouse_tile_x,
                    mouse_tile_y,
                );
                first_click = false;
                // extra click so it expands automatically
                minefield.handle_click(mouse_tile_x, mouse_tile_y);
            }

            minefield.handle_click(mouse_tile_x, mouse_tile_y);
        }
        if !minefield.game_over()
            && is_mouse_button_pressed(MouseButton::Right)
            && mouse_tile_in_bounds
        {
            minefield.flag_tile(mouse_tile_x, mouse_tile_y);
        }

        // handle ui

        // handle restart button
        let button_x = offset_x + field_size as f32 * scaling / 2.0;
        let button_y = scaling * 0.5;
        let mouse_over_restart_button = mouse_x > button_x
            && mouse_x < button_x + scaling
            && mouse_y > button_y
            && mouse_y < button_y + scaling;

        // handle clicking restart button
        if is_mouse_button_pressed(MouseButton::Left) && mouse_over_restart_button {
            started_click_on_button = true;
        }
        if is_mouse_button_released(MouseButton::Left) {
            if mouse_over_restart_button && started_click_on_button {
                minefield = Minefield::empty(field_size);
                first_click = true;
            }
            started_click_on_button = false;
        }

        let clicking_button = is_mouse_button_down(MouseButton::Left) && mouse_over_restart_button;
        let button_color = if clicking_button && started_click_on_button {
            ORANGE
        } else {
            YELLOW
        };
        draw_rectangle(button_x, button_y, scaling, scaling, button_color);
        draw_text(":)", button_x, button_y + scaling * 0.75, scaling, BLACK);

        // draw flag counter
        draw_text(
            &minefield.remaining_flags.to_string(),
            offset_x,
            button_y + scaling * 0.75,
            scaling,
            RED,
        );

        let text = if minefield.has_lost {
            "you lose"
        } else if minefield.has_won {
            "you win!"
        } else {
            ""
        };
        draw_text(
            text,
            offset_x,
            scaling * field_size as f32 + offset_y + scaling,
            scaling,
            RED,
        );

        next_frame().await
    }
}
