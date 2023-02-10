
type Pos = i8;
type Step = i32;
const HEIGHT: usize = 81;
const WIDTH: usize = 81;

const TOTAL_KEYS: usize = 26;
type Keys = [bool; TOTAL_KEYS];

type Tile = u8;
const AIR: Tile = (TOTAL_KEYS*2) as Tile;
const WALL: Tile = AIR + 1;
const FIRST_DOOR: Tile = TOTAL_KEYS as Tile; 

type Matrix = [[Tile; WIDTH]; HEIGHT];
use arrayvec::ArrayVec;
type Place = Vec::<(Tile, Step)>;
type Places = [Place; (AIR as usize)];

// For part 1 I first removed all the unnecessary stuff, leaving only doors and keys + how they are connected. Then I do a depth first search over getting the keys, in each search I do a breadth first search from the key I'm currently on, walking through doors and keys I've already been at. After I've found the shortest distances to the keys from the current key, I continue the depth based search to those keys. Still this was too slow, so I record the least steps taken at each depth, and if a search is taking too many, I cut it. 


pub fn part_1() 
{
    let mut view: Matrix = [[AIR; WIDTH]; HEIGHT];
    let mut start_i = 0;
    let mut start_j = 0;
    
    {
        let mut highest_key = 0;
        let contents = std::fs::read_to_string("src/day18.txt")
                .expect("Should have been able to read the file");

        assert_eq!(HEIGHT, contents.lines().count());
        assert_eq!(WIDTH, contents.lines().next().unwrap().len());

        for (i, line) in contents.lines().enumerate() {
            for (j,&a) in line.as_bytes().iter().enumerate() {
                match a {
                    b'.' => (),
                    b'#' => view[i][j] = WALL,
                    b'@' => {start_i = i; start_j = j;}
                    c => { // it is a door or lock
                        if (b'a'..=b'z').contains(&c) {
                            // key
                            let letter: Tile = c - b'a';
                            view[i][j] = letter;
                            if letter > highest_key {highest_key = letter;}
                        } else { 
                            // door 
                            view[i][j] = (c - b'A') + TOTAL_KEYS as Tile;
                        }
                    }
                }
            }
        }
        assert_eq!(TOTAL_KEYS, (highest_key + 1) as usize);
    }

    // from every key and door, get a vector of all the keys+doors it leads to and the steps to get there
    let mut goes_to: Places = std::array::from_fn(|i| Place::new());
    let mut start_goes_to = Place::new();
    // from every key and door, find the shortest paths to all keys+doors around it.
    {
        fn shortest_distances_to(start_i: Pos, start_j: Pos, mut view: Matrix) -> Place
        {
            let mut poses = Vec::new();
            let mut results = Place::new(); // contains tile which is walkable to, and how many steps it took to get there
            poses.push((start_i, start_j));
            let mut steps = 0;
            while !poses.is_empty() {
                steps += 1;
                for idx in (0..poses.len()).rev() {
                    let pos = poses.remove(idx);
                    const WAYS: [(Pos, Pos); 4] = [(1,0),(0,1),(-1,0),(0,-1)];
                    for way in WAYS {
                        let new_pos = (pos.0 + way.0, pos.1 + way.1);
                        let tile = &mut view[new_pos.0 as usize][new_pos.1 as usize];
                        if *tile < AIR {
                            results.push((*tile, steps));
                            *tile = WALL;
                        }
                        else if *tile == AIR {
                            *tile = WALL;
                            poses.push(new_pos);
                        }
                    }
                }
            }
            results
        }
        
        for i in 0..view.len() {
            for j in 0..view[0].len() {
                if i == start_i && j == start_j { // get where you can go from start
                    start_goes_to = shortest_distances_to(i as Pos, j as Pos, view);
                    continue;
                }
                let tile = view[i][j];
                if tile < AIR { // is either a door or key
                    let res = shortest_distances_to(i as Pos, j as Pos, view);

                    goes_to[tile as usize] = res;
                }
            }
        } 
    }

    let mut least_steps = Step::MAX;
    let mut best_at_each = [Step::MAX; TOTAL_KEYS];
    
    for place in start_goes_to {
        if place.0 < FIRST_DOOR {
            let mut keys = [false; TOTAL_KEYS];
            keys[place.0 as usize] = true;

            recurse(place.0, keys, 1, &goes_to, place.1, &mut least_steps, &mut best_at_each);
        }
        
    }

    
}

fn recurse(start_place: Tile, keys: Keys, total_keys: Tile, goes_to: &Places, mut steps_to_get_here: Step, least_steps: &mut Step, best_at_each: &mut [Step; TOTAL_KEYS])
{
   if steps_to_get_here.saturating_sub(900) > best_at_each[(total_keys - 1) as usize] {
        return;
    }
    else
    if steps_to_get_here < best_at_each[(total_keys - 1) as usize] {
        best_at_each[(total_keys-1) as usize] = steps_to_get_here;
    }

    if total_keys == TOTAL_KEYS as u8 {
        // all keys found!
        if steps_to_get_here < *least_steps {
            *least_steps = steps_to_get_here;
            println!("Took: {}", steps_to_get_here);
        }
        return;
    }
    use arrayvec::ArrayVec;
    let mut places = ArrayVec::<_, 14>::new(); // where they are and how many steps it took
    let mut djikstra = [Step::MAX; AIR as usize]; // min steps for each key+door
    places.push((start_place, steps_to_get_here));
    djikstra[start_place as usize] = steps_to_get_here;

    while !places.is_empty() {
        for idx in (0..places.len()).rev() {
            let (place, steps) = places.remove(idx);

            let it_goes_to = &goes_to[place as usize];
            for &(new_place, steps_there) in it_goes_to {
                let new_steps = steps + steps_there;

                // key and we DON'T have it
                if new_place < FIRST_DOOR && !keys[new_place as usize] {
                    if new_steps < djikstra[new_place as usize] {
                        djikstra[new_place as usize] = new_steps;
                    }
                    continue;
                }
                // we can't even touch the door, if no key
                if new_place >= FIRST_DOOR && !keys[((new_place as usize) - TOTAL_KEYS)] {
                    continue;
                }

                // if we have the key to the door, or we already have the key, maybe go there
                if new_steps < djikstra[new_place as usize] {
                    djikstra[new_place as usize] = new_steps;
                    places.push((new_place, new_steps)); 
                }
            }
        }
    }
    // look at all keys, if we don't have the key, and the steps are not Step::MAX, then recurse there
    // first though, sort them based on how many steps they took
    let mut paths = Vec::new();

    for tile in 0..FIRST_DOOR {
        if !keys[tile as usize] { // we don't have this key
            let steps = djikstra[tile as usize];
            if steps != Step::MAX {
                paths.push((tile, steps));
                // we could walk there!
                // then: create new keys where this one is true and recurse from this point
                
            }
        }
    }
    paths.sort_by_key(|p| p.1);

    for (tile, steps) in paths {
        let mut new_keys = keys;
        new_keys[tile as usize] = true;
        recurse(tile, new_keys, total_keys + 1, goes_to, steps, least_steps, best_at_each);
    }
}