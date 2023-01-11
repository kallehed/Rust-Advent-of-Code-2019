
pub fn part_1()
{
    const DIGITS: usize = 650;
    let mut digits = [0i16; DIGITS];
    {
        let contents = std::fs::read_to_string("src/day16.txt")
            .expect("Should have been able to read the file");
        assert_eq!(DIGITS, contents.len());
        for (i, c) in contents.as_bytes().iter().enumerate() {
            digits[i] = (*c - b'0') as i16;
        }
    }
    println!("{:?}", digits);
    const BASE_PATTERN: [i16; 4] = [0,1,0,-1];

    for _phase in 1..=100
    {
        let mut new_digits = [0i16; DIGITS];
        for (idx, new_digit) in new_digits.iter_mut().enumerate() {
            // create the pattern and start at the second
            let per = idx + 1;
            let mut pattern = Vec::with_capacity(per * BASE_PATTERN.len());
            for b in BASE_PATTERN {
                for _ in 0..per {pattern.push(b);}
            }

            let mut j = 1; // index in pattern, which starts at 1
            let mut sum: i16 = 0;

            for digit in digits {
                sum += digit * pattern[j];
                
                j += 1;
                if j == pattern.len() {j = 0;} // wrap around
            }
            *new_digit = (sum%10).abs(); // get first digit
        }
        digits = new_digits;
    }
    fn print_first_8_digits(array: &[i16; DIGITS]) {
        for d in array.iter().take(8)  {
            print!("{}", d);
        }
    }
    //println!("digits: {:?}", digits);
    print_first_8_digits(&digits);
}

pub fn part_2()
{
    let child = std::thread::Builder::new()
        .stack_size(33554432*2)
        .spawn(do_part_2)
        .unwrap();
    child.join().unwrap();
}

type Int = i8;

fn do_part_2() // you could optimize it alot by taking partial sums
{
    const DIGITS: usize = 6500000;
    let mut digits = [0; DIGITS];
    {
        let contents = std::fs::read_to_string("src/day16.txt")
            .expect("Should have been able to read the file");

        let contents = contents.repeat(10000);
        
        assert_eq!(DIGITS, contents.len());
        for (i, c) in contents.as_bytes().iter().enumerate() {
            digits[i] = (*c - b'0') as Int;
        }
    }

    // first seven digits form:
    let mut first_seven_digits: usize = 0;
    {
        let mut mult = 1;
        for digit in digits[0..7].iter().rev() {
            first_seven_digits += (*digit as usize)*mult;
            mult *= 10;
        }
    }
    println!("first seven digits: {}", first_seven_digits);
    //println!("{:?}", digits);

    for _phase in 1..=100
    {
        let mut new_digits = [0; DIGITS];
        for idx in (first_seven_digits..DIGITS).rev() {
            // create the pattern and start at the second

            let mut sum: i32 = digits[idx] as _;
            if idx + 1 != DIGITS {
                sum += new_digits[idx + 1] as i32;
            }

            new_digits[idx] = ((sum % 10) as Int).abs(); // get first digit
        }
        
        digits = new_digits;
    }
    fn print_first_8_digits(array: &[Int; DIGITS], skip: usize) {
        for d in array.iter().skip(skip).take(8)  {
            print!("{}", d);
        }
    }
    print_first_8_digits(&digits, first_seven_digits);
}