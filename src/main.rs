use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::input::keyboard::*;
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};
use rand::prelude::*;

use modulo::Mod;

use std::collections::LinkedList;
use std::iter::FromIterator;

const GRID_SIZE: (i32, i32) = (20, 20);
const GRID_CELL_SIZE: (i32, i32) = (25, 25);

struct GameState {
    snake: Snake,
    food: Food,
}

impl GameState {
    fn new() -> Self {
        GameState {
            snake: Snake::new(),
            food: Food::new(),
        }
    }

    fn reset(&mut self) {
        self.snake = Snake::new();
        self.food = Food::new();
    }
}

impl event::EventHandler for GameState {
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // clear window buffer
        graphics::clear(ctx, graphics::WHITE);
        // render food
        self.food.draw(ctx)?;
        // render snake
        self.snake.draw(ctx)?;
        // commit changes to window
        graphics::present(ctx)
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const TARGET_FPS: u32 = 10;
        // update game 8 times per second
        while timer::check_update_time(ctx, TARGET_FPS) {
            // update snake
            self.snake.update(ctx, &self.food)?;
            // check if snake ate something
            match self.snake.ate {
                Some(Ate::Food) => self.food = Food::new(),
                Some(Ate::Snake) => self.reset(),
                _ => {}
            };
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
            KeyCode::Left => self.snake.set_direction(Direction::Left),
            KeyCode::Right => self.snake.set_direction(Direction::Right),
            KeyCode::Up => self.snake.set_direction(Direction::Up),
            KeyCode::Down => self.snake.set_direction(Direction::Down),
            _ => (),
        }
    }
}

struct Snake {
    body: LinkedList<(i32, i32)>,
    direction: Direction,
    ate: Option<Ate>,
}

impl Snake {
    fn new() -> Snake {
        Snake {
            body: LinkedList::from_iter(vec![(5, 5), (4, 5), (3, 5)]),
            direction: Direction::Right,
            ate: None,
        }
    }

    fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // iterate throw snake's body
        for &(x, y) in self.body.iter() {
            // create square mesh
            let square = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new_i32(
                    x * GRID_CELL_SIZE.0,
                    y * GRID_CELL_SIZE.1,
                    GRID_CELL_SIZE.0,
                    GRID_CELL_SIZE.1,
                ),
                graphics::Color::from_rgb(255, 0, 0),
            )?;
            // draw square to canvas
            graphics::draw(ctx, &square, graphics::DrawParam::default())?;
        }
        Ok(())
    }

    fn update(&mut self, _ctx: &mut Context, food: &Food) -> GameResult {
        // get snake's head
        let prev_head = self.body.pop_front().unwrap();
        let mut head = prev_head.clone();

        match self.direction {
            Direction::Left => head.0 = (head.0 - 1).modulo(GRID_SIZE.0),
            Direction::Right => head.0 = (head.0 + 1).modulo(GRID_SIZE.0),
            Direction::Up => head.1 = (head.1 - 1).modulo(GRID_SIZE.1),
            Direction::Down => head.1 = (head.1 + 1).modulo(GRID_SIZE.1),
        };

        // reset if snake has eaten
        self.ate = None;

        // check if snake ate food
        if head.0 == food.x && head.1 == food.y {
            self.ate = Some(Ate::Food);
        }
        // check if snake ate his own body
        if self.body.contains(&head) {
            self.ate = Some(Ate::Snake);
        }

        // update snake's body
        self.body.push_front(prev_head);
        self.body.push_front(head);

        if self.ate == None {
            self.body.pop_back();
        }

        Ok(())
    }
}

struct Food {
    x: i32,
    y: i32,
}

impl Food {
    fn new() -> Food {
        let mut range = rand::thread_rng();
        let x = range.gen_range::<i32, _>(0..GRID_SIZE.0);
        let y = range.gen_range::<i32, _>(0..GRID_SIZE.1);
        Food { x, y }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // create square mesh
        let square = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new_i32(
                self.x * GRID_CELL_SIZE.0,
                self.y * GRID_CELL_SIZE.1,
                GRID_CELL_SIZE.0,
                GRID_CELL_SIZE.1,
            ),
            graphics::Color::from_rgb(246, 185, 59),
        )?;
        // draw square to canvas
        graphics::draw(ctx, &square, graphics::DrawParam::default())
    }
}

#[derive(PartialEq, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(PartialEq)]
enum Ate {
    Snake,
    Food,
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
