mod entities;

use bracket_lib::prelude::*;
use entities::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;

pub enum GameMode {
    Menu,
    Playing,
    End,
}
pub struct State {
    mode: GameMode,
    player: Player,
    obstacle: Obstacle,
    frame_time: f32,
    score: i32,
}

impl State {
    pub fn new() -> Self {
        State {
            mode: GameMode::Menu,
            player: Player::new(5, 25),
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            frame_time: 0.0,
            score: 0,
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) play the game");
        ctx.print_centered(9, "(Q) quit the game");

        self.print_debug_info(ctx);

        self.handle_menu_input(ctx);
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;

        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.process_movement();
        }

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }

        if self.player.transform.pos_x > self.obstacle.pos_x {
            self.score += 1;
            self.obstacle = Obstacle::new(SCREEN_WIDTH + self.player.transform.pos_x, self.score);
        }

        self.player.render(ctx);
        self.obstacle.render(ctx, self.player.transform.pos_x);

        ctx.print(0, 0, "press SPACEBAR to flap");
        ctx.print(0, 1, &format!("Score: {}", self.score));

        if self.player.transform.pos_y > SCREEN_HEIGHT
            || self.obstacle.check_hit_obstacle(&self.player)
        {
            self.mode = GameMode::End;
        }
        self.print_debug_info(ctx);
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Game Over");
        ctx.print_centered(6, format!("You earned {} points", self.score));
        ctx.print_centered(8, "(P) play again");
        ctx.print_centered(9, "(Q) quit the game");

        self.print_debug_info(ctx);

        self.handle_menu_input(ctx);
    }

    fn restart(&mut self) {
        self.mode = GameMode::Playing;
        self.frame_time = 0.0;
        self.player = Player::new(5, 25);
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.score = 0;
    }

    fn handle_menu_input(&mut self, ctx: &mut BTerm) {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn print_debug_info(&mut self, ctx: &mut BTerm) {
        ctx.print(2, 2, format!("FPS: {}", ctx.fps));
        ctx.print(2, 3, format!("Frame time: {}", ctx.frame_time_ms));
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}
