use bevy::prelude::*;

#[derive(Component)]
pub struct VisitedGrid {
    pub visited: Vec<bool>,
    pub width: usize,
    pub height: usize,
}

impl VisitedGrid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            visited: vec![false; width * height],
            width,
            height,
        }
    }

    pub fn mark(&mut self, cell: IVec2) {
        if let Some(i) = self.index(cell) {
            self.visited[i] = true;
        }
    }

    pub fn is_marked(&self, cell: IVec2) -> bool {
        self.index(cell).map_or(false, |i| self.visited[i])
    }

    fn index(&self, cell: IVec2) -> Option<usize> {
        if cell.x >= 0 && cell.y >= 0 {
            let x = cell.x as usize;
            let y = cell.y as usize;
            if x < self.width && y < self.height {
                return Some(y * self.width + x);
            }
        }
        None
    }
}
