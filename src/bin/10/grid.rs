use std::cmp::Ordering;

use crate::grid::Dirs::{East, North, South, West};
use crate::Solution;

pub type CostType = i8;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FieldType {
    Pipe(Dirs, Dirs),
    None,
    Start,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Dirs {
    North = 1,
    East = 2,
    South = 4,
    West = 8,
}
pub type MaskType = bool;

#[derive(Debug, Clone)]
pub struct Grid {
    pub fields: Vec<Vec<(CostType, FieldType, MaskType)>>,
    size: (usize, usize),
}

impl Grid {
    pub fn new(size: (usize, usize)) -> Grid {
        Grid {
            fields: vec![vec![(0, FieldType::None, false); size.1]; size.0],
            size,
        }
    }

    pub fn get_size(&self) -> (usize, usize) {
        self.size.clone()
    }

    pub(self) fn is_field_in_bounds(&self, y: Solution, x: Solution) -> bool {
        y >= 0 && x >= 0 && y < self.size.0 as i32 && x < self.size.1 as Solution
    }

    pub fn are_fields_connected(
        current_field: &FieldType,
        other_field: &FieldType,
        direction_of_other_field: Dirs,
    ) -> bool {
        use Dirs::*;
        match other_field {
            FieldType::Pipe(o_d1, o_d2) => match current_field {
                FieldType::Pipe(d1, d2) => match direction_of_other_field {
                    North => (*d1 == North || *d2 == North) && (*o_d1 == South || *o_d2 == South),
                    East => (*d1 == East || *d2 == East) && (*o_d1 == West || *o_d2 == West),
                    South => (*d1 == South || *d2 == South) && (*o_d1 == North || *o_d2 == North),
                    West => (*d1 == West || *d2 == West) && (*o_d1 == East || *o_d2 == East),
                },
                FieldType::None => false,
                FieldType::Start => match direction_of_other_field {
                    North => *o_d1 == South || *o_d2 == South,
                    East => *o_d1 == West || *o_d2 == West,
                    South => *o_d1 == North || *o_d2 == North,
                    West => *o_d1 == East || *o_d2 == East,
                },
            },
            FieldType::None => false,
            FieldType::Start => match current_field {
                FieldType::Pipe(d1, d2) => match direction_of_other_field {
                    North => *d1 == North || *d2 == North,
                    East => *d1 == East || *d2 == East,
                    South => *d1 == South || *d2 == South,
                    West => *d1 == West || *d2 == West,
                },
                FieldType::None => false,
                FieldType::Start => true,
            },
        }
    }

    pub fn is_valid_connection(c1: &Dirs, c2: &Dirs) -> bool {
        match *c1 as u8 + *c2 as u8 {
            5 => true,
            10 => true,
            _ => false,
        }
    }

    pub fn set_field_type(&mut self, y: usize, x: usize, field_type: FieldType) {
        if !self.is_field_in_bounds(y as Solution, x as Solution) {
            return;
        }

        self.fields[y][x].1 = field_type;
    }

    pub fn mark_field(&mut self, y: usize, x: usize) {
        self.fields[y][x].2 = true;
    }

    pub fn is_field_marked(&self, y: usize, x: usize) -> bool {
        self.fields[y][x].2
    }

    pub fn get_left(&self, y: usize, x: usize) -> Option<bool> {
        if !self.is_field_in_bounds(y as Solution, x as Solution - 1) {
            return None;
        }

        Some(Self::are_fields_connected(
            &self.fields[y][x].1,
            &self.fields[y][x - 1].1,
            West,
        ))
    }

    pub fn get_right(&self, y: usize, x: usize) -> Option<bool> {
        if !self.is_field_in_bounds(y as Solution, x as Solution + 1) {
            return None;
        }

        Some(Self::are_fields_connected(
            &self.fields[y][x].1,
            &self.fields[y][x + 1].1,
            East,
        ))
    }

    pub fn get_top(&self, y: usize, x: usize) -> Option<bool> {
        if !self.is_field_in_bounds(y as Solution - 1, x as Solution) {
            return None;
        }

        Some(Self::are_fields_connected(
            &self.fields[y][x].1,
            &self.fields[y - 1][x].1,
            North,
        ))
    }

    pub fn get_bottom(&self, y: usize, x: usize) -> Option<bool> {
        if !self.is_field_in_bounds(y as Solution + 1, x as Solution) {
            return None;
        }

        Some(Self::are_fields_connected(
            &self.fields[y][x].1,
            &self.fields[y + 1][x].1,
            South,
        ))
    }

    pub fn get_left_unmarked(&self, y: usize, x: usize) -> Option<(usize, usize)> {
        if self.is_field_in_bounds(y as Solution, x as Solution - 1)
            && !self.is_field_marked(y, x - 1)
        {
            return Some((y, x - 1));
        }
        None
    }

    pub fn get_right_unmarked(&self, y: usize, x: usize) -> Option<(usize, usize)> {
        if self.is_field_in_bounds(y as Solution, x as Solution + 1)
            && !self.is_field_marked(y, x + 1)
        {
            return Some((y, x + 1));
        }
        None
    }

    pub fn get_top_unmarked(&self, y: usize, x: usize) -> Option<(usize, usize)> {
        if self.is_field_in_bounds(y as Solution - 1, x as Solution)
            && !self.is_field_marked(y - 1, x)
        {
            return Some((y - 1, x));
        }
        None
    }

    pub fn get_bottom_unmarked(&self, y: usize, x: usize) -> Option<(usize, usize)> {
        if self.is_field_in_bounds(y as Solution + 1, x as Solution)
            && !self.is_field_marked(y + 1, x)
        {
            return Some((y + 1, x));
        }
        None
    }

    pub fn get_connected_unvisited_neighbours(&self, y: usize, x: usize) -> Vec<(usize, usize)> {
        let mut neighbours = Vec::new();
        let (left_o, top_o, right_o, bottom_o) = (
            self.get_left(y, x),
            self.get_top(y, x),
            self.get_right(y, x),
            self.get_bottom(y, x),
        );
        if let Some(true) = left_o {
            if !self.is_field_marked(y, x - 1) {
                neighbours.push((y, x - 1));
            }
        };
        if let Some(true) = right_o {
            if !self.is_field_marked(y, x + 1) {
                neighbours.push((y, x + 1));
            }
        };
        if let Some(true) = top_o {
            if !self.is_field_marked(y - 1, x) {
                neighbours.push((y - 1, x));
            }
        };
        if let Some(true) = bottom_o {
            if !self.is_field_marked(y + 1, x) {
                neighbours.push((y + 1, x));
            }
        };

        neighbours
    }

    pub fn get_unmarked_neighbours(&self, y: usize, x: usize) -> Vec<(usize, usize)> {
        let mut neighbours = Vec::new();
        let (left_o, top_o, right_o, bottom_o) = (
            self.get_left_unmarked(y, x),
            self.get_top_unmarked(y, x),
            self.get_right_unmarked(y, x),
            self.get_bottom_unmarked(y, x),
        );
        if let Some(n) = left_o {
            neighbours.push(n);
        };
        if let Some(n) = right_o {
            neighbours.push(n);
        };
        if let Some(n) = top_o {
            neighbours.push(n);
        };
        if let Some(n) = bottom_o {
            neighbours.push(n);
        };

        neighbours
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Field {
    pub coordinate: (usize, usize),
    pub path_length: Solution,
    pub field_type: FieldType,
}

impl Field {
    pub fn new(coordinate: (usize, usize), field_type: FieldType, path_length: Solution) -> Field {
        Field {
            coordinate,
            field_type,
            path_length,
        }
    }
}

impl PartialOrd<Self> for Field {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.path_length > other.path_length {
            return Some(Ordering::Less);
        }

        if self.path_length == other.path_length {
            return Some(Ordering::Equal);
        }

        return Some(Ordering::Greater);
    }
}

impl Ord for Field {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.path_length > other.path_length {
            return Ordering::Less;
        }

        if self.path_length == other.path_length {
            return Ordering::Equal;
        }

        return Ordering::Greater;
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use Dirs::*;

    use super::*;

    #[test]
    pub fn test_fields_connected() {
        assert_eq!(
            Grid::are_fields_connected(&FieldType::None, &FieldType::None, North),
            false
        );
        assert_eq!(
            Grid::are_fields_connected(&FieldType::Start, &FieldType::None, North),
            false
        );
        assert_eq!(
            Grid::are_fields_connected(&FieldType::None, &FieldType::Start, North),
            false,
            "None and Start to North"
        );
        assert_eq!(
            Grid::are_fields_connected(&FieldType::Start, &FieldType::Pipe(North, East), North),
            false,
            "Should not be connected if pipe is North, East and do the north"
        );
        assert_eq!(
            Grid::are_fields_connected(&FieldType::Start, &FieldType::Pipe(South, East), North),
            true
        );
        assert_eq!(
            Grid::are_fields_connected(&FieldType::Start, &FieldType::Pipe(West, East), North),
            false
        );
        assert_eq!(
            Grid::are_fields_connected(&FieldType::Start, &FieldType::Pipe(West, East), West),
            true
        );
        assert_eq!(
            Grid::are_fields_connected(&&FieldType::Pipe(North, East), &FieldType::Start, North),
            true
        );
        assert_eq!(
            Grid::are_fields_connected(&&FieldType::Pipe(South, West), &FieldType::Start, North),
            false
        );
        assert_eq!(
            Grid::are_fields_connected(
                &FieldType::Pipe(East, West),
                &FieldType::Pipe(North, South),
                North
            ),
            false
        );
        assert_eq!(
            Grid::are_fields_connected(
                &FieldType::Pipe(North, East),
                &FieldType::Pipe(North, South),
                North
            ),
            true
        );
        assert_eq!(
            Grid::are_fields_connected(
                &FieldType::Pipe(North, East),
                &FieldType::Pipe(North, South),
                South
            ),
            false
        );
        assert_eq!(
            Grid::are_fields_connected(
                &FieldType::Pipe(East, West),
                &FieldType::Pipe(East, West),
                North
            ),
            false
        );
        assert_eq!(
            Grid::are_fields_connected(
                &FieldType::Pipe(East, West),
                &FieldType::Pipe(East, West),
                East
            ),
            true
        );
        assert_eq!(
            Grid::are_fields_connected(
                &FieldType::Pipe(East, West),
                &FieldType::Pipe(East, West),
                West
            ),
            true
        );
        assert_eq!(
            Grid::are_fields_connected(
                &FieldType::Pipe(North, South),
                &FieldType::Pipe(North, South),
                North
            ),
            true
        );
        assert_eq!(
            Grid::are_fields_connected(
                &FieldType::Pipe(North, South),
                &FieldType::Pipe(North, South),
                South
            ),
            true
        );
        assert_eq!(
            Grid::are_fields_connected(
                &FieldType::Pipe(North, South),
                &FieldType::Pipe(North, South),
                East
            ),
            false
        );
        assert_eq!(
            Grid::are_fields_connected(
                &FieldType::Pipe(North, South),
                &FieldType::Pipe(North, South),
                West
            ),
            false
        );
    }
}
