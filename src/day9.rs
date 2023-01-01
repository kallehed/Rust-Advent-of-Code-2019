
fn compute(inp: i64)
{
    let mut codes: Vec<i64>;
    {

        let contents = std::fs::read_to_string("src/day9.txt")
            .expect("Should have been able to read the file");
        
        codes = contents.split(',').map(|num_str| num_str.parse().unwrap()).collect();

        codes.resize(codes.len() + 1000, 0);
    }
    //println!("codes: {:?}", codes);
    
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
                        println!("Gave input: {}", inp);
                        codes[loc0] = inp;
                        i += 2;
                    }
                    4 => { // output value
                        println!("Output: {}", codes[loc0]);
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


pub fn part_1()
{
    compute(1);
}

pub fn part_2()
{
    compute(2);
}