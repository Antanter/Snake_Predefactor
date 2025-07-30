#[derive(Clone, Copy)]
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
        Snake { body: vec![(width, height)], direction: Direction::Right, grow_next: false, }
    }

    fn is_opposite(dir1: Direction, dir2: Direction) -> bool {
        matches!(
            (dir1, dir2),
            (Direction::Up, Direction::Down) |
            (Direction::Down, Direction::Up) |
            (Direction::Left, Direction::Right) |
            (Direction::Right, Direction::Left)
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

    pub fn update(&mut self) {
        let (head_x, head_y) = self.head_position();
        
        let new_head = match self.direction {
            Direction::Up => (head_x, head_y.saturating_sub(1)),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x.saturating_sub(1), head_y),
            Direction::Right => (head_x + 1, head_y),
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

