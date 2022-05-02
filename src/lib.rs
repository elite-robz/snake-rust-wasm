use wasm_bindgen::prelude::*;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_idx: usize) -> World {
        World {
            width,
            size: width * width,
            snake: Snake::new(snake_idx),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn update(&mut self) {
        let snake_idx = self.snake_head();
        let row = snake_idx / self.width;

        if self.snake.direction == Direction::Right {
            let next_col = (snake_idx + 1) % self.width;
            self.snake.body[0].0 = (row * self.width) + next_col;
        }
        if self.snake.direction == Direction::Left {
            let next_col = (snake_idx - 1) % self.width;
            self.snake.body[0].0 = (row * self.width) + next_col;
        }
        if self.snake.direction == Direction::Down {
            self.snake.body[0].0 = (snake_idx + self.width) % (self.size);
        }
        if self.snake.direction == Direction::Up {
            self.snake.body[0].0 = (snake_idx - self.width) % (self.size);
        }
    }
}

struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    fn new(spawn_index: usize) -> Snake {
        Snake {
            body: vec![SnakeCell(spawn_index)],
            direction: Direction::Left,
        }
    }
}
#[derive(PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
// wasm-pack build --target web