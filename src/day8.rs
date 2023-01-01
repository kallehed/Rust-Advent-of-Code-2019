
const WIDTH: i32 = 25;
const HEIGHT: i32 = 6;
const PIXELS_PER_LAYER: i32 = WIDTH*HEIGHT;

#[derive(Debug)]
struct Layer {
    zeros: i32,
    ones: i32,
    twos: i32,
}

pub fn part_1()
{
    let mut layers = Vec::new();
    {
        let contents = std::fs::read_to_string("src/day8.txt")
            .expect("Should have been able to read the file");
        
        

        for (idx, c) in contents.as_bytes().iter().enumerate()
        {
            if idx % PIXELS_PER_LAYER as usize == 0 {
                layers.push(Layer{zeros: 0, ones: 0, twos: 0});
            }
            let layer = layers.last_mut().unwrap();
            match c {
                b'0' => layer.zeros += 1,
                b'1' => layer.ones += 1,
                b'2' => layer.twos += 1,
                _ => panic!("Error in data!")
            }
        }
    }
    println!("Stuff: {:?}", layers);

    let with_most_zeros = layers.iter().reduce(|best, cur| if cur.zeros < best.zeros {cur} else {best}).unwrap();
    let res = with_most_zeros.ones * with_most_zeros.twos;
    println!("Result: {}", res);
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Pixel {
    Transparent,
    Black,
    White
}

pub fn part_2()
{
    let mut layers: Vec<[[Pixel; WIDTH as usize]; HEIGHT as usize]> = Vec::new();
    {
        let contents = std::fs::read_to_string("src/day8.txt")
            .expect("Should have been able to read the file");

        let mut x = 0;
        let mut y = 0;
        for (idx, c) in contents.as_bytes().iter().enumerate()
        {
            if idx % PIXELS_PER_LAYER as usize == 0 {
                layers.push([[Pixel::Transparent; WIDTH as usize]; HEIGHT as usize]);
                x = 0;
                y = 0;
            }
            let to_change = &mut layers.last_mut().unwrap()[y][x];
            match c {
                b'0' => *to_change = Pixel::Black,
                b'1' => *to_change = Pixel::White,
                b'2' => *to_change = Pixel::Transparent,
                _ => panic!("Error in data!")
            }
            x += 1;
            if x == WIDTH as usize {
                x = 0;
                y += 1;
            }
        }
    }
    println!("layers: {:?}", layers);

    println!("Image:");

    for y in 0..HEIGHT as usize {
        for x in 0..WIDTH as usize {
            let mut pix = Pixel::Transparent;
            for layer in layers.iter() {
                if layer[y][x] != Pixel::Transparent {
                    pix = layer[y][x];
                    break;
                }
            }
            match pix {
                Pixel::Black => print!("⬜"),
                Pixel::White => print!("⬛"),
                _ => panic!("Erroronous pixel")
            }
        }
        println!();
    }
}