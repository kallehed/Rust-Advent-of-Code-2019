
pub fn part_1()
{

    let num1: i32;
    let num2: i32;
    {

        let contents = std::fs::read_to_string("src/day4.txt")
            .expect("Should have been able to read the file");
        
        let mut nums = contents.split('-');
        num1 = nums.next().unwrap().parse().unwrap();
        num2 = nums.next().unwrap().parse().unwrap();
    }
    println!("nums: {},{}", num1, num2);

    let mut total = 0;

    for num in num1..=num2 
    {
        let fits_pattern: bool = 
            (num > 100000 && num <= 999999) &&
            ('two_same: {
                for part in num.to_string().as_bytes().windows(2) {
                    if part[0] == part[1] {
                        break 'two_same true;
                    }
                }
                false
            }) &&
            ('increasing: {
                for part in num.to_string().as_bytes().windows(2) {
                    if part[0] > part[1] {
                        break 'increasing false;
                    }
                }
                true
            });
        if fits_pattern {
            total += 1;
        }
    }
    println!("Total: {}", total);

}

pub fn part_2()
{

    let num1: i32;
    let num2: i32;
    {

        let contents = std::fs::read_to_string("src/day4.txt")
            .expect("Should have been able to read the file");
        
        let mut nums = contents.split('-');
        num1 = nums.next().unwrap().parse().unwrap();
        num2 = nums.next().unwrap().parse().unwrap();
    }
    println!("nums: {},{}", num1, num2);

    let mut total = 0;

    for num in num1..=num2 
    {
        let fits_pattern: bool = 
            (num > 100000 && num <= 999999) &&
            ('two_same: {
                let temp = num.to_string();
                let seq = temp.as_bytes();
                for part in seq.windows(4) { // check middle
                    if part[1] == part[2] && part[0] != part[1] && part[3] != part[1] {
                        break 'two_same true;
                    }
                }
                (seq[0] == seq[1] && seq[0] != seq[2]) || (seq[seq.len()-1] == seq[seq.len()-2] && seq[seq.len() - 1] != seq[seq.len() - 3]) // check left and right
            }) &&
            ('increasing: {
                for part in num.to_string().as_bytes().windows(2) {
                    if part[0] > part[1] {
                        break 'increasing false;
                    }
                }
                true
            });
        if fits_pattern {
            total += 1;
        }
    }
    println!("Total: {}", total);

}