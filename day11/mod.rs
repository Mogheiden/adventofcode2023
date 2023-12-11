use grid::Grid;
use std::collections::HashSet;

pub fn part1() {
    let grid =
        include_str!("./day11.txt")
            .lines()
            .fold(Grid::<char>::new(0, 0), |mut grid, line| {
                let row = line.chars().collect::<Vec<_>>();
                grid.push_row(row);
                grid
            });
    let mut visited_rows = HashSet::<usize>::new();
    let mut visited_cols = HashSet::<usize>::new();
    let mut galaxies = Vec::<(usize, usize)>::new();
    for (coord, &char) in grid.indexed_iter() {
        if char == '#' {
            visited_cols.insert(coord.1);
            visited_rows.insert(coord.0);
            galaxies.push(coord);
        }
    }
    let mut total = 0;
    for i in 0..(galaxies.len() - 1) {
        let start = galaxies[i];
        for j in (i + 1)..galaxies.len() {
            let mut doubles = 0;
            let dest = galaxies[j];
            if start.0 < dest.0 {
                for k in start.0..dest.0 {
                    if !(visited_rows.contains(&k)) {
                        doubles += 1;
                    }
                }
            } else {
                for k in dest.0..start.0 {
                    if !(visited_rows.contains(&k)) {
                        doubles += 1;
                    }
                }
            }

            if start.1 < dest.1 {
                for k in start.1..dest.1 {
                    if !(visited_cols.contains(&k)) {
                        doubles += 1;
                    }
                }
            } else {
                for k in dest.1..start.1 {
                    if !(visited_cols.contains(&k)) {
                        doubles += 1;
                    }
                }
            }
            // dbg!(doubles);
            total += doubles * 999999 + dest.1.abs_diff(start.1) + dest.0.abs_diff(start.0);
        }
        // dbg!(total);
    }
    dbg!(total);
    // dbg!(count);
}
