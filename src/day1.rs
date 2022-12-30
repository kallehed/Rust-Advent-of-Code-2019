
pub fn part_1()
{
    let mut numbers = Vec::<i32>::new();

    {

        let contents = std::fs::read_to_string("src/day1.txt")
            .expect("Should have been able to read the file");
        
        for line in contents.lines() {
            Vec::<i32>::push(&mut numbers, line.parse().expect("Not a number"));
        }

    }
   
    let mut sum = 0;

    for num in numbers {
        sum += (num / 3) - 2;
    }
    println!("sum: {sum}");

}

pub fn part_2()
{
    let mut numbers = Vec::<i32>::new();

    {
        let contents = std::fs::read_to_string("src/day1.txt")
            .expect("Should have been able to read the file");
        
        for line in contents.lines() {
            Vec::<i32>::push(&mut numbers, line.parse().expect("Not a number"));
        }
    }
   
    let mut sum = 0;

    for mut num in numbers.into_iter() {
        loop {
            num = (num / 3) - 2;
            if num > 0 {sum += num;}
            else {break;}
        }
    }
    println!("sum: {sum}");

}
