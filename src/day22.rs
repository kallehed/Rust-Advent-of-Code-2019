
pub fn part_1()
{

    enum Instr {
        DealWithIncrement(u8),
        DealNewStack,
        Cut(i16),
    }

    let instrs = {
        let mut instrs = Vec::new();
        let s = std::include_str!("day22.txt");
        for line in s.lines() {

            if let Some(num) = line.strip_prefix("deal with increment ") {
                instrs.push(Instr::DealWithIncrement(num.parse().unwrap()));
            }
            else if line.cmp("deal into new stack") == std::cmp::Ordering::Equal {
                instrs.push(Instr::DealNewStack);
            }
            else if let Some(num) = line.strip_prefix("cut ") {
                instrs.push(Instr::Cut(num.parse().unwrap()));
            }
        }
        instrs
    };

    const LEN: usize = 10007;
    type Card = u16;

    let mut deck: [Card; LEN] = std::array::from_fn::<_, LEN, _>(|i| i.try_into().unwrap());

    for instr in instrs {
        match instr {
            Instr::DealNewStack => {
                deck.reverse();
            }
            Instr::DealWithIncrement(n) => {
                let copy = deck;
                // from copy, deal inside deck
                let mut index: Card = 0;
                for item in copy {
                    deck[index as usize] = item;

                    index += <u8 as Into<Card>>::into(n);
                    if index >= (LEN as _) {
                        index -= LEN as Card;
                    }
                }
            }
            Instr::Cut(n) => {
                let mut rot = (n as isize) % (LEN as isize);
                if rot < 0 {
                    rot += LEN as isize;    
                }
                deck.rotate_left(rot as _);
            }
        }
    }
    println!("final: {:?}", deck);
    println!("position: {:?}", deck.iter().position(|&k| k == 2019).unwrap());

}

pub fn part_2()
{
    enum Instr {
        DealWithIncrement(u8),
        DealNewStack,
        Cut(i16),
    }

    let instrs = {
        let mut instrs = Vec::new();
        let s = std::include_str!("day22.txt");
        for line in s.lines() {

            if let Some(num) = line.strip_prefix("deal with increment ") {
                instrs.push(Instr::DealWithIncrement(num.parse().unwrap()));
            }
            else if line.cmp("deal into new stack") == std::cmp::Ordering::Equal {
                instrs.push(Instr::DealNewStack);
            }
            else if let Some(num) = line.strip_prefix("cut ") {
                instrs.push(Instr::Cut(num.parse().unwrap()));
            }
        }
        instrs
    };

    const LEN: usize = 2021;
    type Card = u16;

    let mut deck: [Card; LEN] = std::array::from_fn::<_, LEN, _>(|i| i.try_into().unwrap());

    for instr in instrs {
        match instr {
            Instr::DealNewStack => {
                deck.reverse();
            }
            Instr::DealWithIncrement(n) => {
                let copy = deck;
                // from copy, deal inside deck
                let mut index: Card = 0;
                for item in copy {
                    deck[index as usize] = item;

                    index += <u8 as Into<Card>>::into(n);
                    if index >= (LEN as _) {
                        index -= LEN as Card;
                    }
                }
            }
            Instr::Cut(n) => {
                let mut rot = (n as isize) % (LEN as isize);
                if rot < 0 {
                    rot += LEN as isize;    
                }
                deck.rotate_left(rot as _);
            }
        }
    }
    println!("final: {:?}", deck);
    println!("position 2020: {:?}", deck[2020]);

}