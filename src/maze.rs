use indoc::indoc;

pub struct MonstrousMazeInput {
    pub grid: String,
    pub endurance: u8,
}

pub struct MonstrousMazeOutput {
    pub path: String,
}

pub struct Coordinate {
    pub start_x: usize,
    pub start_y: usize,
    pub end_x: usize,
    pub end_y: usize,
}

fn find_starting_points(grid: &String) -> Coordinate {
    let mut start_x = 0;
    let mut start_y = 0;
    let mut coordinate = Coordinate {
        start_x,
        start_y,
        end_x: 0,
        end_y: 0,
    };

    let array = grid.split("\n");

    for row in array {
        for column in row.chars() {
            if column == 'Y' {
                coordinate.start_y = start_y;
                coordinate.start_x = start_x;
            }
            start_x += 1;
        }
        start_y += 1;
        start_x = 0;
    }

    return coordinate;
}

fn find_ending_points(grid: &String) -> Coordinate {
    let mut end_x = 0;
    let mut end_y = 0;
    let mut coordinate = Coordinate {
        start_x: 0,
        start_y: 0,
        end_x,
        end_y,
    };

    let array = grid.split("\n");

    for row in array {
        for column in row.chars() {
            if column == 'X' {
                coordinate.end_y = end_y;
                coordinate.end_x = end_x;
            }
            end_x += 1;
        }
        end_y += 1;
        end_x = 0;
    }

    return coordinate;
}


fn find_exit(maze: &MonstrousMazeInput, mut coordinate: &mut Coordinate, mut direction: i32) -> bool {
    let grid = maze.grid.split("\n");
    let mut rows: Vec<Vec<char>> = Vec::new();
    println!("coord end");
    print!("{} ", coordinate.end_y);
    println!("{}", coordinate.end_x);

    println!("coord start");
    print!("{} ", coordinate.start_y);
    println!("{}", coordinate.start_x);

    grid.for_each(|x| rows.push(x.chars().collect()));
    let mut ok: bool = false;

    //print!("{} ", coordinate.start_y);
    //println!("{}", coordinate.start_x);

    for i in 0..4{
        if !ok {
            if i != direction as u8 {
                // println!("{}", i);
                match i {
                    0 => {
                        let tmp: isize = coordinate.start_y as isize - 1;
                        if tmp > 0 {
                            if rows.get(tmp as usize).unwrap().get(coordinate.start_x).unwrap() == &' ' ||
                                rows.get(tmp as usize).unwrap().get(coordinate.start_x).unwrap() == &'X'
                            {
                                println!("^");
                                coordinate.start_y -= 1;
                                if coordinate.start_x == coordinate.end_x && coordinate.start_y == coordinate.end_y {
                                    ok = true;
                                }else {
                                    ok = find_exit(&maze, &mut coordinate, 2);
                                }

                            }
                        }
                        //i += 1;
                        // break;
                    }
                    1 => {
                        let tmp: isize = coordinate.start_x as isize + 1;
                        if tmp < rows.get(coordinate.start_y).unwrap().len() as isize {
                            if rows.get(coordinate.start_y).unwrap().get(coordinate.start_x + 1).unwrap() == &' ' ||
                                rows.get(coordinate.start_y).unwrap().get(coordinate.start_x + 1).unwrap() == &'X'
                            {
                                println!(">");
                                coordinate.start_x += 1;
                                if coordinate.start_x == coordinate.end_x && coordinate.start_y == coordinate.end_y {
                                    ok = true;
                                }else {
                                    ok = find_exit(&maze, &mut coordinate, 3);
                                }
                            }
                        }
                        //i += 1;
                        // break;
                    }
                    2 => {
                        let tmp: isize = coordinate.start_y as isize + 1;
                        if tmp < rows.len() as isize {
                            if rows.get(coordinate.start_y + 1).unwrap().get(coordinate.start_x).unwrap() == &' ' ||
                                rows.get(coordinate.start_y + 1).unwrap().get(coordinate.start_x).unwrap() == &'X'
                            {
                                println!("v");
                                coordinate.start_y += 1;
                                if coordinate.start_x == coordinate.end_x && coordinate.start_y == coordinate.end_y {
                                    ok = true;
                                }else {
                                    ok = find_exit(&maze, &mut coordinate, 0);
                                }
                            }
                        }
                       // i += 1;
                        // break;
                    }
                    3 => {
                        let tmp: isize = coordinate.start_x as isize - 1;
                        if tmp > 0 {
                            if rows.get(coordinate.start_y).unwrap().get(coordinate.start_x - 1).unwrap() == &' '  ||
                                rows.get(coordinate.start_y).unwrap().get(coordinate.start_x - 1).unwrap() == &'X'
                                {
                                println!("<");
                                coordinate.start_x -= 1;
                                    if coordinate.start_x == coordinate.end_x && coordinate.start_y == coordinate.end_y {
                                        ok = true;
                                    }else {
                                        ok = find_exit(&maze, &mut coordinate, 1);
                                    }
                            }
                        }
                        //i += 1;
                        // break;
                    }
                    _ => {
                        //i += 1;
                        println!("yo");
                        find_exit(&maze, &mut coordinate, -1);
                    }
                }
            }
            // println!("i:{}",i);
        }
    }

    if coordinate.start_x == coordinate.end_x && coordinate.start_y == coordinate.end_y {
        ok = true;
    }

    return ok;
}

// return path;

fn main() {
    let tmp_maze = indoc! {"┌─────┬─────────┬─┐
                            │     │         │ │
                            ├── ──┴─┬── ────┘ │
                            │                 │
                            │ ┌─┬───┴─┬── │ ┌─┤
                            │ │Y│     │   │ │ │
                            │ │ │ ──┐ ├───┤ └─┤
                            │   │   │ │   │   │
                            │ ──┘ │ │ │ ──┴─┐ │
                            │     │ │       │ │
                            └─────┴─┴───────┴X┘"};

    let grid: String = tmp_maze.to_string();
    let endurance = 1 as u8;

    let maze = MonstrousMazeInput {
        grid,
        endurance,
    };
    let mut path = MonstrousMazeOutput {
        path: " ".to_string(),
    };

    let mut start_coordinate: Coordinate = find_starting_points(&maze.grid);
    let mut end_coordinate = find_ending_points(&maze.grid);
    let mut coordinate: Coordinate = Coordinate {
        start_x: start_coordinate.start_x,
        start_y: start_coordinate.start_y,
        end_x: end_coordinate.end_x,
        end_y: end_coordinate.end_y,
    };
    let tmp = find_exit(&maze, &mut coordinate, -1);
    if tmp {
        println!("finish")
    }else {
        println!("nop")
    }
    // println!("{}", tmp.path);
}
