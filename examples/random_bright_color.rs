extern crate bbggez;

use bbggez::{Timer, Utility};
use ggez::{
    event,
    event::EventHandler,
    graphics,
    graphics::{Color, DrawMode, MeshBuilder},
    nalgebra::Point2,
    Context, ContextBuilder, GameResult,
};

struct Game {
    utility: Utility,
    color: Color,
    timer: Timer,
}

impl Game {
    pub fn new(_context: &mut Context) -> Game {
        let mut utility = Utility::new();

        Game {
            utility,
            color: utility.random_bright_color(),
            timer: Timer::new(1.0),
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, context: &mut Context) -> GameResult<()> {
        self.timer.update(context);
        if self.timer.is_time_up() {
            self.color = self.utility.random_bright_color();
            self.timer.reset();
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context, graphics::BLACK);

        let (width, height) = graphics::drawable_size(context);

        let circle = MeshBuilder::new()
            .circle(
                DrawMode::fill(),
                Point2::new(width / 2.0, height / 2.0),
                200.0,
                0.1,
                self.color,
            )
            .build(context)?;

        graphics::draw(context, &circle, (Point2::new(0.0, 0.0),))?;

        graphics::present(context)
    }
}

fn main() {
    let (mut context, mut event_loop) = ContextBuilder::new("Random color example", "Brookzerker")
        .build()
        .expect("Game context was not able to be created");
    let mut game = Game::new(&mut context);

    match event::run(&mut context, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly"),
        Err(error) => println!("Error occured: {}", error),
    };
}
