use rand::Rng;
use ggez::graphics::{Color, DrawMode, DrawParam, Mesh, Canvas, Rect};
use ggez::{Context, GameResult};

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    Snake,
    Food,
    Wall,
}

pub struct Map {
    width: usize,
    height: usize,
    cells: Vec<Vec<Cell>>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = vec![vec![Cell::Empty; width + 1]; height + 1];
        Map { width, height, cells }
    }

    pub fn get_size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn refresh_snake(&mut self) {
        for row in &mut self.cells {
            for cell in row {
                if *cell == Cell::Snake {
                    *cell = Cell::Empty;
                }
            }
        }
    }

    pub fn apply_cells(&mut self, positions: &[(usize, usize)], cell_type: Cell) {
        self.refresh_snake();
        for &(x, y) in positions {
            if self.is_inside(x, y) {
                self.set_cell(x, y, cell_type);
            }
        }
    }

    pub fn is_inside(&self, x: usize, y: usize) -> bool {
        0 < x && x < self.width && 0 < y && y < self.height
    }

    pub fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        self.cells[y][x] = cell;
    }

    pub fn place_food(&mut self) {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..=self.width);
        let y = rng.gen_range(0..=self.height);
        self.set_cell(x, y, Cell::Food);
    }

    pub fn count_food(&self) -> usize {
        self.cells.iter().flatten().filter(|&&cell| cell == Cell::Food).count()
    }

    pub fn render_graphics(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        let cell_size = 20.0;

        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let color = match cell {
                    Cell::Empty => Color::new(0.1, 0.1, 0.1, 1.0),
                    Cell::Snake => Color::GREEN,
                    Cell::Food => Color::RED,
                    Cell::Wall => Color::BLUE,
                };

                let rectangle = Rect::new(
                    x as f32 * cell_size,
                    y as f32 * cell_size,
                    cell_size,
                    cell_size,
                );

                let mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), rectangle, color)?;
                canvas.draw(&mesh, DrawParam::default());
            }
        }

        Ok(())
    }
}