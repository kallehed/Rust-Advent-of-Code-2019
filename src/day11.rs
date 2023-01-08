use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Color {
    Black = 0,
    White = 1
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Dir {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3
}



pub fn part_1()
{
    let mut codes: Vec<i64>;
    {

        let contents = std::fs::read_to_string("src/day11.txt")
            .expect("Should have been able to read the file");
        
        codes = contents.split(',').map(|num_str| num_str.parse().unwrap()).collect();

        codes.resize(codes.len() + 1000, 0);
    }
    //println!("codes: {:?}", codes);

    // stuff relevant to paint-robot
    const SIZE: usize = 1001; // width and height of matrix for painting
    let mut canvas = [[Color::Black; SIZE]; SIZE];
    let (mut robot_x, mut robot_y) = (SIZE/2,SIZE/2);
    let mut positions_painted = HashSet::<(usize, usize)>::new(); // contains positions that have been painted
    let mut robot_output_index = 0;
    let mut robot_dir = Dir::Up;
    
    let mut i: usize = 0;
    let mut relative_base = 0;
    loop 
    {
        let code = codes[i];
        let op_code = code % 100;

        match op_code {
            op if ((1..=9).contains(&op)) =>
            {
                let first_mode = code % 1000 - op_code;
                let sec_mode = code % 10000 - (first_mode + op_code);
                let tri_mode = code % 100000 - (first_mode + op_code + sec_mode);

                // locations of values
                let mut loc0 = i + 1;
                let mut loc1 = i + 2;
                let mut loc2 = i + 3;

                match first_mode {
                    0 => loc0 = codes[loc0] as usize,
                    100 => (),
                    200 => loc0 = (codes[loc0] + relative_base) as usize,
                    _ => panic!("Wrong first mode: {}", code)
                }
                match sec_mode {
                    0 => loc1 = codes[loc1] as usize,
                    1000 => (),
                    2000 => loc1 = (codes[loc1] + relative_base) as usize,
                    _ => panic!("Wrong second mode: {}", code)
                }

                match tri_mode {
                    0 => loc2 = codes[loc2] as usize,
                    10000 => (),
                    20000 => loc2 = (codes[loc2] + relative_base) as usize,
                    _ => panic!("Wrong third mode: {}", code)
                }

                let val0 = codes[loc0];
                let mut val1 = 0;
                if loc1 < codes.len() {
                    val1 = codes[loc1];
                }
                
                match op {
                    1 => { // add
                        codes[loc2] = val0 + val1;
                        i += 4;
                    }
                    2 => { // multiply
                        codes[loc2] = val0 * val1;
                        i += 4;
                    }
                    3 => { // take input 1 and store it at next
                        let inp = canvas[robot_y][robot_x] as i64;
                        //println!("Gave input: {}", inp);
                        codes[loc0] = inp;
                        i += 2;
                    }
                    4 => { // output value
                        let out = codes[loc0];
                        //println!("Output: {}", out);

                        if robot_output_index == 0 {
                            // paint this panel to out
                            canvas[robot_y][robot_x] = match out {0 => Color::Black, 1 => Color::White, _ => panic!("Illegal output by robot!")};
                            positions_painted.insert((robot_x,robot_y));
                            robot_output_index = 1; 
                        } else { // 0 => left, 1 => right
                            let mut val = robot_dir as i64;
                            match out {
                                0 => val -= 1,
                                1 => val += 1,
                                _ => panic!("Illegal turn from output!")
                            }
                            val = val.rem_euclid(4);
                            robot_dir = unsafe { core::mem::transmute(val as u8) };

                            match robot_dir {
                                Dir::Up => robot_y -= 1,
                                Dir::Right => robot_x += 1,
                                Dir::Down => robot_y += 1,
                                Dir::Left => robot_x -= 1
                            }

                            robot_output_index = 0;
                        }

                        i += 2;
                    }
                    5 => { // jump if true
                        if val0 != 0 {
                            i = val1 as usize;
                        } else {
                            i += 3;
                        }
                    }
                    6 => { // jump if false
                        if val0 == 0 {
                            i = val1 as usize;
                        } else {
                            i += 3;
                        }
                    }
                    7 => { // less than
                        codes[loc2] = (val0 < val1) as i64;
                        i += 4;
                    }
                    8 => { // equal
                        codes[loc2] = (val0 == val1) as i64;
                        i += 4;
                    }
                    9 => { // increase relative base by first argument
                        relative_base += val0;
                        i += 2;
                    }
                    _ => {
                        panic!("very invalid: {}", code);
                    }
                }
            }
            
            99 => {
                break;
            }
            _ => {
                panic!("Wrong instruction: {}", code);
            }
        }
    }
    println!("drawn: {}", positions_painted.len());
}

pub fn part_2()
{
    let mut codes: Vec<i64>;
    {

        let contents = std::fs::read_to_string("src/day11.txt")
            .expect("Should have been able to read the file");
        
        codes = contents.split(',').map(|num_str| num_str.parse().unwrap()).collect();

        codes.resize(codes.len() + 1000, 0);
    }
    //println!("codes: {:?}", codes);

    // stuff relevant to paint-robot
    //const SIZE: usize = 121; // width and height of matrix for painting
    const WIDTH: usize = 91;
    const HEIGHT: usize = 21;
    let mut canvas = [[Color::Black; WIDTH]; HEIGHT];
    let (mut robot_x, mut robot_y) = (WIDTH/2,HEIGHT/2);
    let mut positions_painted = HashSet::<(usize, usize)>::new(); // contains positions that have been painted
    let mut robot_output_index = 0;
    let mut robot_dir = Dir::Up;
    
    canvas[robot_y][robot_x] = Color::White;

    let mut i: usize = 0;
    let mut relative_base = 0;
    loop 
    {
        let code = codes[i];
        let op_code = code % 100;

        match op_code {
            op if ((1..=9).contains(&op)) =>
            {
                let first_mode = code % 1000 - op_code;
                let sec_mode = code % 10000 - (first_mode + op_code);
                let tri_mode = code % 100000 - (first_mode + op_code + sec_mode);

                // locations of values
                let mut loc0 = i + 1;
                let mut loc1 = i + 2;
                let mut loc2 = i + 3;

                match first_mode {
                    0 => loc0 = codes[loc0] as usize,
                    100 => (),
                    200 => loc0 = (codes[loc0] + relative_base) as usize,
                    _ => panic!("Wrong first mode: {}", code)
                }
                match sec_mode {
                    0 => loc1 = codes[loc1] as usize,
                    1000 => (),
                    2000 => loc1 = (codes[loc1] + relative_base) as usize,
                    _ => panic!("Wrong second mode: {}", code)
                }

                match tri_mode {
                    0 => loc2 = codes[loc2] as usize,
                    10000 => (),
                    20000 => loc2 = (codes[loc2] + relative_base) as usize,
                    _ => panic!("Wrong third mode: {}", code)
                }

                let val0 = codes[loc0];
                let mut val1 = 0;
                if loc1 < codes.len() {
                    val1 = codes[loc1];
                }
                
                match op {
                    1 => { // add
                        codes[loc2] = val0 + val1;
                        i += 4;
                    }
                    2 => { // multiply
                        codes[loc2] = val0 * val1;
                        i += 4;
                    }
                    3 => { // take input 1 and store it at next
                        let inp = canvas[robot_y][robot_x] as i64;
                        //println!("Gave input: {}", inp);
                        codes[loc0] = inp;
                        i += 2;
                    }
                    4 => { // output value
                        let out = codes[loc0];
                        //println!("Output: {}", out);

                        if robot_output_index == 0 {
                            // paint this panel to out
                            canvas[robot_y][robot_x] = match out {0 => Color::Black, 1 => Color::White, _ => panic!("Illegal output by robot!")};
                            positions_painted.insert((robot_x,robot_y));
                            robot_output_index = 1; 
                        } else { // 0 => left, 1 => right
                            let mut val = robot_dir as i64;
                            match out {
                                0 => val -= 1,
                                1 => val += 1,
                                _ => panic!("Illegal turn from output!")
                            }
                            val = val.rem_euclid(4);
                            robot_dir = unsafe { core::mem::transmute(val as u8) };

                            match robot_dir {
                                Dir::Up => robot_y -= 1,
                                Dir::Right => robot_x += 1,
                                Dir::Down => robot_y += 1,
                                Dir::Left => robot_x -= 1
                            }

                            robot_output_index = 0;
                        }

                        i += 2;
                    }
                    5 => { // jump if true
                        if val0 != 0 {
                            i = val1 as usize;
                        } else {
                            i += 3;
                        }
                    }
                    6 => { // jump if false
                        if val0 == 0 {
                            i = val1 as usize;
                        } else {
                            i += 3;
                        }
                    }
                    7 => { // less than
                        codes[loc2] = (val0 < val1) as i64;
                        i += 4;
                    }
                    8 => { // equal
                        codes[loc2] = (val0 == val1) as i64;
                        i += 4;
                    }
                    9 => { // increase relative base by first argument
                        relative_base += val0;
                        i += 2;
                    }
                    _ => {
                        panic!("very invalid: {}", code);
                    }
                }
            }
            
            99 => {
                break;
            }
            _ => {
                panic!("Wrong instruction: {}", code);
            }
        }
    }
    
    for row in &canvas[..] {
        for pixel in &row[..] {
            print!("{}", (match pixel {Color::Black => ' ', Color::White => 'X'}));
        }
        println!("");
    }
}