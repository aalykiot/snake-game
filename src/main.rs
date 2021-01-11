use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::input::keyboard::*;
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};

struct GameState {
    snake: Snake,
}

impl GameState {
    fn new() -> Self {
        GameState {
            snake: Snake::new(),
        }
    }
}

impl event::EventHandler for GameState {
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // clear window buffer
        graphics::clear(ctx, graphics::WHITE);
        // render snake
        self.snake.draw(ctx)?;
        // commit changes to window
        graphics::present(ctx)
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const TARGET_FPS: u32 = 8;
        // update game 8 times per second
        while timer::check_update_time(ctx, TARGET_FPS) {
            self.snake.update(ctx)?;
        }
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Left => self.snake.dir = Direction::Left,
            KeyCode::Right => self.snake.dir = Direction::Right,
            KeyCode::Up => self.snake.dir = Direction::Up,
            KeyCode::Down => self.snake.dir = Direction::Down,
            _ => (),
        }
    }
}

struct Snake {
    pos_x: i32,
    pos_y: i32,
    dir: Direction,
}

impl Snake {
    fn new() -> Snake {
        Snake {
            pos_x: 0,
            pos_y: 0,
            dir: Direction::Right,
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // draw the head os snake
        let square = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new_i32(self.pos_x * 25, self.pos_y * 25, 25, 25),
            graphics::Color::from_rgb(255, 0, 0),
        )?;

        graphics::draw(ctx, &square, graphics::DrawParam::default())
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        match self.dir {
            Direction::Left => self.pos_x -= 1,
            Direction::Right => self.pos_x += 1,
            Direction::Up => self.pos_y -= 1,
            Direction::Down => self.pos_y += 1,
        };
        Ok(())
    }
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
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
    let state = &mut GameState::new();
    event::run(ctx, event_loop, state)
}
