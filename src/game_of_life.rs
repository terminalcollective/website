// Adopted from https://github.com/sinon/game-of-life

use std::{
    fmt::{self, Debug, Display},
    ops::{Add, Index},
};

type Coord = i32;

pub const NORTH: Point = Point::new(0, -1);
pub const NORTH_EAST: Point = Point::new(1, -1);
pub const EAST: Point = Point::new(1, 0);
pub const SOUTH_EAST: Point = Point::new(1, 1);
pub const SOUTH: Point = Point::new(0, 1);
pub const SOUTH_WEST: Point = Point::new(-1, 1);
pub const WEST: Point = Point::new(-1, 0);
pub const NORTH_WEST: Point = Point::new(-1, -1);

pub const ORTHO_PLUS_DIR: [Point; 8] = [
    NORTH, NORTH_EAST, EAST, SOUTH_EAST, SOUTH, SOUTH_WEST, WEST, NORTH_WEST,
];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Point {
    pub x: Coord,
    pub y: Coord,
}
impl AsRef<Point> for Point {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Add for Point {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Point {
    #[inline]
    #[must_use]
    pub const fn new(x: Coord, y: Coord) -> Self {
        Point { x, y }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum CellState {
    Alive,
    Dead,
}

impl Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CellState::Dead => {
                write!(f, " ")?;
            }
            CellState::Alive => {
                write!(f, "0")?;
            }
        }
        Ok(())
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
struct NeighbourState {
    dead: i32,
    alive: i32,
}

#[derive(Debug)]
pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<T>,
}

impl<T> Grid<T> {
    pub fn contains(&self, p: &Point) -> bool {
        p.x >= 0 && (p.x as usize) < self.width && p.y >= 0 && (p.y as usize) < self.height
    }

    fn pos(&self, p: usize) -> Point {
        Point::new((p % self.width) as i32, (p / self.width) as i32)
    }
    fn idx(&self, p: &Point) -> usize {
        ((self.width as i32) * p.y + p.x) as usize
    }

    pub fn try_get<U: AsRef<Point>>(&self, p: U) -> Option<&T> {
        if self.contains(p.as_ref()) {
            Some(&self[*p.as_ref()])
        } else {
            None
        }
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, pos: Point) -> &Self::Output {
        &self.cells[self.idx(&pos)]
    }
}

impl Grid<CellState> {
    pub fn new_empty(width: usize, height: usize) -> Self {
        let size = width * height;
        let cells: Vec<CellState> = (0..size).map(|_| CellState::Dead).collect();
        Grid {
            width,
            height,
            cells,
        }
    }

    pub fn new_random(width: usize, height: usize) -> Self {
        let size = width * height;
        let cells: Vec<CellState> = (0..size)
            .map(|_| {
                if fastrand::bool() {
                    CellState::Alive
                } else {
                    CellState::Dead
                }
            })
            .collect();
        Grid {
            width,
            height,
            cells,
        }
    }
    pub fn update_states(&mut self) -> u32 {
        let mut new_grid: Vec<CellState> = Vec::new();
        for (idx, &cell) in self.cells.iter().enumerate() {
            let state = self.get_neighbours_state(self.pos(idx));
            let cellstate = self.get_cell_state(&cell, state);
            new_grid.push(cellstate);
        }
        self.cells = new_grid;
        self.cells
            .iter()
            .filter(|&&c| c == CellState::Alive)
            .count() as u32
    }
    /*
    Any live cell with 0 or 1 live neighbors becomes dead, because of underpopulation
    Any live cell with 2 or 3 live neighbors stays alive, because its neighborhood is just right
    Any live cell with more than 3 live neighbors becomes dead, because of overpopulation
    Any dead cell with exactly 3 live neighbors becomes alive, by reproduction
     */
    fn get_cell_state(&self, cell: &CellState, state: NeighbourState) -> CellState {
        match (&cell, state.alive) {
            (CellState::Alive, 0..=1) => CellState::Dead,
            (CellState::Alive, 2..=3) => CellState::Alive,
            (CellState::Alive, 4..=8) => CellState::Dead,
            (CellState::Dead, 3) => CellState::Alive,
            (_, _) => *cell,
        }
    }
    fn get_neighbours_state(&self, point: Point) -> NeighbourState {
        let mut alive = 0;
        let mut dead = 0;
        for neighbour in self.get_neighbours(point).map(|p| self.try_get(p)) {
            match neighbour {
                Some(c) => match c {
                    CellState::Alive => alive += 1,
                    CellState::Dead => dead += 1,
                },
                None => {
                    continue;
                }
            }
        }
        NeighbourState { alive, dead }
    }

    fn get_neighbours(&self, point: Point) -> impl Iterator<Item = Point> + use<'_> {
        ORTHO_PLUS_DIR
            .into_iter()
            .map(move |d| point + d)
            .filter(|p| self.contains(p))
    }
}

impl Default for Grid<CellState> {
    fn default() -> Self {
        Grid::new_empty(10, 10)
    }
}

impl Display for Grid<CellState> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.height {
            for w in row * self.width..(row + 1) * self.width {
                write!(f, "{}", self.cells[w])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_try_get() {
        let g = Grid::new_empty(0, 0);
        assert!(g.try_get(Point { x: 10, y: 10 }) == None);
    }

    #[test]
    fn test_grid_new_random() {
        let rand_g = Grid::new_random(10, 10);
        assert_eq!(rand_g.cells.len(), 100);
    }

    #[test]
    fn test_get_neighbours_state() {
        let mut g = Grid::new_empty(3, 3);
        g.cells[1] = CellState::Alive;
        // x 0 x
        // x x x
        // x x x
        let state = g.get_neighbours_state(Point { x: 0, y: 0 });
        assert_eq!(state.dead, 2);
        assert_eq!(state.alive, 1);
    }

    #[test]
    fn test_get_neighbours_state_unknown_point() {
        let g = Grid::new_empty(3, 3);
        let state = g.get_neighbours_state(Point { x: 5, y: 5 });
        assert_eq!(state.dead, 0);
        assert_eq!(state.alive, 0);
    }

    #[test]
    fn test_grid_display() {
        let mut g = Grid::new_empty(3, 3);
        g.cells[4] = CellState::Alive;
        let s = format!("{}", g);
        assert_eq!(s, "💀💀💀\n💀😇💀\n💀💀💀\n".to_string());
    }

    #[test]
    fn test_grid_debug() {
        let mut g = Grid::new_empty(3, 3);
        g.cells[4] = CellState::Alive;
        let s = format!("{:?}", g);
        assert_eq!(s, "Grid { width: 3, height: 3, cells: [Dead, Dead, Dead, Dead, Alive, Dead, Dead, Dead, Dead] }".to_string());
    }

    #[test]
    fn test_update_state() {
        let mut g = Grid::new_random(10, 10);
        g.update_states();
    }

    #[test]
    fn test_get_cell_state() {
        let g = Grid::new_empty(3, 3);
        // Any live cell with 0 or 1 live neighbors becomes dead, because of underpopulation
        assert_eq!(
            g.get_cell_state(&CellState::Alive, NeighbourState { alive: 1, dead: 0 }),
            CellState::Dead
        );
        //Any live cell with 2 or 3 live neighbors stays alive, because its neighborhood is just right
        assert_eq!(
            g.get_cell_state(&CellState::Alive, NeighbourState { alive: 3, dead: 0 }),
            CellState::Alive
        );
        // Any live cell with more than 3 live neighbors becomes dead, because of overpopulation
        assert_eq!(
            g.get_cell_state(&CellState::Alive, NeighbourState { alive: 5, dead: 1 }),
            CellState::Dead
        );
        // Any dead cell with exactly 3 live neighbors becomes alive, by reproduction
        assert_eq!(
            g.get_cell_state(&CellState::Dead, NeighbourState { alive: 3, dead: 0 }),
            CellState::Alive
        );
    }
}
