use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::{Context, ContextBuilder, GameResult};

struct MainState {}

impl MainState {
    fn new() -> Self {
        MainState {}
    }
}

impl event::EventHandler for MainState {
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // clear background
        graphics::clear(ctx, graphics::WHITE);

        // draw square
        let square = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, 25.0, 25.0),
            graphics::Color::new(1.0, 0.0, 0.0, 1.0),
        )?;
        graphics::draw(ctx, &square, graphics::DrawParam::default())?;

        // commit changes to window
        graphics::present(ctx)
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
}

fn main() -> GameResult {
    // create game context
    let (ctx, event_loop) = &mut ContextBuilder::new("snake_game", "aa-studios")
        .window_setup(conf::WindowSetup::default().title("Snake Game"))
        .window_mode(
            conf::WindowMode::default()
                .dimensions(500_f32, 500_f32)
                .resizable(false),
        )
        .build()?;

    // initialize game state and run game-loop
    let state = &mut MainState::new();
    event::run(ctx, event_loop, state)
}
