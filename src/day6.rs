pub fn part_1() // store all orbits in a map, then recursively go through each planet and count the chain to "COM". Kind of unoptimized solution, still runs instantly
{
    use std::collections::HashMap;
    let contents;
    let mut orbits = HashMap::new(); // what orbits what
    {
        contents = std::fs::read_to_string("src/day6.txt")
            .expect("Should have been able to read the file");

        for line in contents.lines() {
            let mut parts = line.split(')');
            let fir = parts.next().unwrap();
            let sec = parts.next().unwrap();
            orbits.insert(sec, fir);
        }
    }
    println!("orbits: {:?}", orbits);
    
    let mut total = 0;
    for mut value in orbits.values() {
        total += 1;

        while *value != "COM" {
            total += 1;
            value = orbits.get(value).unwrap();
        }
    }
    println!("Total: {}", total);
}

pub fn part_2() // walk from YOU, while at each step checking if you encountered the path from SAN to COM. 
{
    use std::collections::HashMap;
    let contents;
    let mut orbits = HashMap::new(); // what orbits what
    {
        contents = std::fs::read_to_string("src/day6.txt")
            .expect("Should have been able to read the file");

        for line in contents.lines() {
            let mut parts = line.split(')');
            let fir = parts.next().unwrap();
            let sec = parts.next().unwrap();
            orbits.insert(sec, fir);
        }
    }
    println!("orbits: {:?}", orbits);

    let mut value = orbits.get("YOU").unwrap();
    let mut you_steps = 0;
    'outer: loop {
        you_steps += 1;
        value = orbits.get(value).unwrap();
        
        let mut from_san = orbits.get("SAN").unwrap();
        let mut san_steps = 0;
        while *from_san != "COM" {
            san_steps += 1;
            from_san = orbits.get(from_san).unwrap();

            if value == from_san {
                println!("Steps total: {}", you_steps + san_steps);
                break 'outer;
            }
        }
    }
    
    
    
}