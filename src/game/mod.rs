use crossterm::event::{self, Event, KeyCode};
use crate::game::snake::Snake;
use crate::game::snake::Direction;
use crate::game::map::Map;
use crate::game::map::Cell;
use std::process::exit;
use std::time::{Duration, Instant};

pub mod snake;
pub mod map;

pub struct Game {
    apples: usize,
    map: Map,
    snake: Snake,
    last_update: Instant,
    update_delay: Duration,
}

impl Game {
    pub fn start_game(width: usize, height: usize) -> Self {
        let mut map = Map::new(width, height);
        let snake = Snake::new(width / 2, height / 2);

        let apples = 2;
        for _ in 0..apples {
            map.place_food();
        }

        Game { apples, map, snake, last_update: Instant::now(), update_delay: Duration::from_millis(150) }
    }

    pub fn get_map(&self) -> &Map {
        &self.map
    }

    pub fn get_snake(&mut self) -> &mut Snake {
        &mut self.snake
    }
 
    pub fn update(&mut self) {

        if self.last_update.elapsed() >= self.update_delay {
            let snake_coords = self.snake.get_body();
            self.map.apply_cells(snake_coords, Cell::Snake);
            Game::handle_input(&mut self.snake);
            self.snake.update();

            if self.map.count_food() < self.apples {
                self.map.place_food();
                self.snake.grow();
            }
            
            if self.is_game_over() {
                println!("Игра окончена!");
                exit(0);
            }
            self.last_update = Instant::now();
        }
    }

    fn is_game_over(&self) -> bool {
        let head = self.snake.head_position();
        if !self.map.is_inside(head.0, head.1) || self.snake.is_collision(head) {
            return true;
        }
        false
    }

    fn handle_input(snake: &mut Snake) {
        if event::poll(std::time::Duration::from_millis(0)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Up => snake.set_dir(Direction::Up),
                    KeyCode::Down => snake.set_dir(Direction::Down),
                    KeyCode::Left => snake.set_dir(Direction::Left),
                    KeyCode::Right => snake.set_dir(Direction::Right),
                    KeyCode::Char('q') => exit(0),
                    _ => {}
                }
            }
        }
    }
}