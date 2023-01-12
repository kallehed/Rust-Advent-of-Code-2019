pub fn part_1()
{
    let mut codes: Vec<i64>;
    {

        let contents = std::fs::read_to_string("src/day17.txt")
            .expect("Should have been able to read the file");
        
        codes = contents.split(',').map(|num_str| num_str.parse().unwrap()).collect();

        codes.resize(codes.len() + 2000, 0);
    }
    //println!("codes: {:?}", codes);
    #[derive(Copy, Clone, Eq, PartialEq)]
    enum Tile {
        Space,
        Scaffold
    }

    let mut view = [[Tile::Space; 37]; 33];
    {
        let mut x = 0;
        let mut y = 0;
        
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
                            let out = codes[loc0] as u8;
                            match out {
                                b'.' => x += 1,
                                b'#' | b'>' | b'<' | b'^' | b'v' => {view[y][x] = Tile::Scaffold;x += 1; }
                                b'\n' => {y += 1;x = 0;}
                                _ => panic!("Illegal char: {}", out)
                            }
                            let c = char::from(out);
                            print!("{}", c);
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

    {
        let mut total = 0;
        for y in 1..(view.len()-1) {
            for x in 1..(view[0].len()-1) {
                use Tile::Scaffold as S;
                if view[y][x]   == S &&
                   view[y-1][x] == S &&
                   view[y+1][x] == S &&
                   view[y][x-1] == S && 
                   view[y][x+1] == S
                {
                    let alignement_parameter = y * x;
                    total += alignement_parameter;
                }
            }
        }
        println!("Total: {}", total);
    }
    
}

pub fn part_2() // for this one I drew out the map on paper and figures out the patterns, which was easier than I thought!
{
    let mut codes: Vec<i64>;
    {

        let contents = std::fs::read_to_string("src/day17.txt")
            .expect("Should have been able to read the file");
        
        codes = contents.split(',').map(|num_str| num_str.parse().unwrap()).collect();

        codes.resize(codes.len() + 2000, 0);
    }
    codes[0] = 2;
    //println!("codes: {:?}", codes);
    #[derive(Copy, Clone, Eq, PartialEq)]
    enum Tile {
        Space,
        Scaffold
    }

    let mut view = [[Tile::Space; 37]; 33];

    let mut instruction_at = 0;
    let instructions = b"A,A,B,C,B,C,B,C,C,A\n\
                                    R,8,L,4,R,4,R,10,R,8\n\
                                    L,12,L,12,R,8,R,8\n\
                                    R,10,R,4,R,4\n\
                                    n\n";

    let mut last = 0;
    {
        let mut x = 0;
        let mut y = 0;
        let mut writing_mode = true;
        
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
                            writing_mode = false;
                            let inp = instructions[instruction_at];
                            instruction_at += 1;
                            println!("Gave input: {}", inp);
                            codes[loc0] = inp as _;
                            i += 2;
                        }
                        4 => { // output value
                            last = codes[loc0];
                            let out = last as u8;
                            let c = char::from(out);
                            print!("{}", c);
                            if writing_mode {
                                match out {
                                    b'.' => x += 1,
                                    b'#' | b'>' | b'<' | b'^' | b'v' => {view[y][x] = Tile::Scaffold;x += 1;}
                                    b'\n' => {y += 1;x = 0;}
                                    _ => ()
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
    println!("Answer: {}", last);
}