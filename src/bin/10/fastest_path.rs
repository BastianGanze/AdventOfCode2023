use crate::grid::Dirs::{East, North, South, West};
use crate::grid::FieldType::Pipe;
use crate::grid::{CostType, Field, FieldType, Grid};
use crate::{ParseOutput, Solution};
use std::collections::BinaryHeap;

pub fn get_loop(out: &mut ParseOutput) -> Vec<Field> {
    let mut l = Vec::new();
    let (ref mut grid, start_coord) = out;
    let start = Field::new(*start_coord, FieldType::Start, 0);
    l.push(start);
    grid.mark_field(start_coord.0, start_coord.1);

    let neighbours = grid.get_connected_unvisited_neighbours(start_coord.0, start_coord.1);
    let next_field_pos = neighbours.get(0).unwrap();
    let mut current_field_option = Some(Field::new(
        *next_field_pos,
        grid.fields[next_field_pos.0][next_field_pos.1].1,
        1,
    ));
    grid.mark_field(next_field_pos.0, next_field_pos.1);

    l.push(current_field_option.unwrap());

    while let Some(current_field) = current_field_option {
        let neighbour = grid.get_connected_unvisited_neighbours(
            current_field.coordinate.0,
            current_field.coordinate.1,
        );
        if neighbour.is_empty() {
            current_field_option = None;
        } else {
            let f = Field::new(
                neighbour[0],
                grid.fields[neighbour[0].0][neighbour[0].1].1,
                current_field.path_length + 1,
            );
            grid.mark_field(neighbour[0].0, neighbour[0].1);
            l.push(f);
            current_field_option = Some(f);
        }
    }

    let next = &l[1];
    let prev = l.last().unwrap();
    l[0].field_type = Pipe(
        match (
            l[0].coordinate.0 as Solution - prev.coordinate.0 as Solution,
            l[0].coordinate.1 as Solution - prev.coordinate.1 as Solution,
        ) {
            (0, -1) => West,
            (0, 1) => East,
            (-1, 0) => South,
            (1, 0) => North,
            _ => panic!("No"),
        },
        match (
            next.coordinate.0 as Solution - l[0].coordinate.0 as Solution,
            next.coordinate.1 as Solution - l[0].coordinate.1 as Solution,
        ) {
            (0, -1) => West,
            (0, 1) => East,
            (-1, 0) => South,
            (1, 0) => North,
            _ => panic!("No"),
        },
    );

    l
}

pub fn count_unmarked_fields(grid: &mut Grid, start_coord: (usize, usize)) -> Solution {
    if grid.is_field_marked(start_coord.0, start_coord.1) {
        return 0;
    }
    let mut open_fields = BinaryHeap::<Field>::new();
    let start = Field::new(start_coord, FieldType::None, 0);
    let mut fields_marked = 0;

    grid.mark_field(start_coord.0, start_coord.1);
    fields_marked += 1;

    open_fields.push(start);

    while !open_fields.is_empty() {
        let current_field = open_fields.pop().unwrap();
        let neighbours =
            grid.get_unmarked_neighbours(current_field.coordinate.0, current_field.coordinate.1);
        for (y, x) in neighbours {
            grid.mark_field(y, x);
            open_fields.push(Field::new(
                (y, x),
                current_field.field_type,
                current_field.path_length + 1,
            ));
            fields_marked += 1;
        }
    }

    fields_marked
}

pub fn manhattan_distance(p1: (usize, usize), p2: (usize, usize)) -> Solution {
    (p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)) as Solution
}

fn calc_cost(field: &((usize, usize), CostType), neighbour: (usize, usize, CostType)) -> Solution {
    if (neighbour.2 - field.1) > 1 {
        return Solution::MAX;
    }

    (field.1 + 1) as Solution
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Turn {
    Left,
    Right,
    Straight,
}

pub fn turn_direction(p1: (Solution, Solution), p2: (Solution, Solution)) -> Turn {
    let cross_product = p1.0 * p2.1 - p1.1 * p2.0;

    if cross_product > 0 {
        Turn::Left
    } else if cross_product < 0 {
        Turn::Right
    } else {
        Turn::Straight
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turn() {
        assert_eq!(turn_direction((1, 0), (0, 1)), Turn::Left);
        assert_eq!(turn_direction((0, 1), (1, 0)), Turn::Right);
        assert_eq!(turn_direction((0, 1), (0, 1)), Turn::Straight);
    }
}
