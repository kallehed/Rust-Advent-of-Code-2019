
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Planet
{
    x: i64,
    y: i64,
    z: i64,
    x_vel: i64,
    y_vel: i64,
    z_vel: i64
}

pub fn part_1()
{
    let mut planets: [_; 4];
    {
        let contents = std::fs::read_to_string("src/day12.txt")
                .expect("Should have been able to read the file");
        let mut lines = contents.lines();
        planets = core::array::from_fn(|_|
            {
                let line = lines.next().unwrap();
                let mut after_eq = line.split('=');
                after_eq.next();
                let x: i64 = after_eq.next().unwrap().split(',').next().unwrap().parse().unwrap();
                let y: i64 = after_eq.next().unwrap().split(',').next().unwrap().parse().unwrap();
                let z: i64 = std::str::from_utf8(after_eq.next().unwrap().as_bytes().split_last().unwrap().1).unwrap().parse().unwrap();
                Planet{x, y, z, x_vel:0, y_vel:0, z_vel:0}
            }
        );
    }
    println!("Planets: {:?}", planets);

    for _ in 0..1000 {
        // apply gravity:
        for i0 in 0..(planets.len() - 1) {
            for i1 in (i0+1)..planets.len()
            {
                let grav = |p0: &mut Planet, p1: &Planet| // aplies gravity from one planet to another.
                {
                    let adjust = |num0: &mut i64, num1: i64, vel: &mut i64| // adjusts a specific axis
                    {
                        if num1 > *num0 {*vel += 1;}
                        else if num1 < *num0 {*vel -= 1;}
                    };
                    adjust(&mut p0.x, p1.x, &mut p0.x_vel);
                    adjust(&mut p0.y, p1.y, &mut p0.y_vel);
                    adjust(&mut p0.z, p1.z, &mut p0.z_vel);
                };

                let parts = planets.split_at_mut(i1);
                let p0 = &mut parts.0[i0];
                let p1 = &mut parts.1[0];
               
                grav(p0, p1); // succesfully get two references to the planets and then let BOTH of them gravitate towards each other.
                grav(p1, p0);
            }
        }
        // apply velocities
        for planet in &mut planets
        {
            planet.x += planet.x_vel;
            planet.y += planet.y_vel;
            planet.z += planet.z_vel;
        }
    }
    let mut total_energy = 0;
    for p in &planets {
        total_energy += (p.x.abs() + p.y.abs() + p.z.abs()) *  (p.x_vel.abs() + p.y_vel.abs() + p.z_vel.abs());
    }
    println!("Total energy: {}", total_energy);
}

pub fn part_2()
{
    let mut planets: [_; 4];
    {
        let contents = std::fs::read_to_string("src/day12.txt")
                .expect("Should have been able to read the file");
        let mut lines = contents.lines();
        planets = core::array::from_fn(|_|
            {
                let line = lines.next().unwrap();
                let mut after_eq = line.split('=');
                after_eq.next();
                let x: i64 = after_eq.next().unwrap().split(',').next().unwrap().parse().unwrap();
                let y: i64 = after_eq.next().unwrap().split(',').next().unwrap().parse().unwrap();
                let z: i64 = std::str::from_utf8(after_eq.next().unwrap().as_bytes().split_last().unwrap().1).unwrap().parse().unwrap();
                Planet{x, y, z, x_vel:0, y_vel:0, z_vel:0}
            }
        );
    }
    let start_planets = planets;
    println!("Planets: {:?}", start_planets);

    let mut x_time = -1_i64;
    let mut y_time = -1_i64;
    let mut z_time = -1_i64;

    'x_l: for steps in 1i64.. {
        for i0 in 0..(planets.len() - 1) {
            for i1 in (i0+1)..planets.len()
            {
                unsafe {
                    let p0 = &mut *std::ptr::addr_of_mut!(planets[i0]);
                    let p1 = &mut *std::ptr::addr_of_mut!(planets[i1]);

                    if p1.x > p0.x {p0.x_vel += 1;p1.x_vel -= 1;}
                    else if p1.x < p0.x {p0.x_vel -= 1;p1.x_vel += 1;}
                }
            }
        }
        for planet in &mut planets
        {
            planet.x += planet.x_vel;
        }
        if planets.iter().zip(start_planets).all(|(p, s_p)| p.x == s_p.x && p.x_vel == s_p.x_vel) {
            x_time = steps;
            break 'x_l;
        }
    }
    println!("X time: {}", x_time);

    'y_l: for steps in 1i64.. {
        for i0 in 0..(planets.len() - 1) {
            for i1 in (i0+1)..planets.len()
            {
                unsafe {
                    let p0 = &mut *std::ptr::addr_of_mut!(planets[i0]);
                    let p1 = &mut *std::ptr::addr_of_mut!(planets[i1]);

                    if p1.y > p0.y {p0.y_vel += 1;p1.y_vel -= 1;}
                    else if p1.y < p0.y {p0.y_vel -= 1;p1.y_vel += 1;}
                }
            }
        }
        for planet in &mut planets
        {
            planet.y += planet.y_vel;
        }
        if planets.iter().zip(start_planets).all(|(p, s_p)| p.y == s_p.y && p.y_vel == s_p.y_vel) {
            y_time = steps;
            break 'y_l;
        }
    }
    println!("Y time: {}", y_time);

    'z_l: for steps in 1i64.. {
        for i0 in 0..(planets.len() - 1) {
            for i1 in (i0+1)..planets.len()
            {
                unsafe {
                    let p0 = &mut *std::ptr::addr_of_mut!(planets[i0]);
                    let p1 = &mut *std::ptr::addr_of_mut!(planets[i1]);

                    if p1.z > p0.z {p0.z_vel += 1;p1.z_vel -= 1;}
                    else if p1.z < p0.z {p0.z_vel -= 1;p1.z_vel += 1;}
                }
            }
        }
        for planet in &mut planets
        {
            planet.z += planet.z_vel;
        }
        if planets.iter().zip(start_planets).all(|(p, s_p)| p.z == s_p.z && p.z_vel == s_p.z_vel) {
            z_time = steps;
            break 'z_l;
        }
    }
    println!("Z time: {}", z_time);

    // calculate total time it would take
    {
        use gcd::Gcd;
        let x_time = x_time as u64;
        let y_time = y_time as u64;
        let z_time = z_time as u64;

        // calculate the smallest number that modded to all of the times gives 0.
        let res = ((x_time.gcd(y_time).gcd(z_time))*(x_time * y_time * z_time)) / (x_time.gcd(y_time) * x_time.gcd(z_time)*y_time.gcd(z_time));
        println!("Res: {}", res);

    }

}