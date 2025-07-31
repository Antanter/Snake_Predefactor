use std::collections::HashMap;
use rand::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
pub struct Snake {
    body: Vec<(usize, usize)>,
    direction: Direction,
    grow_next: bool,
}

impl Snake {
    pub fn new(width: usize, height: usize) -> Self {
        Snake {
            body: vec![(width, height)],
            direction: Direction::Right,
            grow_next: false,
        }
    }

    fn is_opposite(dir1: Direction, dir2: Direction) -> bool {
        matches!(
            (dir1, dir2),
            (Direction::Up, Direction::Down)
                | (Direction::Down, Direction::Up)
                | (Direction::Left, Direction::Right)
                | (Direction::Right, Direction::Left)
        )
    }

    pub fn set_dir(&mut self, dir: Direction) {
        if !Self::is_opposite(self.direction, dir) {
            self.direction = dir;
        }
    }

    pub fn get_dir(&self) -> &Direction {
        &self.direction
    }

    pub fn get_body(&self) -> &Vec<(usize, usize)> {
        &self.body
    }

    pub fn update(&mut self, (x, y): (usize, usize)) {
        let (head_x, head_y) = self.head_position();

        let new_head = match self.direction {
            Direction::Up => (head_x, (head_y + y - 1) % y),
            Direction::Down => (head_x, (head_y + 1) % y),
            Direction::Left => ((head_x + x - 1) % x, head_y),
            Direction::Right => ((head_x + 1) % x, head_y),
        };

        self.body.insert(0, new_head);

        if self.grow_next {
            self.grow_next = false;
        } else {
            self.body.pop();
        }
    }

    pub fn grow(&mut self) {
        self.grow_next = true;
    }

    pub fn head_position(&self) -> (usize, usize) {
        self.body[0]
    }

    pub fn is_collision(&self, pos: (usize, usize)) -> bool {
        self.body.iter().skip(1).any(|&p| p == pos)
    }

    pub fn occupies(&self, x: usize, y: usize) -> bool {
        self.body.contains(&(x, y))
    }
}

pub type State = [u8; 7];

pub struct QLearningSnake {
    q_table: HashMap<(State, Direction), f32>,
    alpha: f32,
    gamma: f32,
    epsilon: f32,
    last_state: Option<State>,
    last_action: Option<Direction>,
}

impl QLearningSnake {
    pub fn new() -> Self {
        QLearningSnake {
            q_table: HashMap::new(),
            alpha: 0.1,
            gamma: 0.9,
            epsilon: 0.1,
            last_state: None,
            last_action: None,
        }
    }

    pub fn decide(&mut self, state: State) -> Direction {
        let mut rng = thread_rng();
        let raw: u32 = rng.gen_range(0..=10_000_000);

        if (raw as f32) / 10_000_000.0 < self.epsilon {
            let actions = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];
            *actions.choose(&mut rng).unwrap()
        } else {
            self.best_action(state)
        }
    }

    fn best_action(&self, state: State) -> Direction {
        let actions = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];
        *actions
            .iter()
            .max_by(|a, b| {
                let qa = *self.q_table.get(&(state, **a)).unwrap_or(&0.0);
                let qb = *self.q_table.get(&(state, **b)).unwrap_or(&0.0);
                qa.partial_cmp(&qb).unwrap()
            })
            .unwrap()
    }

    pub fn learn(&mut self, state: State, reward: f32) {
        if let (Some(prev_state), Some(prev_action)) = (self.last_state, self.last_action) {
            let max_future_q = {
                let actions = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];
                actions
                    .iter()
                    .map(|a| *self.q_table.get(&(state, *a)).unwrap_or(&0.0))
                    .fold(f32::MIN, f32::max)
            };

            let old_q = *self.q_table.get(&(prev_state, prev_action)).unwrap_or(&0.0);
            let new_q = old_q + self.alpha * (reward + self.gamma * max_future_q - old_q);

            self.q_table.insert((prev_state, prev_action), new_q);
        }

        self.last_state = Some(state);
    }

    pub fn remember_action(&mut self, action: Direction) {
        self.last_action = Some(action);
    }

    pub fn encode_state(&self, snake: &Snake, food: (usize, usize), map_size: (usize, usize)) -> State {
        let (map_width, map_height) = map_size;
        let head = snake.head_position();
        let dir = *snake.get_dir();

        let next_pos = |dir: Direction| -> (usize, usize) {
            match dir {
                Direction::Up => (head.0, (head.1 + map_height - 1) % map_height),
                Direction::Down => (head.0, (head.1 + 1) % map_height),
                Direction::Left => ((head.0 + map_width - 1) % map_width, head.1),
                Direction::Right => ((head.0 + 1) % map_width, head.1),
            }
        };

        let is_danger = |pos: (usize, usize)| -> bool {
            snake.is_collision(pos)
        };

        let danger_ahead = is_danger(next_pos(dir)) as u8;
        let danger_left = is_danger(next_pos(Self::turn_left(dir))) as u8;
        let danger_right = is_danger(next_pos(Self::turn_right(dir))) as u8;

        let food_up = (food.1 < head.1) as u8;
        let food_down = (food.1 > head.1) as u8;
        let food_left = (food.0 < head.0) as u8;
        let food_right = (food.0 > head.0) as u8;

        [
            danger_ahead,
            danger_left,
            danger_right,
            food_up,
            food_down,
            food_left,
            food_right,
        ]
    }

    fn turn_left(dir: Direction) -> Direction {
        match dir {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn turn_right(dir: Direction) -> Direction {
        match dir {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}
