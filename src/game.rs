use uefi::proto::{
    console::{
        gop::{BltOp, BltPixel, GraphicsOutput},
        text::{Input, Key},
    },
    rng::Rng,
};

use crate::{food::Food, snake::Snake, timer::Timer};

pub const TILE_SIZE: usize = 50;
pub const TIME_STEP: u64 = 1000000;

pub struct Game<'a> {
    field_width: usize,
    field_height: usize,
    snake: Snake,
    food: Food,
    gop: &'a mut GraphicsOutput<'a>,
    rng: &'a mut Rng,
    input: &'a mut Input,
    timer: Timer<'a>,
}

impl<'a> Game<'a> {
    pub fn new(
        gop: &'a mut GraphicsOutput<'a>,
        rng: &'a mut Rng,
        input: &'a mut Input,
        timer: Timer<'a>,
    ) -> Self {
        let (width, height) = gop.current_mode_info().resolution();

        let (field_width, field_height) = (width / TILE_SIZE, height / TILE_SIZE);

        Self {
            field_width,
            field_height,
            snake: Snake::default(),
            food: Food::default(),
            gop,
            rng,
            input,
            timer,
        }
    }

    pub fn run(&mut self) {
        self.gop
            .blt(BltOp::VideoFill {
                color: BltPixel::new(50, 50, 50),
                dest: (0, 0),
                dims: self.gop.current_mode_info().resolution(),
            })
            .unwrap();
        self.gop
            .blt(BltOp::VideoFill {
                color: BltPixel::new(0, 0, 0),
                dest: (0, 0),
                dims: (self.field_width * TILE_SIZE, self.field_height * TILE_SIZE),
            })
            .unwrap();

        self.snake.respawn(self.gop);
        self.respawn_food();

        loop {
            self.input();

            if self.timer.ready() {
                let mut position = self.snake.next_position();

                self.bounds(&mut position);

                if self.food.position() == position {
                    self.snake.eat(self.gop, position);
                    self.respawn_food();
                } else {
                    self.snake.crawl(self.gop, position);

                    if self.snake.dead() {
                        self.snake.respawn(self.gop);

                        let position = self.food.position();
                        self.gop
                            .blt(BltOp::VideoFill {
                                color: BltPixel::new(0, 0, 0),
                                dest: (position.0 * TILE_SIZE, position.1 * TILE_SIZE),
                                dims: (TILE_SIZE, TILE_SIZE),
                            })
                            .unwrap();

                        self.respawn_food();
                    }
                }
            }
        }
    }

    fn input(&mut self) {
        while let Ok(Some(key)) = self.input.read_key() {
            if let Key::Printable(c16) = key {
                match char::from(c16) {
                    'w' => {
                        self.snake.up();
                    }
                    'a' => {
                        self.snake.left();
                    }
                    's' => {
                        self.snake.down();
                    }
                    'd' => {
                        self.snake.right();
                    }
                    _ => {}
                }
            }
        }
    }

    fn bounds(&self, position: &mut (usize, usize)) {
        if position.0 == usize::MAX {
            position.0 = self.field_width - 1;
        }
        if position.1 == usize::MAX {
            position.1 = self.field_height - 1;
        }
        if position.0 >= self.field_width {
            position.0 = 0;
        }
        if position.1 >= self.field_height {
            position.1 = 0;
        }
    }

    fn respawn_food(&mut self) {
        loop {
            let mut buffer = [0; 2];
            self.rng.get_rng(None, &mut buffer).unwrap();

            let position = (
                buffer[0] as usize % self.field_width,
                buffer[1] as usize % self.field_height,
            );

            if !self.snake.contains(position) {
                self.food.respawn(self.gop, position);
                break;
            }
        }
    }
}
