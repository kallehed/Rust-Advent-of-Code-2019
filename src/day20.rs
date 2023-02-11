use std::collections::HashMap;

#[allow(clippy::needless_range_loop)]

pub fn part_1()
{
    type Pos = (usize, usize);

    #[derive(Debug, Copy, Clone)]
    enum Place {
        /// passage in donut
        Free, 
        Wall,
        /// space outside donut
        Outside, 
        Portal(Pos)
    }

    {
        let content = include_str!("day20.txt");

        println!("{}", content);

        // first get all names and their positions

        // create 2d vector of all characters
        let mut data = Vec::<Vec<char>>::new();
        data.push(Vec::new());
        for c in content.chars() {
            if c == '\n' { data.push(Vec::new()) }
            if c != '\n' && c != '\r' {
                data.last_mut().unwrap().push(c);
            }
        }
        data.push(Vec::new());
        data.last_mut().unwrap().append(&mut vec![' '; 200]);
        data.iter_mut().for_each(|k| k.push(' '));
        println!("{:?}", data);

        // iterate through 2D vector, spotting names of portals

        let mut map = HashMap::new();

        let mut portals = Vec::new();

        let mut add_to_map = |name: (char,char), pos: Pos| 
        {
            map.entry(name).and_modify(|k|
            {  
                portals.push((*k, pos));
            }).or_insert(pos);
        };

        for i in 0..data.len() {
            for j in 0..data[i].len() {
                let item = data[i][j];
                if item.is_alphabetic() {
                    if data[i + 1][j].is_alphabetic() {
                        // we have got a vertical portal
                        if data[i + 2][j] == '.' {
                            // portal is below
                            add_to_map((item, data[i + 1][j]), (i + 2, j));
                        } else {
                            // portal is above
                            add_to_map( (item, data[i + 1][j]), (i - 1, j));
                        }
                    } else if data[i][j + 1].is_alphabetic() {
                        // horizontal
                        if data[i][j + 2] == '.' {
                            // right
                            add_to_map((item, data[i][j + 1]), (i, j + 2), );
                        } else {
                            // left
                            add_to_map((item, data[i][j + 1]), (i, j - 1));
                        }
                    }
                }
            }
        }
        println!("{:?}", map);
        println!("{:?}", portals);

        let start = *map.get(&('A','A')).unwrap();
        let end = *map.get(&('Z','Z')).unwrap();

        let mut cells = Vec::new();
        

        for i in 0..data.len() {
            cells.push(Vec::new());
            for j in 0..data[i].len() {
                let item = data[i][j];
                let t = cells.last_mut().unwrap();
                match item {
                    '#' => t.push(Place::Wall),
                    '.' => t.push(Place::Free),
                    _   => t.push(Place::Outside),
                }
            }
        }
        

        // add portals //and end

        for p in portals {
            cells[p.0.0][p.0.1] = Place::Portal(p.1);
            cells[p.1.0][p.1.1] = Place::Portal(p.0);
        }
        //cells[end.0][end.1] = Place::End; // should not be important to depth first search
        
        println!("{:?}", cells);

        // do a depth first search with a matrix for how long it took to get to everywhere

        let mut marked = Vec::new();
        for i in 0..cells.len() {
            marked.push(Vec::new());
            for j in 0..cells[i].len() {
                marked.last_mut().unwrap().push(usize::MAX);
            }
        }

        fn recurse(pos: Pos, cells: &Vec<Vec<Place>>, steps: usize, marked: &mut Vec<Vec<usize>>)
        {
            if steps < marked[pos.0][pos.1] {
                marked[pos.0][pos.1] = steps;
            } else {
                return;
            }

            const WAYS: [(isize,isize); 4] = [(1,0),(0,1),(-1,0),(0,-1)];

            for way in WAYS {
                let n_pos = (pos.0.checked_add_signed(way.0).unwrap(), pos.1.checked_add_signed(way.1).unwrap());

                let cell = cells[n_pos.0][n_pos.1];
                match cell {
                    Place::Free => {
                        recurse(n_pos, cells, steps+1, marked);
                    }
                    Place::Portal(to) => {
                        recurse(to, cells, steps + 2, marked);
                    }
                    _ => ()
                }
            }
        }
        recurse(start, &cells, 0, &mut marked);

        println!("end: {}", marked[end.0][end.1]);
    }
}

pub fn part_2() {
    use std::thread;
    let a = thread::Builder::new().stack_size(4294967296).spawn(do_part_2).unwrap();
    println!("sthread");
    a.join().unwrap();
}

fn do_part_2()
{
    println!("running thread");
    type Pos = (usize, usize);

    #[derive(Debug, Copy, Clone)]
    enum Place {
        /// passage in donut
        Free, 
        Wall,
        /// space outside donut
        Outside, 
        Portal(Pos, i8)
    }

    {
        let content = include_str!("day20.txt");

        println!("{}", content);

        // first get all names and their positions

        // create 2d vector of all characters
        let mut data = Vec::<Vec<char>>::new();
        data.push(Vec::new());
        for c in content.chars() {
            if c == '\n' { data.push(Vec::new()) }
            if c != '\n' && c != '\r' {
                data.last_mut().unwrap().push(c);
            }
        }
        data.push(Vec::new());
        data.last_mut().unwrap().append(&mut vec![' '; 200]);
        data.iter_mut().for_each(|k| k.push(' '));
        println!("{:?}", data);

        // iterate through 2D vector, spotting names of portals

        let mut map = HashMap::new();

        let mut portals = Vec::new();

        let width = data[0].len();
        let height = data.len();

        let mut add_to_map = |name: (char,char), pos: Pos| 
        {
            let di = if (pos.1 < 5) || (pos.1 > (width - 5)) || (pos.0 < 5) || (pos.0 > (height - 5)) {-1} else {1};
            let asd = (pos, di);
            map.entry(name).and_modify(|k|
            {  
                portals.push((*k, asd));
            }).or_insert(asd);
        };

        for i in 0..data.len() {
            for j in 0..data[i].len() {
                let item = data[i][j];
                if item.is_alphabetic() {
                    if data[i + 1][j].is_alphabetic() {
                        // we have got a vertical portal
                        if data[i + 2][j] == '.' {
                            // portal is below
                            add_to_map((item, data[i + 1][j]), (i + 2, j));
                        } else {
                            // portal is above
                            add_to_map( (item, data[i + 1][j]), (i - 1, j));
                        }
                    } else if data[i][j + 1].is_alphabetic() {
                        // horizontal
                        if data[i][j + 2] == '.' {
                            // right
                            add_to_map((item, data[i][j + 1]), (i, j + 2));
                        } else {
                            // left
                            add_to_map((item, data[i][j + 1]), (i, j - 1));
                        }
                    }
                }
            }
        }
        println!("{:?}", map);
        println!("{:?}", portals);

        let start = *map.get(&('A','A')).unwrap();
        let end = *map.get(&('Z','Z')).unwrap();

        let mut cells = Vec::new();
        

        for i in 0..data.len() {
            cells.push(Vec::new());
            for j in 0..data[i].len() {
                let item = data[i][j];
                let t = cells.last_mut().unwrap();
                match item {
                    '#' => t.push(Place::Wall),
                    '.' => t.push(Place::Free),
                    _   => t.push(Place::Outside),
                }
            }
        }
        

        // add portals //and end

        for p in portals {
            cells[p.0.0.0][p.0.0.1] = Place::Portal(p.1.0, p.0.1);
            cells[p.1.0.0][p.1.0.1] = Place::Portal(p.0.0, p.1.1);
        }
        //cells[end.0][end.1] = Place::End; // should not be important to depth first search
        
        println!("{:?}", cells);

        // do a depth first search with a matrix for how long it took to get to everywhere

        let mut marked = Vec::new();
        for _ in 0..30 { // levels deep
            marked.push(Vec::new());
        }
        for dim in 0..marked.len() {
            for i in 0..cells.len() {
                marked[dim].push(Vec::new());
                for j in 0..cells[i].len() {
                    marked[dim].last_mut().unwrap().push(usize::MAX);
                }
            }
        }

        fn recurse(pos: Pos, cells: &Vec<Vec<Place>>, steps: usize, marked: &mut Vec<Vec<Vec<usize>>>, dim: i8)
        {
            if dim < 0 {
                return;
            }
            if (dim as usize) >= marked.len() {
                return;
            }
            if steps < marked[dim as usize][pos.0][pos.1] {
                marked[dim as usize][pos.0][pos.1] = steps;
            } else {
                return;
            }

            const WAYS: [(isize,isize); 4] = [(1,0),(0,1),(-1,0),(0,-1)];

            for way in WAYS {
                let n_pos = (pos.0.checked_add_signed(way.0).unwrap(), pos.1.checked_add_signed(way.1).unwrap());

                let cell = cells[n_pos.0][n_pos.1];
                match cell {
                    Place::Free => {
                        recurse(n_pos, cells, steps+1, marked, dim);
                    }
                    Place::Portal(to, dim_change) => {
                        recurse(to, cells, steps + 2, marked, dim + dim_change);
                    }
                    _ => ()
                }
            }
        }
        recurse((start.0.0, start.0.1), &cells, 0, &mut marked, 0);

        println!("end: {}", marked[0][end.0.0][end.0.1]);
        //println!("{:?}", marked);
    }
}