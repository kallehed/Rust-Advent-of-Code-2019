

pub fn part_1() {

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    enum Tile {
        Empty = 0,
        Bug
    }
    const N: usize = 5;

    let mut space = [[Tile::Empty; N]; N];
    {
        let cont = include_str!("day24.txt");
        println!("{}", cont);

        for (i, l) in cont.lines().enumerate() {
            for (j, c) in l.chars().enumerate() {
                space[i][j] = match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Bug,
                    _ => unreachable!(),
                }
            }
        }
        println!("{:?}", space);
    }

    let mut set = std::collections::HashSet::new();

    loop {
        //println!("{:?}", space);

        let print = || {
            for a in space {
                for b in a {
                    print!("{}", match b {
                        Tile::Empty => '.',
                        Tile::Bug => '#'
                    });
                }
                println!();
            }
            println!();
        }; 
        //print();

        // put data into i32, 5*5 = 25 bits. 
        {
            let mut dat: i32 = 0;
            let mut wher = 0;
            for a in space {
                for b in a {
                    if b == Tile::Bug {
                        dat |= 1 << wher;
                    }
                    wher += 1;
                }
            }
            //println!("{:b}", dat);
            if !set.insert(dat) {
                println!("{:25b} appears twice!", dat);

                // find biodiversity rating
                let mut total = 0;
                let mut rating = 1_usize;
                for n in 0..(N*N)
                {
                    if (dat & (1_i32 << n)) != 0i32 {
                        println!("{}, {}", n, rating);
                        total += rating;
                    }

                    rating *= 2;
                }
                println!("rating: {}", total);
                break;
            }
        }


        let copy = space;

        let safe = |i, j| i >= 0 && j >= 0 && i < (N as _) && j < (N as _);
        // from copy, modify space
        for i in 0..N {
            for j in 0..N {
                let mut adjacent = 0;
                let others = [(1,0),(-1,0),(0,1),(0,-1)];
                for other in others {
                    let n_i = (i as i8) + other.0;
                    let n_j = (j as i8) + other.1;
                    if safe(n_i, n_j)
                    {
                        adjacent += (copy[n_i as usize][n_j as usize] == Tile::Bug) as i8;
                    }
                }
                match copy[i][j] {
                    Tile::Empty => {
                        if adjacent == 1 || adjacent == 2 {
                            space[i][j] = Tile::Bug;
                        }
                    },
                    Tile::Bug => {
                        if adjacent != 1 {
                            space[i][j] = Tile::Empty;
                        }
                    },
                }
            }
        }
    }
}

pub fn part_2() {

    // have an array of 2D spaces, where depth 0 is in the middle
    const LAYERS: usize = 401;
    const DEPTH_0: usize = LAYERS/2;

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    enum Tile {
        Empty = 0,
        Bug
    }
    const N: usize = 5;

    let mut spaces = [[[Tile::Empty; N]; N]; LAYERS];
    {
        let cont = include_str!("day24.txt");
        println!("{}", cont);

        for (i, l) in cont.lines().enumerate() {
            for (j, c) in l.chars().enumerate() {
                spaces[DEPTH_0][i][j] = match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Bug,
                    _ => unreachable!(),
                }
            }
        }
        println!("{:?}", spaces[DEPTH_0]);
    }

    for _ in 0..200 { // 200 minutes
        let copies = spaces;

        // from copy, modify space
        for space in 1..(LAYERS-1) { // always have layers around. if wrong answer -> increase layers
            for i in 0..N {
                for j in 0..N {
                    if i == 2 && j == 2 {continue;} // don't change the middle one
                    let mut adjacent = 0;
                    let others = [(1,0),(-1,0),(0,1),(0,-1)];
                    for (other_idx, other) in others.iter().enumerate() {
                        let n_i = (i as i8) + other.0;
                        let n_j = (j as i8) + other.1;
                        if n_i == 2 && n_j == 2 { // middle
                            let n_space = space + 1;
                            match other_idx {
                                0 => { // over middle, add A,B,C,D,E
                                    for j_t in 0..N {
                                        adjacent += (copies[n_space][0][j_t] == Tile::Bug) as i8;
                                    }
                                }
                                1 => { // under middle, add U,V,W,X,Y
                                    for j_t in 0..N {
                                        adjacent += (copies[n_space][4][j_t] == Tile::Bug) as i8;
                                    }
                                }
                                2 => { // we are left to middle, add A F K P U
                                    for i_t in 0..N {
                                        adjacent += (copies[n_space][i_t][0] == Tile::Bug) as i8;
                                    }
                                }
                                3 => { // we are to the right of middle, add E,J,O,T,Y   
                                    for i_t in 0..N {
                                        adjacent += (copies[n_space][i_t][4] == Tile::Bug) as i8;
                                    }
                                }
                                _ => unreachable!()
                            }
                        }
                        else if n_i == -1 { // we have arrived at outer UP from this level
                            adjacent += (copies[space - 1][1][2]) as i8;
                        } else if n_i == 5 { // we are BELOW at the outer level
                            adjacent += (copies[space - 1][3][2]) as i8;
                        } else if n_j == -1 { // we are to the LEFT, at outer level
                            adjacent += (copies[space - 1][2][1]) as i8;
                        } else if n_j == 5 {
                            adjacent += (copies[space - 1][2][3]) as i8;
                        } else {
                            // in range and not in middle?
                            adjacent += (copies[space][n_i as usize][n_j as usize] == Tile::Bug) as i8;
                        }
                    }
                    match copies[space][i][j] {
                        Tile::Empty => {
                            if adjacent == 1 || adjacent == 2 {
                                spaces[space][i][j] = Tile::Bug;
                            }
                        },
                        Tile::Bug => {
                            if adjacent != 1 {
                                spaces[space][i][j] = Tile::Empty;
                            }
                        },
                    }
                }
            }
        }
        
    }

    // count total amount of bugs
    let mut bugs = 0;
    for a in spaces {
        for b in a {
            for c in b {
                bugs += (c == Tile::Bug) as i32;
            }
        }
    }
    println!("total bugs: {}", bugs);
}