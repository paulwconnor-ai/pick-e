use bevy::prelude::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CellState {
    Unknown,
    Free,
    Solid,
}

/// Per-entity occupancy grid
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
}
