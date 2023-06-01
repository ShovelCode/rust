use crossterm::{
    event::{poll, read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    ExecutableCommand,
};
use std::io::{stdout, Write};
use std::time::{Duration, Instant};

const WIDTH: u16 = 20;
const HEIGHT: u16 = 10;

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

struct Game {
    snake: Vec<Point>,
    food: Point,
    direction: Direction,
    game_over: bool,
}

impl Game {
    fn new() -> Game {
        Game {
            snake: vec![Point { x: 2, y: 2 }],
            food: Point { x: 10, y: 5 },
            direction: Direction::Right,
            game_over: false,
        }
    }

    fn update(&mut self) {
        let mut new_head = self.snake[0];
        match self.direction {
            Direction::Up => new_head.y -= 1,
            Direction::Down => new_head.y += 1,
            Direction::Left => new_head.x -= 1,
            Direction::Right => new_head.x += 1,
        }

        if self.snake.contains(&new_head)
            || new_head.x < 0
            || new_head.x >= WIDTH as i32
            || new_head.y < 0
            || new_head.y >= HEIGHT as i32
        {
            self.game_over = true;
            return;
        }

        self.snake.insert(0, new_head);

        if self.snake[0].x == self.food.x && self.snake[0].y == self.food.y {
            self.food = self.generate_food();
        } else {
            self.snake.pop();
        }
    }

    fn generate_food(&self) -> Point {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut x;
        let mut y;
        loop {
            x = rng.gen_range(0..WIDTH);
            y = rng.gen_range(0..HEIGHT);
            if !self.snake.iter().any(|&p| p.x == x && p.y == y) {
                break;
            }
        }
        Point { x, y }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = stdout();

    let mut game = Game::new();
    let mut last_update_time = Instant::now();

    loop {
        if poll(Duration::from_millis(50))? {
            if let Event::Key(key_event) = read()? {
                match key_event.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up => game.direction = Direction::Up,
                    KeyCode::Down => game.direction = Direction::Down,
                    KeyCode::Left => game.direction = Direction::Left,
                    KeyCode::Right => game.direction = Direction::Right,
                    _ => {}
                }
            }
        }

        if last_update_time.elapsed() >= Duration::from_millis(500) {
            game.update();
            last_update_time = Instant::now();
        }

        if game.game_over {
            break;
        }

        stdout.execute(Clear(ClearType::All))?;
        draw(&mut stdout, &game)?;
        stdout.flush()?;
    }

    disable_raw_mode()?;
    Ok(())
