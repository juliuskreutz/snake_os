use alloc::collections::VecDeque;
use uefi::proto::console::gop::{BltOp, BltPixel, GraphicsOutput};

use crate::game;

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Right
    }
}

pub struct Snake {
    segments: VecDeque<(usize, usize)>,
    direction: Direction,
    rotations: VecDeque<Direction>,
    color: BltPixel,
}

impl Default for Snake {
    fn default() -> Self {
        Self {
            segments: VecDeque::default(),
            direction: Direction::default(),
            rotations: VecDeque::default(),
            color: BltPixel::new(0, 255, 0),
        }
    }
}

impl Snake {
    pub fn respawn(&mut self, gop: &mut GraphicsOutput) {
        self.segments.iter().for_each(|segment| {
            gop.blt(BltOp::VideoFill {
                color: BltPixel::new(50, 50, 50),
                dest: (segment.0 * game::TILE_SIZE, segment.1 * game::TILE_SIZE),
                dims: (game::TILE_SIZE, game::TILE_SIZE),
            })
            .unwrap();
        });

        self.segments.clear();
        self.direction = Direction::default();
        self.rotations.clear();

        let segment = <(usize, usize)>::default();
        self.segments.push_front(segment);

        gop.blt(BltOp::VideoFill {
            color: self.color,
            dest: (segment.0 * game::TILE_SIZE, segment.1 * game::TILE_SIZE),
            dims: (game::TILE_SIZE, game::TILE_SIZE),
        })
        .unwrap();
    }

    pub fn next_position(&mut self) -> (usize, usize) {
        while let Some(rotation) = self.rotations.pop_front() {
            if match self.direction {
                Direction::Up | Direction::Down => match rotation {
                    Direction::Up | Direction::Down => false,
                    Direction::Right | Direction::Left => true,
                },
                Direction::Right | Direction::Left => match rotation {
                    Direction::Right | Direction::Left => false,
                    Direction::Up | Direction::Down => true,
                },
            } {
                self.direction = rotation;
                break;
            }
        }

        let mut position = self.segments[0];

        match self.direction {
            Direction::Up => position.1 = position.1.checked_sub(1).unwrap_or(usize::MAX),
            Direction::Right => position.0 = position.0.checked_add(1).unwrap_or(usize::MIN),
            Direction::Down => position.1 = position.1.checked_add(1).unwrap_or(usize::MIN),
            Direction::Left => position.0 = position.0.checked_sub(1).unwrap_or(usize::MAX),
        }

        position
    }

    pub fn crawl(&mut self, gop: &mut GraphicsOutput, position: (usize, usize)) {
        let segment = self.segments.pop_back().unwrap();

        gop.blt(BltOp::VideoFill {
            color: BltPixel::new(50, 50, 50),
            dest: (segment.0 * game::TILE_SIZE, segment.1 * game::TILE_SIZE),
            dims: (game::TILE_SIZE, game::TILE_SIZE),
        })
        .unwrap();

        self.segments.push_front(position);

        gop.blt(BltOp::VideoFill {
            color: self.color,
            dest: (position.0 * game::TILE_SIZE, position.1 * game::TILE_SIZE),
            dims: (game::TILE_SIZE, game::TILE_SIZE),
        })
        .unwrap();
    }

    pub fn eat(&mut self, gop: &mut GraphicsOutput, position: (usize, usize)) {
        self.segments.push_front(position);

        gop.blt(BltOp::VideoFill {
            color: self.color,
            dest: (position.0 * game::TILE_SIZE, position.1 * game::TILE_SIZE),
            dims: (game::TILE_SIZE, game::TILE_SIZE),
        })
        .unwrap();
    }

    pub fn dead(&self) -> bool {
        let segment = self.segments[0];

        self.segments.iter().skip(1).any(|s| *s == segment)
    }

    pub fn contains(&self, position: (usize, usize)) -> bool {
        self.segments.contains(&position)
    }

    pub fn up(&mut self) {
        self.turn(Direction::Up);
    }

    pub fn left(&mut self) {
        self.turn(Direction::Left);
    }

    pub fn down(&mut self) {
        self.turn(Direction::Down);
    }

    pub fn right(&mut self) {
        self.turn(Direction::Right);
    }

    fn turn(&mut self, direction: Direction) {
        self.rotations.push_back(direction);
    }
}
