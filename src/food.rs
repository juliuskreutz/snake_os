use uefi::proto::console::gop::{BltOp, BltPixel, GraphicsOutput};

use crate::game;

pub struct Food {
    position: (usize, usize),
    color: BltPixel,
}

impl Default for Food {
    fn default() -> Self {
        Self {
            position: <(usize, usize)>::default(),
            color: BltPixel::new(255, 0, 0),
        }
    }
}

impl Food {
    pub fn position(&self) -> (usize, usize) {
        self.position
    }

    pub fn respawn(&mut self, gop: &mut GraphicsOutput, position: (usize, usize)) {
        self.position = position;

        gop.blt(BltOp::VideoFill {
            color: self.color,
            dest: (position.0 * game::TILE_SIZE, position.1 * game::TILE_SIZE),
            dims: (game::TILE_SIZE, game::TILE_SIZE),
        })
        .unwrap();
    }
}
