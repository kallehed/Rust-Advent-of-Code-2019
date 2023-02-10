pub fn part_1()
{
    let mut total = 0;
    for x in 0..50 {
        for y in 0..50 {
            if ask(x,y) {
                total += 1;
                println!("x: {}, y: {}", x, y);
            }
        }
    }
    println!("res: {}", total);
}

const SIZE: usize = 1000;

#[allow(clippy::collapsible_if)]
pub fn part_2()
{
    let mut x = 0;

    let mut best_x = 10000000;
    let mut best_y = 10000000;


    let mut highest_y = 0;
    loop {
        //println!("x: {}", x);
        let mut found = false;
        for y in (highest_y)..1000 {
            if ask(x, y) {
                if !found {
                    found = true;
                    highest_y = y;
                }
                if ask(x + 99, y) && ask(x, y + 99) && ask(x+99, y+99) {
                    if x + y < best_x + best_y {
                        best_x = x;
                        best_y = y;
                    }
                } else {
                    
                }
            } else if found {
                    // found end
                    break;
            }
        }
        x += 1;
        if x > 1500 {
            break;
        }
    }
    println!("RES: {}", best_x * 10000 + best_y);

    
}

fn ask(drone_x: i32, drone_y: i32) -> bool
{
    let mut codes: Vec<i64>;
    {

        let contents = std::fs::read_to_string("src/day19.txt")
            .expect("Should have been able to read the file");
        
        codes = contents.split(',').map(|num_str| num_str.parse().unwrap()).collect();

        codes.resize(codes.len() + 1000, 0);
    }
    //println!("codes: {:?}", codes);
    let mut to_say = 0;
    
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
                        let inp = if to_say == 0 {drone_x} else {drone_y};
                        to_say = 1;
                        //println!("Gave input: {}", inp);
                        codes[loc0] = inp as _;
                        i += 2;
                    }
                    4 => { // output value
                        //println!("Output: {}", codes[loc0]);
                        return codes[loc0] > 0;
                        //to_say = 0;
                        //i += 2;
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
                unreachable!();
                //break;
            }
            _ => {
                panic!("Wrong instruction: {}", code);
            }
        }
    }

}