use std::{cmp::max, time::Duration};

use rusty_time::timer::Timer;

use crate::{frame::Drawable, NUM_COLS, NUM_ROWS};

pub struct Invador {
    pub x: usize,
    pub y: usize,
    pub dead: bool,
}

pub struct Invadors {
    pub army: Vec<Invador>,
    move_timer: Timer,
    direction: i32,
}

impl Invadors {
    pub fn new() -> Self {
        let mut army = Vec::new();
        for x in 0..NUM_COLS {
            for y in 0..NUM_ROWS {
                if (x > 1)
                    && (x < NUM_COLS - 2)
                    && (y > 0)
                    && (y < 9)
                    && (x % 2 == 0)
                    && (y % 2 == 0)
                {
                    army.push(Invador { x, y, dead: false });
                }
            }
        }
        Self {
            army,
            move_timer: Timer::from_millis(2000),
            direction: 1,
        }
    }
    pub fn update(&mut self, delta: Duration) -> bool {
        self.move_timer.update(delta);
        if self.move_timer.ready {
            self.move_timer.reset();
            let mut downwards = false;
            if self.direction == -1 {
                let min_x = self.army.iter().map(|invador| invador.x).min().unwrap_or(0);
                if min_x == 0 {
                    self.direction = 1;
                    downwards = true;
                }
            } else {
                let max_x = self.army.iter().map(|invador| invador.x).max().unwrap_or(0);
                if max_x == NUM_COLS - 1 {
                    self.direction = -1;
                    downwards = true;
                }
            }

            if downwards {
                let new_duration = max(self.move_timer.duration.as_millis() - 250, 250);
                self.move_timer = Timer::from_millis(new_duration as u64);
                for invador in self.army.iter_mut() {
                    invador.y += 1;
                }
            } else {
                for invador in self.army.iter_mut() {
                    invador.x = ((invador.x as i32) + self.direction) as usize;
                }
            }

            return true;
        }

        false
    }
    pub fn all_killed(&self) -> bool {
        self.army.is_empty()
    }

    pub fn reached_button(&self) -> bool {
        self.army.iter().map(|invador| invador.y).max().unwrap_or(0) >= NUM_ROWS - 1
    }

    pub fn kill_invador_at(&mut self, x: usize, y: usize) -> bool {
        if let Some(idx) = self
            .army
            .iter()
            .position(|invador| invador.x == x && invador.y == y)
        {
            self.army.remove(idx);

            true
        } else {
            false
        }
    }
}

impl Drawable for Invadors {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        for invador in self.army.iter() {
            frame[invador.x][invador.y] = if (self.move_timer.time_left.as_secs_f32()
                / self.move_timer.duration.as_secs_f32())
                > 0.5
            {
                "x"
            } else {
                "+"
            };
        }
    }
}
