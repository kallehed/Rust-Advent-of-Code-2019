
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Tile {
    Empty = 0,
    Wall = 1,
    Block = 2,
    HorPad = 3,
    Ball = 4
}

pub fn part_1()
{
    let mut codes: Vec<i64>;
    {

        let contents = std::fs::read_to_string("src/day13.txt")
            .expect("Should have been able to read the file");
        
        codes = contents.split(',').map(|num_str| num_str.parse().unwrap()).collect();

        codes.resize(codes.len() + 1000, 0);
    }

    // code for the care package game
    const SIZE: usize = 100;
    let mut screen = [[Tile::Empty; SIZE]; SIZE];

    let mut input = (0usize, 0usize, Tile::Empty);
    let mut instruction_at = 0u8;
    
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
                    _ => panic!("Wrong second mode: {}", code)
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
                        let inp = -1;
                        println!("Gave input: {}", inp);
                        codes[loc0] = inp;
                        i += 2;
                    }
                    4 => { // output value
                        let out = codes[loc0];
                        //print!("Output: {}", out);

                        if instruction_at == 0 {
                            input.0 = out as usize;
                            instruction_at = 1;
                        } else if instruction_at == 1 {
                            input.1 = out as usize;
                            instruction_at = 2;
                        } else {
                            assert!(out >= 0 && out <= 4);
                            input.2 = unsafe {std::mem::transmute(out as i8)};
                            screen[input.1][input.0] = input.2;
                            instruction_at = 0;
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
    // count how many block tiles there are
    let mut res = 0;
    for row in screen {
        for cell in row {
            if cell == Tile::Block {res += 1;}
        }
    }
    println!("Total block tiles: {}", res);
}

pub fn part_2()
{
    let mut codes: Vec<i64>;
    {

        let contents = std::fs::read_to_string("src/day13.txt")
            .expect("Should have been able to read the file");
        
        codes = contents.split(',').map(|num_str| num_str.parse().unwrap()).collect();

        codes.resize(codes.len() + 1000, 0);
    }
    codes[0] = 2; // play for free

    // code for the care package game
    const SIZE: usize = 40;
    let mut screen = [[Tile::Empty; SIZE]; 21];

    let mut input = (0isize, 0isize, Tile::Empty);
    let mut instruction_at = 0u8;

    let mut player_x = 19;
    let mut ball_x = 0isize;
    
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
                    _ => panic!("Wrong second mode: {}", code)
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
                        

                        // draw screen
                        /*for row in screen {
                            for pixel in row {
                                use Tile::*;
                                let c = match pixel {Empty => ' ', Wall => 'W', Block => 'B', HorPad => 'H', Ball => 'O' };
                                print!("{}", c)
                            }
                            println!();
                        }*/
                        let out;

                        if ball_x > player_x { // move under ball
                            out = 1;
                        } else if ball_x < player_x {
                            out = -1;
                        } else {
                            out = 0;
                        }
                        player_x += out;
                        
                        //r_i = 2;
                        //println!("Gave input: {}", out);
                        codes[loc0] = out as i64;
                        i += 2;
                    }
                    4 => { // output value
                        let out = codes[loc0];
                        //println!("Output: {}", out);

                        if instruction_at == 0 {
                            input.0 = out as isize;
                            instruction_at = 1;
                        } else if instruction_at == 1 {
                            input.1 = out as isize;
                            instruction_at = 2;
                        } else if input.0 == -1 && input.1 == 0 {
                            println!("SCORE: {}", out);
                            instruction_at = 0;
                        }
                        else {
                            assert!(out >= 0 && out <= 4);
                            input.2 = unsafe {std::mem::transmute(out as i8)};
                            screen[input.1 as usize][input.0 as usize] = input.2;
                            instruction_at = 0;
                            if input.2 == Tile::Ball {
                                ball_x = input.0;
                            }
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
}