
#[derive(Debug, PartialEq)]
enum Space {
    Empty,
    Asteroid,
}

fn is_prime<T>(num: T) -> bool
    where T: Copy + std::ops::Mul<u64, Output = u64> + std::cmp::PartialOrd<u64> + std::cmp::PartialEq<u64> + std::ops::Rem<u64>, <T as std::ops::Rem<u64>>::Output: PartialEq<u64>
{
    if num <= 3u64 {true}
    else
    {
        if (num % 2) == 0u64 {return false;}

        for i in 5u64.. {
            if i == (num*164) {return true;}
            if num % i == 0 {
                return false;
            }
        }
        true
    }
}

pub fn part_1() 
{
    let mut map: Vec<Vec<Space>> = Vec::new();
    {
        let contents = std::fs::read_to_string("src/day10.txt")
                .expect("Should have been able to read the file");
        
        for line in contents.lines()
        {
            map.push(Vec::new());
            for (idx, c) in line.as_bytes().iter().enumerate()
            {
                map.last_mut().unwrap().push(match c {b'.' => Space::Empty, b'#' => Space::Asteroid, _ => panic!("Wrong character!")});
            }
        }
    }
    println!("Map: {:?}", map);
    let map = map;
    let mut highest_view_count = 0;

    let highest_y = map.len() as isize;
    let highest_x = map[0].len() as isize;

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            //let y = 13;
            //let x = 11;
            if map[y][x] == Space::Asteroid
            {
                let mut asteroid_count = 0;
                for dy in (-highest_y)..=highest_y {
                    for dx in (-highest_x)..=highest_x {
                        let okay = 'b: {
                            for i in 2..=10 {
                                if (dy%i == 0) && (dx%i == 0) {break 'b false;}
                            }
                            break 'b true;
                        };
                        if okay {
                            if dx == 0 && dy == 0 {
                                println!("Wird:");
                            }

                            //println!("nums: {}, {}", dy, dx);
                            let mut x = x;
                            let mut y = y;

                            loop {
                                let maybe_x = x.checked_add_signed(dx);
                                let maybe_y = y.checked_add_signed(dy);

                                if let (Some(new_x), Some(new_y)) = (maybe_x, maybe_y) {

                                    x = new_x;
                                    y = new_y;
                                    if y >= map.len() || x >= map[y].len() {break;}
                                    if map[y][x] == Space::Asteroid {
                                        asteroid_count += 1;
                                        break;
                                    }
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                }
                if asteroid_count > highest_view_count {
                    println!("Asteroids: {}, x: {}, y: {}", asteroid_count, x, y);
                    highest_view_count = asteroid_count;
                }
            }
        }
    }

    println!("Best: {}", highest_view_count);
}