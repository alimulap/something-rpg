use godot::{
    classes::{file_access::ModeFlags, FileAccess, ITileMapLayer, TileMapLayer},
    prelude::*,
};
use rand::{seq::SliceRandom, thread_rng};

use crate::tile::{Tile, TileSideType};

// const TILEMAP_DIMENSION: [usize; 2] = [30, 10];

#[derive(GodotClass)]
#[class(base=TileMapLayer)]
pub struct PGTilemap {
    #[export(file = "*.json")]
    tiles_json: GString,
    #[export]
    dimension_width: u32,
    #[export]
    dimension_height: u32,
    #[export]
    start_pos: Vector2i,
    placement_stack: Vec<Placement>,
    base: Base<TileMapLayer>,
}

#[godot_api]
impl ITileMapLayer for PGTilemap {
    fn init(base: Base<TileMapLayer>) -> Self {
        let dimension_width = 30u32;
        let dimension_height = 10u32;
        let placement_stack = Vec::new();
        Self {
            base,
            placement_stack,
            tiles_json: GString::new(),
            dimension_width,
            dimension_height,
            start_pos: Vector2i::default(),
        }
    }

    fn ready(&mut self) {
        self.placement_stack =
            Vec::with_capacity((self.dimension_width * self.dimension_height) as usize);
        let tiles_str = FileAccess::open(&self.tiles_json, ModeFlags::READ)
            .unwrap()
            .get_as_text()
            .to_string();
        let tiles = serde_json::from_str::<Vec<Tile>>(&tiles_str).unwrap();

        let mut rng = thread_rng();

        while self.placement_stack.len() < self.placement_stack.capacity() {
            let i = self.placement_stack.len() as u32;
            let mut possibilities = Vec::new();
            for tile in &tiles {
                if self.can_place_tile(i, tile) {
                    possibilities.push(tile.clone());
                }
            }
            if possibilities.is_empty() {
                if self.placement_stack.is_empty() {
                    panic!("Something is wrong with the algorithm");
                }

                if self.backtrack() {
                    continue;
                } else {
                    panic!("Something is wrong with the algorithm");
                }
            }

            possibilities.shuffle(&mut rng);
            let tile = possibilities.pop().unwrap();
            self.placement_stack.push(Placement {
                tile,
                possibilities,
            });
        }

        self.place_tiles();
    }

    fn process(&mut self, _delta: f64) {}

    fn physics_process(&mut self, _delta: f64) {}
}

impl PGTilemap {
    fn place_tiles(&mut self) {
        while let Some(tile) = self.placement_stack.pop().map(|placement| placement.tile) {
            let i = self.placement_stack.len() as u32;
            let pos = self.start_pos
                + Vector2i::new(
                    (i % self.dimension_width) as i32,
                    (i / self.dimension_width) as i32,
                );
            self.base_mut()
                .set_cell_ex(pos)
                .source_id(tile.source)
                .atlas_coords(Vector2i::new(tile.altas_coord.0, tile.altas_coord.1))
                .alternative_tile(tile.alternative)
                .done();
        }
    }

    fn can_place_tile(&self, i: u32, tile: &Tile) -> bool {
        let top_tile = if i < self.dimension_width {
            &TileSideType::Void
        } else {
            &self.placement_stack[(i - self.dimension_width) as usize]
                .tile
                .sides
                .bottom
        };

        let left_tile = if i == 0 || i % self.dimension_width == 0 {
            &TileSideType::Void
        } else {
            &self.placement_stack[i as usize - 1].tile.sides.right
        };

        let right_tile = if i % self.dimension_width == self.dimension_width - 1 {
            Some(&TileSideType::Void)
        } else {
            None
        };

        let bottom_tile =
            if (self.dimension_width * self.dimension_height) - i < self.dimension_width {
                Some(&TileSideType::Void)
            } else {
                None
            };

        right_tile
            .map(|right_tile| tile.sides.right.eq(right_tile))
            .unwrap_or(true)
            && tile.sides.left.eq(left_tile)
            && tile.sides.top.eq(top_tile)
            && bottom_tile
                .map(|bottom_tile| tile.sides.bottom.eq(bottom_tile))
                .unwrap_or(true)
    }

    fn backtrack(&mut self) -> bool {
        godot_print!("backtrack");
        while let Some(placement_tile) = self.placement_stack.pop() {
            let mut possibilities = placement_tile.possibilities;
            while let Some(tile) = possibilities.pop() {
                let index = possibilities.len() as u32;
                if self.can_place_tile(index, &tile) {
                    self.placement_stack.push(Placement {
                        tile,
                        possibilities,
                    });
                    return true;
                }
            }
        }
        false
    }
}

struct Placement {
    tile: Tile,
    possibilities: Vec<Tile>,
}
