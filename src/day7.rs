use std::collections::HashSet;

fn calc_intcode(mut codes: Vec<i32>, inputs: &[i32; 2]) -> i32
{
    let mut i: usize = 0;
    let mut inputs_given = 0;
    loop 
    {
        let code = codes[i];
        let op_code = code % 100;

        match op_code {
            op if ((1..=2).contains(&op)) || ((5..=8).contains(&op)) =>
            {
                let first_mode = code % 1000 - op_code;
                let sec_mode = code % 10000 - (first_mode + op_code);

                let mut elem0 = codes[i + 1];
                let mut elem1 = codes[i + 2];
                let pos = codes[i + 3] as usize; 

                if first_mode == 0 {elem0 = codes[elem0 as usize];}
                if sec_mode == 0 {elem1 = codes[elem1 as usize];}

                match op {
                    1 => {
                        codes[pos] = elem0 + elem1;
                        i += 4;
                    }
                    2 => {
                        codes[pos] = elem0 * elem1;
                        i += 4;
                    }
                    5 => { // jump if true
                        if elem0 != 0 {
                            i = elem1 as usize;
                        } else {
                            i += 3;
                        }
                    }
                    6 => { // jump if false
                        if elem0 == 0 {
                            i = elem1 as usize;
                        } else {
                            i += 3;
                        }
                    }
                    7 => {
                        codes[pos] = (elem0 < elem1) as i32;
                        i += 4;
                    }
                    8 => {
                        codes[pos] = (elem0 == elem1) as i32;
                        i += 4;
                    }
                    _ => {
                        panic!("very invalid: {}", code);
                    }
                }

            }
            3 => { // take input 5 and store it at next
                let pos = codes[i + 1];
                codes[pos as usize] = inputs[inputs_given];
                inputs_given += 1;
                i += 2;
            }
            4 => { // output value at position
                let pos = codes[i + 1];
                //println!("Output: {}", codes[pos as usize]);
                return codes[pos as usize];
                //i += 2;
            }
            /*99 => {
                break;
            }*/
            _ => {
                panic!("Wrong instruction: {}", code);
            }
        }
    }
}

fn thruster_signal(codes: &Vec<i32>, phase_setting: [i32; 5]) -> i32
{
    let mut input = 0;
    for setting in phase_setting
    {
        let inputs = [setting, input];
        input = calc_intcode((*codes).clone(), &inputs);
    }
    input
}


pub fn part_1()
{
    let mut codes: Vec<i32>;
    {

        let contents = std::fs::read_to_string("src/day7.txt")
            .expect("Should have been able to read the file");
        
        codes = contents.split(',').map(|num_str| num_str.parse().unwrap()).collect();
    }
    println!("codes: {:?}", codes);

    let mut biggest = 0;

    for n0 in 0..=4 {
        for n1 in 0..=4 {
            for n2 in 0..=4 {
                for n3 in 0..=4 {
                    for n4 in 0..=4 {
                        let setting = [n0,n1,n2,n3,n4];
                        let sig = thruster_signal(&codes, setting);
                        if sig > biggest && {
                            let mut h = HashSet::new();
                            for sett in setting {h.insert(sett);}
                            h.len() == 5
                        } {
                            biggest = sig;
                            println!("val: {}, matric: {:?}", biggest, setting);
                        }
                    }
                }
            }
        }
    }
    println!("signal: {}", biggest);
    


}