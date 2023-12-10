use grid::Grid;
use std::collections::HashMap;

fn find_start(grid: Grid<char>) -> (usize, usize) {
    for (coord, &char) in grid.indexed_iter() {
        if char == 'S' {
            return coord;
        }
    }
    panic!();
}

fn next_dir(direction: char, pipe: char) -> char {
    if pipe == '-' {
        if direction == 'w' {
            return 'w';
        } else if direction == 'e' {
            return 'e';
        } else {
            panic!();
        }
    } else if pipe == '|' {
        if direction == 'n' {
            return 'n';
        } else if direction == 's' {
            return 's';
        } else {
            panic!();
        }
    } else if pipe == 'F' {
        if direction == 'n' {
            return 'e';
        } else if direction == 'w' {
            return 's';
        } else {
            panic!();
        }
    } else if pipe == 'L' {
        if direction == 'w' {
            return 'n';
        } else if direction == 's' {
            return 'e';
        } else {
            panic!();
        }
    } else if pipe == 'J' {
        if direction == 's' {
            return 'w';
        } else if direction == 'e' {
            return 'n';
        } else {
            panic!();
        }
    } else if pipe == '7' {
        if direction == 'e' {
            return 's';
        } else if direction == 'n' {
            return 'w';
        } else {
            panic!();
        }
    } else {
        return 'x';
    }
}

fn is_inside(coord: (usize, usize), pipes: &HashMap<(usize, usize), char>) -> i32 {
    if pipes.contains_key(&coord) {
        return 0;
    }
    let mut num_crossings = 0;
    for i in 0..coord.1 {
        if pipes.contains_key(&(coord.0, i)) {
            let pipe = pipes[&(coord.0, i)];
            if pipe == 'J' || pipe == 'L' || pipe == '|' || pipe == 'S' {
                num_crossings += 1;
            }
        }
    }
    // if num_crossings % 2 == 1 {
    //     dbg!(coord);
    // }
    return num_crossings % 2;
}

pub fn part1() {
    let grid =
        include_str!("./day10.txt")
            .lines()
            .fold(Grid::<char>::new(0, 0), |mut grid, line| {
                let row = line.chars().collect::<Vec<_>>();
                grid.push_row(row);
                grid
            });

    let mut coord = find_start(grid.clone());
    // dbg!(coord);
    let mut loop_len = 0;
    let mut direction = 'n';

    while direction != 'x' {
        let pipe;
        if direction == 'n' {
            pipe = grid[(coord.0 - 1, coord.1)];
            coord = (coord.0 - 1, coord.1);
        } else if direction == 's' {
            pipe = grid[(coord.0 + 1, coord.1)];
            coord = (coord.0 + 1, coord.1);
        } else if direction == 'e' {
            pipe = grid[(coord.0, coord.1 + 1)];
            coord = (coord.0, coord.1 + 1);
        } else if direction == 'w' {
            pipe = grid[(coord.0, coord.1 - 1)];
            coord = (coord.0, coord.1 - 1);
        } else {
            panic!()
        }
        // dbg!(direction, coord, pipe);
        direction = next_dir(direction, pipe);

        loop_len += 1;
    }
    dbg!(loop_len / 2);
}

pub fn part2() {
    let grid =
        include_str!("./day10.txt")
            .lines()
            .fold(Grid::<char>::new(0, 0), |mut grid, line| {
                let row = line.chars().collect::<Vec<_>>();
                grid.push_row(row);
                grid
            });

    let mut coord = find_start(grid.clone());
    let mut all_pipes = HashMap::new();
    let mut direction = 's';

    all_pipes.insert(coord, 'S');

    while direction != 'x' {
        let pipe;
        if direction == 'n' {
            pipe = grid[(coord.0 - 1, coord.1)];
            coord = (coord.0 - 1, coord.1);
        } else if direction == 's' {
            pipe = grid[(coord.0 + 1, coord.1)];
            coord = (coord.0 + 1, coord.1);
        } else if direction == 'e' {
            pipe = grid[(coord.0, coord.1 + 1)];
            coord = (coord.0, coord.1 + 1);
        } else if direction == 'w' {
            pipe = grid[(coord.0, coord.1 - 1)];
            coord = (coord.0, coord.1 - 1);
        } else {
            panic!()
        }
        // dbg!(direction, coord, pipe);
        direction = next_dir(direction, pipe);
        all_pipes.insert(coord, pipe);
    }
    let mut inside = 0;
    // let height = grid.size().0;
    for (coord, _) in grid.indexed_iter() {
        inside += is_inside(coord, &all_pipes);
    }
    dbg!(inside);
}
