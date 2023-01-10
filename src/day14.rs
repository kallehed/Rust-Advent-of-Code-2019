use std::{collections::HashMap, any};

type OreId = usize;
#[derive(Copy,Clone, Eq, PartialEq, Debug)]
struct OreInRecipe
{
    ore_id: OreId,
    amount: usize,
}

#[derive(Debug, Default)]
struct Recipe
{
    get: usize, // what you get
    takes: Vec<OreInRecipe> // what it takes
}

pub fn part_1()
{
    let contents;
    let mut chars: Vec<char>;
    let mut names_to_id = HashMap::new();
    
    {
        contents = std::fs::read_to_string("src/day14.txt")
            .expect("Should have been able to read the file");

        // read file and map all names("ORE" or "FUEL") to their own id's
        chars = contents.chars().collect();
        chars.push('\0');
        let mut start_name: isize = -1;
        for i in 0..chars.len() {
            let c = chars[i];
            if start_name == -1 {
                if c.is_alphabetic() 
                {
                    start_name = i as isize;
                } 
            } else if !c.is_alphabetic()
            {
                // we have a name in [start_name..i]

                let name = &contents[(start_name as usize)..i];
                if !names_to_id.contains_key(name) {
                    names_to_id.insert(name, names_to_id.len());
                }

                start_name = -1;
            }
        }
    }
    let names_to_id = names_to_id;
    println!("{:?}", names_to_id);

    let mut recipes = Vec::new();

    // write down recipes
    for line in contents.lines() {
        let making_id;
        let get;
        let mut takes = Vec::new();

        let mut parts = line.split(" => ");
        { // look at what it takes to make it
            let amt_and_ores = parts.next().unwrap().split(", ");
            for spec_ore in amt_and_ores {
                let mut num_and_name = spec_ore.split(' ');
                let amount = num_and_name.next().unwrap().parse().unwrap();
                let ore_id = *names_to_id.get(num_and_name.next().unwrap()).unwrap();
                takes.push(OreInRecipe { ore_id, amount });
            }
        }
        { // look at what we are making
            let mut what_making = parts.next().unwrap().split(' ');
            get = what_making.next().unwrap().parse().unwrap();
            making_id = *names_to_id.get(what_making.next().unwrap()).unwrap() as OreId;
        }
        while recipes.len() <= making_id {recipes.push(Default::default());}
        recipes[making_id] = Recipe{get, takes}
    }
    println!("Recipes: {:?}", recipes);

    let fuel_id = *names_to_id.get("FUEL").unwrap();
    let ore_id = *names_to_id.get("ORE").unwrap();
    println!("Fuel ID: {}, Ore ID: {}", fuel_id, ore_id);


    let mut needed = vec![0isize; recipes.len()];
    needed[fuel_id] = 1; // you need 1 fuel
    let mut do_excess = false;
    loop {
        let mut anything_changed = false;
        for idx in 0..needed.len() {
            if idx == ore_id {continue;} // ore can not be made.
            let need = needed[idx];
            if need > 0 {
                let get =  recipes[idx].get;
                if need >= get as isize { // if we get less or eq what we need, make one "portion", could make more next loop
                    anything_changed = true;
                    needed[idx] -= get as isize;
                    // we can get exactly what we want by getting these ingredients
                    for ore in recipes[idx].takes.iter() {
                        needed[ore.ore_id] += ore.amount as isize;
                    }
                } else if do_excess {
                    // we can make too much ore. 
                    anything_changed = true;
                    do_excess = false;

                    needed[idx] -= get as isize;
                    for ore in recipes[idx].takes.iter() {
                        needed[ore.ore_id] += ore.amount as isize;
                    }
                }
            }
        }
        if !anything_changed {
            // we must make excess of something.
            if do_excess {
                // stalemate
                break;
            }
            do_excess = true;
        }

        //println!("needed: {:?}", needed);
    }
    println!("Ore needed: {}", needed[ore_id]);

}

pub fn part_2()
{
    let contents;
    let mut chars: Vec<char>;
    let mut names_to_id = HashMap::new();
    
    {
        contents = std::fs::read_to_string("src/day14.txt")
            .expect("Should have been able to read the file");

        // read file and map all names("ORE" or "FUEL") to their own id's
        chars = contents.chars().collect();
        chars.push('\0');
        let mut start_name: isize = -1;
        for i in 0..chars.len() {
            let c = chars[i];
            if start_name == -1 {
                if c.is_alphabetic() 
                {
                    start_name = i as isize;
                } 
            } else if !c.is_alphabetic()
            {
                // we have a name in [start_name..i]

                let name = &contents[(start_name as usize)..i];
                if !names_to_id.contains_key(name) {
                    names_to_id.insert(name, names_to_id.len());
                }

                start_name = -1;
            }
        }
    }
    let names_to_id = names_to_id;
    println!("{:?}", names_to_id);

    let mut recipes = Vec::new();

    // write down recipes
    for line in contents.lines() {
        let making_id;
        let get;
        let mut takes = Vec::new();

        let mut parts = line.split(" => ");
        { // look at what it takes to make it
            let amt_and_ores = parts.next().unwrap().split(", ");
            for spec_ore in amt_and_ores {
                let mut num_and_name = spec_ore.split(' ');
                let amount = num_and_name.next().unwrap().parse().unwrap();
                let ore_id = *names_to_id.get(num_and_name.next().unwrap()).unwrap();
                takes.push(OreInRecipe { ore_id, amount });
            }
        }
        { // look at what we are making
            let mut what_making = parts.next().unwrap().split(' ');
            get = what_making.next().unwrap().parse().unwrap();
            making_id = *names_to_id.get(what_making.next().unwrap()).unwrap() as OreId;
        }
        while recipes.len() <= making_id {recipes.push(Default::default());}
        recipes[making_id] = Recipe{get, takes}
    }
    println!("Recipes: {:?}", recipes);

    let fuel_id = *names_to_id.get("FUEL").unwrap();
    let ore_id = *names_to_id.get("ORE").unwrap();
    println!("Fuel ID: {}, Ore ID: {}", fuel_id, ore_id);

    let works = |fuel_to_produce|
    {
        let mut needed = vec![0isize; recipes.len()];
        needed[fuel_id] = fuel_to_produce ; // you need 1 fuel
        let mut do_excess = false;
        loop {
            let mut anything_changed = false;
            for idx in 0..needed.len() {
                if idx == ore_id {continue;} // ore can not be made.
                let need = needed[idx];
                if need > 0 {
                    let get =  recipes[idx].get;
                    if need >= get as isize { // if we get less or eq what we need, make one "portion", could make more next loop
                        anything_changed = true;
                        let times = need / get as isize;
                        needed[idx] -= get as isize * times;
                        // we can get exactly what we want by getting these ingredients
                        for ore in recipes[idx].takes.iter() {
                            needed[ore.ore_id] += ore.amount as isize * times;
                        }
                    } else if do_excess {
                        // we can make too much ore. 
                        anything_changed = true;
                        do_excess = false;

                        needed[idx] -= get as isize;
                        for ore in recipes[idx].takes.iter() {
                            needed[ore.ore_id] += ore.amount as isize;
                        }
                    }
                }
            }
            if !anything_changed {
                // we must make excess of something.
                if do_excess {
                    // stalemate
                    break;
                }
                do_excess = true;
            }

            //println!("needed: {:?}", needed);
        }
        needed[ore_id] <= 1000000000000 // return whether this worked or not
    };
    // find the highest that works
    let mut lowest_work = 1;
    let mut highest_not_work = 10000000; // a very big number guaranteed to not work
    loop {
        let next = (highest_not_work + lowest_work) / 2;
        if works(next) {
            lowest_work = next;
        }
        else {
            highest_not_work = next;
        }
        if lowest_work + 1 == highest_not_work {
            break;
        }
    }

    println!("res: {}", lowest_work);

}