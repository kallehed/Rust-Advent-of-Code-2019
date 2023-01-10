#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Tile {
    Unexplored,
    Free,
    Wall,
    OxygenSystem,
    BeenHere // already walked here (for finding the shortest path)
}

pub fn part_1()
{
    let mut codes: Vec<i64>;
    {
        let contents = std::fs::read_to_string("src/day15.txt")
            .expect("Should have been able to read the file");
        
        codes = contents.split(',').map(|num_str| num_str.parse().unwrap()).collect();

        codes.resize(codes.len() + 1000, 0);
    }
    // first let the robot walk around randomly for some time, scouting out the area completely
    let mut area = [[Tile::Unexplored; 41]; 41];
    let start_x: usize = area[0].len()/2 + 1;
    let start_y: usize = area.len()/2 + 1;
    let mut x = start_x;
    let mut y = start_y;
    let mut dx: isize = 0;
    let mut dy: isize = 0;
    
    let mut i: usize = 0;
    let mut relative_base = 0;
    for _ in 0..75000000 
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
                        use rand::Rng;
                        let inp = rand::thread_rng().gen_range(1..=4);

                        match inp {
                            1 => {dx = 0; dy = -1;}, // north
                            2 => {dx = 0; dy = 1;}, // south
                            3 => {dx = -1 ;dy = 0;}, // west
                            4 => {dx = 1; dy = 0;}, // east
                            _ => panic!("Invalid random number!")
                        }
                        //println!("Gave input: {}", inp);
                        codes[loc0] = inp;
                        i += 2;
                    }
                    4 => { // output value
                        let out = codes[loc0];
                        let n_x = x.checked_add_signed(dx).unwrap(); // new x and y
                        let n_y = y.checked_add_signed(dy).unwrap();
                        match out {
                            0 => area[n_y][n_x] = Tile::Wall, // reached wall
                            1 => {x = n_x; y = n_y; area[n_y][n_x] = Tile::Free}, // successful walk
                            2 => {x = n_x; y = n_y; area[n_y][n_x] = Tile::OxygenSystem}, // walked into oxygen system
                            a => panic!("Invalid output: {} from droid!", a)
                        }
                        //println!("Output: {}", out);
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
    // draw area
    for (ny, row)in area.iter().enumerate() {
        for (nx, place) in row.iter().enumerate() {
            use Tile::*;
            
            let c = if x == nx && y == ny {
                'D'
            } else {
                match place {Wall => '#', Free => '.', Unexplored => ' ', OxygenSystem => 'O', _ => 'B'}
            };
            print!("{}", c);
        }
        println!();
    }

    // find shortest path from start_x, start_y to where the oxygen system is
    let mut positions = Vec::new(); // breadth first search THAT WORKS
    positions.push((start_x, start_y));
    area[start_y][start_x] = Tile::BeenHere;
    'whole: for steps in 1..
    {
        for i in (0..positions.len()).rev()
        {
            let pos = positions[i];
            positions.remove(i);
            let moves = [(1,0),(-1,0),(0,1),(0,-1)];
            for a_move in moves {
                let n_pos = (pos.0.checked_add_signed(a_move.0).unwrap(), pos.1.checked_add_signed(a_move.1).unwrap());
                match area[n_pos.1][n_pos.0] {
                    Tile::Free => {area[n_pos.1][n_pos.0] = Tile::BeenHere; positions.push(n_pos);}
                    Tile::OxygenSystem => {println!("Took: {}", steps); break 'whole;}
                    _ => ()
                }
            }
        }
    }
    /* 
    // depth first search (also works)
    let mut steps_map = [[usize::MAX; 41]; 41];
    fn dive (area: &[[Tile; 41]; 41], steps_map: &mut [[usize; 41]; 41], pos: (usize, usize), steps: usize)
    {
        if area[pos.1][pos.0] == Tile::OxygenSystem {
            println!("Found! At steps: {}", steps);
        }
        let moves = [(1,0),(-1,0),(0,1),(0,-1)];
        for a_move in moves {
            let n_pos = (pos.0.checked_add_signed(a_move.0).unwrap(), pos.1.checked_add_signed(a_move.1).unwrap());
            if area[n_pos.1][n_pos.0] != Tile::Wall && steps_map[n_pos.1][n_pos.0] > steps {
                steps_map[n_pos.1][n_pos.0] = steps;
                dive(area, steps_map, n_pos, steps+1);
            }
        }
    }
    dive(&area, &mut steps_map, (start_x, start_y), 0);*/
}

pub fn part_2()
{
    let mut codes: Vec<i64>;
    {
        let contents = std::fs::read_to_string("src/day15.txt")
            .expect("Should have been able to read the file");
        
        codes = contents.split(',').map(|num_str| num_str.parse().unwrap()).collect();

        codes.resize(codes.len() + 1000, 0);
    }
    // first let the robot walk around randomly for some time, scouting out the area completely
    let mut area = [[Tile::Unexplored; 41]; 41];
    let start_x: usize = area[0].len()/2 + 1;
    let start_y: usize = area.len()/2 + 1;
    let mut x = start_x;
    let mut y = start_y;
    let mut dx: isize = 0;
    let mut dy: isize = 0;
    
    let mut i: usize = 0;
    let mut relative_base = 0;
    for _ in 0..75000000 
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
                        use rand::Rng;
                        let inp = rand::thread_rng().gen_range(1..=4);

                        match inp {
                            1 => {dx = 0; dy = -1;}, // north
                            2 => {dx = 0; dy = 1;}, // south
                            3 => {dx = -1 ;dy = 0;}, // west
                            4 => {dx = 1; dy = 0;}, // east
                            _ => panic!("Invalid random number!")
                        }
                        //println!("Gave input: {}", inp);
                        codes[loc0] = inp;
                        i += 2;
                    }
                    4 => { // output value
                        let out = codes[loc0];
                        let n_x = x.checked_add_signed(dx).unwrap(); // new x and y
                        let n_y = y.checked_add_signed(dy).unwrap();
                        match out {
                            0 => area[n_y][n_x] = Tile::Wall, // reached wall
                            1 => {x = n_x; y = n_y; area[n_y][n_x] = Tile::Free}, // successful walk
                            2 => {x = n_x; y = n_y; area[n_y][n_x] = Tile::OxygenSystem}, // walked into oxygen system
                            a => panic!("Invalid output: {} from droid!", a)
                        }
                        //println!("Output: {}", out);
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
    println!("asd{}", x);
    // draw area
    let mut oxy_x = 0;
    let mut oxy_y = 0;
    for (ny, row)in area.iter().enumerate() {
        for (nx, place) in row.iter().enumerate() {
            use Tile::*;
            
            let c = if x == nx && y == ny {
                'D'
            } else {
                match place {Wall => '#', Free => '.', Unexplored => ' ', OxygenSystem => {oxy_x = nx;oxy_y = ny;'O'}, _ => 'B'}
            };
            print!("{}", c);
        }
        println!();
    }

    // breadth first search that goes through all paths, until there are no paths left.
    let mut positions = Vec::new(); 
    positions.push((oxy_x, oxy_y));
    area[oxy_y][oxy_x] = Tile::BeenHere;
    for steps in 0..
    {
        for i in (0..positions.len()).rev()
        {
            let pos = positions[i];
            positions.remove(i);
            let moves = [(1,0),(-1,0),(0,1),(0,-1)];
            for a_move in moves {
                let n_pos = (pos.0.checked_add_signed(a_move.0).unwrap(), pos.1.checked_add_signed(a_move.1).unwrap());
                if area[n_pos.1][n_pos.0] == Tile::Free {
                    area[n_pos.1][n_pos.0] = Tile::BeenHere;
                    positions.push(n_pos);
                }
            }
        }
        if positions.is_empty() {println!("took {}", steps);break;}
    }
}