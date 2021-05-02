use bracket_lib::prelude::*;

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
    transform: Transform,
}

impl Player {
    pub fn new(pos_x: i32, pos_y: i32) -> Self {
        Player {
            transform: Transform { pos_x, pos_y },
            velocity: Velocity { y: 0.0 },
        }
    }

    pub fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(
            self.transform.pos_x,
            self.transform.pos_y,
            YELLOW,
            BLACK,
            to_cp437('@'),
        );
    }

    pub fn process_movement(&mut self){
        self.apply_gravity();
        self.move_player();
    }

    pub fn flap(&mut self){
        self.velocity.y = -2.0;
    }
    
    fn move_player(&mut self){
        self.transform.pos_y += self.velocity.y as i32;
        self.transform.pos_x += 1;

        if self.transform.pos_y < 0{
            self.transform.pos_y = 0;
        }
    }

    fn apply_gravity(&mut self){
        if self.velocity.y < 2.0 {
            self.velocity.y += 0.2;
        }
    }

}

impl Obstacle {
    pub fn new(pos_x: i32, pos_y: i32) -> Self {
        Obstacle {
            transform: Transform { pos_x, pos_y },
        }
    }
}
