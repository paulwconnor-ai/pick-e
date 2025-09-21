use bevy::prelude::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CellState {
    Unknown,
    Free,
    Solid,
}

#[derive(Component)]
pub struct OccupancyGrid {
    pub resolution: f32, // meters per cell
    pub width: usize,
    pub height: usize,
    pub origin: Vec2,          // world-space origin of (0,0) in grid
    pub cells: Vec<CellState>, // flat grid: y * width + x
}

impl OccupancyGrid {
    pub fn new(width: usize, height: usize, resolution: f32, origin: Vec2) -> Self {
        Self {
            resolution,
            width,
            height,
            origin,
            cells: vec![CellState::Unknown; width * height],
        }
    }

    pub fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn set(&mut self, x: usize, y: usize, state: CellState) {
        if x < self.width && y < self.height {
            let idx = self.index(x, y);
            self.cells[idx] = state;
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<CellState> {
        if x < self.width && y < self.height {
            Some(self.cells[self.index(x, y)])
        } else {
            None
        }
    }

    /// New: get using IVec2
    pub fn get_cell(&self, cell: IVec2) -> Option<CellState> {
        let x = cell.x;
        let y = cell.y;
        if x >= 0 && y >= 0 {
            self.get(x as usize, y as usize)
        } else {
            None
        }
    }

    /// Converts from world position to grid cell
    pub fn world_to_cell(&self, pos: Vec2) -> Option<IVec2> {
        let rel = (pos - self.origin) / self.resolution;
        let x = rel.x.floor() as i32;
        let y = rel.y.floor() as i32;
        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            Some(IVec2::new(x, y))
        } else {
            None
        }
    }

    /// Converts from grid cell to world center position
    pub fn cell_to_world(&self, cell: IVec2) -> Vec2 {
        self.origin + (cell.as_vec2() + Vec2::splat(0.5)) * self.resolution
    }

    /// New: iterate over all (cell, state) pairs
    pub fn iter(&self) -> impl Iterator<Item = (IVec2, CellState)> + '_ {
        self.cells.iter().enumerate().map(move |(i, state)| {
            let x = (i % self.width) as i32;
            let y = (i / self.width) as i32;
            (IVec2::new(x, y), *state)
        })
    }
}
