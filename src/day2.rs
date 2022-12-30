
pub fn get_ints_from_string(string: &str) -> Vec<i32>
{
    string.split(',')
        .map(|substr| str::parse(substr).unwrap())
        .collect()
}

pub fn part_1()
{
    let mut codes;
    {
        let contents = std::fs::read_to_string("src/day2.txt")
            .expect("Should have been able to read the file");
        
        codes = get_ints_from_string(&contents);
        println!("s:{:?}", codes);
    }

    codes[1] = 12;
    codes[2] = 2;

    for i in (0..codes.len()).step_by(4)
    {
        // match, if 1 add the two numbers where the two folling numbers point to, write to where the third number says to write to.
        match codes[i] {
            1 => {
                let num1 = codes[i+1] as usize;
                let num2 = codes[i+2] as usize;
                let location = codes[i+3] as usize;
                let sum = codes[num1] + codes[num2];
                codes[location] = sum;
            }
            2 => {
                let num1 = codes[i+1] as usize;
                let num2 = codes[i+2] as usize;
                let location = codes[i+3] as usize;
                let prod = codes[num1] * codes[num2];
                codes[location] = prod;
            }
            99 => {
                break;
            }
            _ => {
                panic!("wrong number: {}", codes[i]);
            }
        }

    }
    println!("Results: {}", codes[0]);
}

pub fn part_2()
{
    let start_codes: Vec<i64>;
    {
        let contents = std::fs::read_to_string("src/day2.txt")
            .expect("Should have been able to read the file");
        
        start_codes = get_ints_from_string(&contents).iter().map(|x| *x as i64).collect();
        //println!("s:{:?}", start_codes);
    }
    let start_codes = start_codes;

    let (corr_noun, corr_verb) = 'outer: {
        for noun in 0..=99 {
            for verb in 0..=99
            {
                let mut codes = start_codes.clone();
                codes[1] = noun;
                codes[2] = verb;

                for i in (0..codes.len()).step_by(4)
                {
                    // match, if 1 add the two numbers where the two folling numbers point to, write to where the third number says to write to.
                    match codes[i] {
                        1 => {
                            let num1 = codes[i+1] as usize;
                            let num2 = codes[i+2] as usize;
                            let location = codes[i+3] as usize;
                            let sum = codes[num1] + codes[num2];
                            codes[location] = sum;
                        }
                        2 => {
                            let num1 = codes[i+1] as usize;
                            let num2 = codes[i+2] as usize;
                            let location = codes[i+3] as usize;
                            let prod = codes[num1] * codes[num2];
                            codes[location] = prod;
                        }
                        99 => {
                            break;
                        }
                        _ => {
                            panic!("wrong number: {}", codes[i]);
                        }
                    }
                }
                let result = codes[0];
                if result == 19690720 {
                    break 'outer (noun, verb);
                }
            }
        }
        (-1,-1)
    };
    println!("Noun: {corr_noun}, Verb: {corr_verb}, res {}", (100*corr_noun + corr_verb));
}