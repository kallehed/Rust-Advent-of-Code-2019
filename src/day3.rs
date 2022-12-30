
pub fn part_1()
{
    use std::collections::HashSet;
    let mut places_arr = [(); 2].map(|_| HashSet::new());

    {
        let contents = std::fs::read_to_string("src/day3.txt")
            .expect("Should have been able to read the file");
        
        for (index,line) in contents.lines().enumerate()
        {
            let places = &mut places_arr[index];
            let (mut x, mut y): (i32,i32) = (0, 0);

            for dir in line.split(',')
            {
                let way = *dir.as_bytes().first().unwrap();
                let times: i32 = (dir[1..]).parse().unwrap();
                println!("way: {}, times: {times}", way as char);
                
                let mov = match way {
                    b'R' => (1,0),
                    b'D' => (0,-1),
                    b'L' => (-1,0),
                    b'U' => (0,1),
                    _ => panic!("ILLEGAL DIR: {}", way as char)
                };

                for _ in 0..times
                {
                    x += mov.0;
                    y += mov.1;
                    let place = (x,y);
                    places.insert(place);
                }
            }
        }   
    }
    // for the positions that overlap, compute the smallest of their manhattan-distances
    //let v = places_arr[0].intersection(&places_arr[1]).fold(i32::MAX, |smallest, val| std::cmp::min(smallest, val.0.abs() + val.1.abs())); // this works too
    let v = places_arr[0].intersection(&places_arr[1])
                .map(|pos| pos.0.abs() + pos.1.abs())
                .min().unwrap();
    println!("{}",v);
}


pub fn part_2()
{
    use std::collections::HashMap;
    let mut places_arr = [(); 2].map(|_| HashMap::new());

    {
        let contents = std::fs::read_to_string("src/day3.txt")
            .expect("Should have been able to read the file");
        
        for (index,line) in contents.lines().enumerate()
        {
            let places = &mut places_arr[index];
            let (mut x, mut y): (i32,i32) = (0, 0);
            let mut steps = 0;

            for dir in line.split(',')
            {
                let way = *dir.as_bytes().first().unwrap();
                let times: i32 = (dir[1..]).parse().unwrap();
                println!("way: {}, times: {times}", way as char);
                
                let mov = match way {
                    b'R' => (1,0),
                    b'D' => (0,-1),
                    b'L' => (-1,0),
                    b'U' => (0,1),
                    _ => panic!("ILLEGAL DIR: {}", way as char)
                };

                for _ in 0..times
                {
                    x += mov.0;
                    y += mov.1;
                    steps += 1;
                    let place = (x,y);
                    places.entry(place).or_insert(steps);
                }
            }
        }
    }

    // first, get the positions that overlap, then get the minimum of the steps to get there for both the pipes

    let v = places_arr[0].keys().filter(|place| places_arr[1].contains_key(place))
    .fold(i32::MAX, |smallest, key| std::cmp::min(smallest, places_arr[0].get(key).unwrap() + places_arr[1].get(key).unwrap()));
    
    println!("{}",v);
}