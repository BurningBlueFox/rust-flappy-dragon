use bracket_lib::prelude::*;

use super::SCREEN_HEIGHT;

pub struct Transform {
    pub pos_x: i32,
    pub pos_y: i32,
}

struct Velocity {
    y: f64,
}

pub struct Player {
    pub transform: Transform,
    velocity: Velocity,
}
pub struct Obstacle {
    pub pos_x: i32,
    gap_y: i32,
    size: i32,
}

impl Player {
    pub fn new(pos_x: i32, pos_y: i32) -> Self {
        Player {
            transform: Transform { pos_x, pos_y },
            velocity: Velocity { y: 0.0 },
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set(0, self.transform.pos_y, YELLOW, BLACK, to_cp437('@'));
    }

    pub fn process_movement(&mut self) {
        self.apply_gravity();
        self.move_player();
    }

    pub fn flap(&mut self) {
        self.velocity.y = -2.0;
    }

    fn move_player(&mut self) {
        self.transform.pos_y += self.velocity.y as i32;
        self.transform.pos_x += 1;

        if self.transform.pos_y < 0 {
            self.transform.pos_y = 0;
        }
    }

    fn apply_gravity(&mut self) {
        if self.velocity.y < 2.0 {
            self.velocity.y += 0.2;
        }
    }
}

impl Obstacle {
    pub fn new(pos_x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();

        Obstacle {
            pos_x,
            gap_y: random.range(10, 40),
            size: i32::max(2, 20 - score),
        }
    }

    pub fn render(&self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.pos_x - player_x;
        let half_size: i32 = self.size / 2;

        //Draw the top part of the obstacle
        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('/'));
        }

        //Draw the botton half of the obstacle
        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('/'));
        }
    }

    pub fn check_hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let does_x_match = player.transform.pos_x == self.pos_x;
        let player_above_gap = player.transform.pos_y < self.gap_y - half_size;
        let player_below_gap = player.transform.pos_y > self.gap_y + half_size;

        does_x_match && (player_above_gap || player_below_gap)
    }
}
