use std::cmp::Ordering;

use crate::Solution;

pub type CostType = Solution;
pub type MaskType = bool;

#[derive(Debug, Clone)]
pub struct Grid {
    pub fields: Vec<Vec<(CostType, MaskType)>>,
    size: (usize, usize),
}
impl Grid {
    pub fn new(size: (usize, usize)) -> Grid {
        Grid {
            fields: vec![vec![(0, false); size.1]; size.0],
            size,
        }
    }

    pub fn get_size(&self) -> (usize, usize) {
        self.size.clone()
    }

    pub(self) fn is_field_in_bounds(&self, y: i32, x: i32) -> bool {
        y >= 0 && x >= 0 && y < self.size.0 as i32 && x < self.size.1 as i32
    }

    pub fn get_field_cost(&self, y: usize, x: usize) -> CostType {
        self.fields[y][x].0
    }

    pub fn set_field_height(&mut self, y: usize, x: usize, cost: CostType) {
        if !self.is_field_in_bounds(y as i32, x as i32) {
            return;
        }

        self.fields[y][x].0 = cost;
    }

    pub fn mark_field(&mut self, y: usize, x: usize) {
        self.fields[y][x].1 = true;
    }

    pub fn is_field_marked(&self, y: usize, x: usize) -> bool {
        self.fields[y][x].1
    }

    pub fn get_left(&self, y: usize, x: usize) -> Option<CostType> {
        if !self.is_field_in_bounds(y as i32, x as i32 - 1) {
            return None;
        }

        Some(self.get_field_cost(y, x - 1))
    }

    pub fn get_right(&self, y: usize, x: usize) -> Option<CostType> {
        if !self.is_field_in_bounds(y as i32, x as i32 + 1) {
            return None;
        }

        Some(self.get_field_cost(y, x + 1))
    }

    pub fn get_top(&self, y: usize, x: usize) -> Option<CostType> {
        if !self.is_field_in_bounds(y as i32 - 1, x as i32) {
            return None;
        }

        Some(self.get_field_cost(y - 1, x))
    }

    pub fn get_bottom(&self, y: usize, x: usize) -> Option<CostType> {
        if !self.is_field_in_bounds(y as i32 + 1, x as i32) {
            return None;
        }

        Some(self.get_field_cost(y + 1, x))
    }

    pub fn get_unmarked_neighbours(
        &self,
        y: usize,
        x: usize,
        old_dir: (i32, i32),
        old_straights: i32,
        min_straights_before_turn: i32,
    ) -> Vec<(usize, usize, CostType, (i32, i32), i32)> {
        let mut neighbours = Vec::new();
        let (y_d, x_d) = old_dir;
        let (left_o, top_o, right_o, bottom_o) = (
            self.get_left(y, x),
            self.get_top(y, x),
            self.get_right(y, x),
            self.get_bottom(y, x),
        );
        if let Some(n) = left_o
            && !(y_d == 0 && x_d == 1)
        {
            let new_dir = (0, -1);
            let new_straights = if new_dir == old_dir {
                old_straights + 1
            } else {
                0
            };
            let turned = new_straights == 0;
            if old_dir == (0, 0)
                || !turned
                || (turned && old_straights >= min_straights_before_turn)
            {
                neighbours.push((y, x - 1, n, new_dir, new_straights));
            }
        };
        if let Some(n) = right_o
            && !(y_d == 0 && x_d == -1)
        {
            let new_dir = (0, 1);
            let new_straights = if new_dir == old_dir {
                old_straights + 1
            } else {
                0
            };
            let turned = new_straights == 0;
            if old_dir == (0, 0)
                || !turned
                || (turned && old_straights >= min_straights_before_turn)
            {
                neighbours.push((y, x + 1, n, new_dir, new_straights));
            }
        };
        if let Some(n) = top_o
            && !(y_d == 1 && x_d == 0)
        {
            let new_dir = (-1, 0);
            let new_straights = if new_dir == old_dir {
                old_straights + 1
            } else {
                0
            };

            let turned = new_straights == 0;
            if old_dir == (0, 0)
                || !turned
                || (turned && old_straights >= min_straights_before_turn)
            {
                neighbours.push((y - 1, x, n, new_dir, new_straights));
            }
        };
        if let Some(n) = bottom_o
            && !(y_d == -1 && x_d == 0)
        {
            let new_dir = (1, 0);
            let new_straights = if new_dir == old_dir {
                old_straights + 1
            } else {
                0
            };
            let turned = new_straights == 0;
            if old_dir == (0, 0)
                || !turned
                || (turned && old_straights >= min_straights_before_turn)
            {
                neighbours.push((y + 1, x, n, new_dir, new_straights));
            }
        };
        neighbours
    }

    pub fn print_grid(&self, path: &Vec<(usize, usize)>) {
        for (y, l) in self.fields.iter().enumerate() {
            let mut line = Vec::<String>::new();
            for (x, c) in l.iter().enumerate() {
                line.push(format!("{}", c.0));
                if path.contains(&(y, x)) {
                    line.push("#".into());
                } else {
                    line.push(" ".into());
                }
            }
            println!("{}", line.join(""));
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Field {
    pub coordinate: (usize, usize),
    pub total_heat_loss: u32,
    pub cost: u32,
    pub direction: (i32, i32),
    pub straights: i32,
}

impl Field {
    pub fn new(
        coordinate: (usize, usize),
        cost: u32,
        total_heat_loss: u32,
        direction: (i32, i32),
        straights: i32,
    ) -> Field {
        Field {
            coordinate,
            cost,
            total_heat_loss,
            direction,
            straights,
        }
    }
}

impl PartialOrd<Self> for Field {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.cost > other.cost {
            return Some(Ordering::Less);
        }

        if self.cost == other.cost {
            return Some(Ordering::Equal);
        }

        return Some(Ordering::Greater);
    }
}

impl Ord for Field {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.cost > other.cost {
            return Ordering::Less;
        }

        if self.cost == other.cost {
            return Ordering::Equal;
        }

        return Ordering::Greater;
    }
}
