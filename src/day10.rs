use std::f64::{consts::PI};


#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Space {
    Empty,
    Asteroid,
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
            for c in line.as_bytes()
            {
                map.last_mut().unwrap().push(match c {b'.' => Space::Empty, b'#' => Space::Asteroid, _ => panic!("Wrong character!")});
            }
        }
    }
    println!("Map: {:?}", map);
    let map = map;
    let mut highest_visible = 0;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == Space::Empty {continue;}

            let mut map = map.clone();

            for oy in 0..map.len() {
                for ox in 0..map[0].len() {
                    if y == oy && x == ox {continue;}
                    if map[oy][ox] == Space::Empty {continue;}

                    let dx = ox as isize - x as isize;
                    let dy = oy as isize - y as isize;

                    use gcd::Gcd;
                    let common_div = dx.unsigned_abs().gcd(dy.unsigned_abs());
                    let dx = dx / common_div as isize;
                    let dy = dy / common_div as isize;

                    let mut nx = ox as isize;
                    let mut ny = oy as isize;

                    loop // go behind and mark spots that can't be seen
                    {
                        nx += dx;
                        ny += dy;
                        if  nx < 0 || ny < 0 || ny >= map.len() as isize || nx >= map[0].len() as isize {break;}
                        map[ny as usize][nx as usize] = Space::Empty;
                    }
                }
            }
            // count asteroids left
            let mut visible = 0;
            for oy in 0..map.len() {
                for ox in 0..map[0].len() {
                    if y == oy && x == ox {continue;}
                    if map[oy][ox] == Space::Asteroid {visible += 1;}
                }
            }
            println!("x: {}, y: {}, Visible: {}",x, y, visible);
            highest_visible = std::cmp::max(highest_visible, visible);
        }
    }
    println!("Highest: {}", highest_visible);

}

pub fn part_2() // x: 20, y: 21 is the best
{
    let mut map: Vec<Vec<Space>> = Vec::new();
    {
        let contents = std::fs::read_to_string("src/day10.txt")
                .expect("Should have been able to read the file");
        
        for line in contents.lines()
        {
            map.push(Vec::new());
            for c in line.as_bytes()
            {
                map.last_mut().unwrap().push(match c {b'.' => Space::Empty, b'#' => Space::Asteroid, _ => panic!("Wrong character!")});
            }
        }
    }
    println!("Map: {:?}", map);

    let mut angle = PI/2.0; // angle, which will be moved to the next asteroid(plus a little extra) over and over again

    const SHOOT_X: usize = 20; // where the laser is placed
    const SHOOT_Y: usize = 21; 

    let mut destroyed = 0;

    loop {
        // find the closest asteroid to destroy
        let mut smallest_angle_dif = f64::MAX;
        let mut distance_of = f64::MAX;
        let mut best_pos = (usize::MAX,usize::MAX); // x and y
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                if (x == SHOOT_X && y == SHOOT_Y) || map[y][x] == Space::Empty {continue;}
                let rel_x = (x as f64) - (SHOOT_X as f64);
                let rel_y = (SHOOT_Y as f64) - (y as f64); // array coordinates are upside down to math coordinates

                let aster_angle = f64::atan2(rel_y, rel_x);
                let mut rel_angle = angle - aster_angle;
                if rel_angle < 0.0 {rel_angle += std::f64::consts::TAU;}
                let dif = smallest_angle_dif - rel_angle;
                let distance = rel_x.abs() + rel_y.abs();
                if dif.abs() <= f64::EPSILON {
                    if distance < distance_of {
                        distance_of = distance;
                        best_pos = (x, y);
                    }
                }
                else if dif > f64::EPSILON {
                    smallest_angle_dif = rel_angle;
                    distance_of = distance;
                    best_pos = (x, y);
                }
            }
        }
        angle -= smallest_angle_dif + f64::EPSILON*10.0;
        if angle < (-std::f64::consts::PI) {angle += std::f64::consts::TAU;}

        // destroy asteroid
        map[best_pos.1][best_pos.0] = Space::Empty;
        destroyed += 1;

        println!("Destroyed {}: x:{}, y:{}", destroyed, best_pos.0, best_pos.1);
        if destroyed == 200 {
            println!("Result: {}", (100 * best_pos.0) + best_pos.1);
            break;
        }
        
    }
}