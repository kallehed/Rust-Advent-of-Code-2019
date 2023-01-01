use std::collections::HashSet;
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread::{self, JoinHandle};

fn calc_intcode(mut codes: Vec<i32>, first_input: i32, tx: Sender<i32>, rx: Receiver<i32>, result_tx: Sender<i32>)
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
            3 => { // take input and store it at next
                let pos = codes[i + 1];
                //println!("want input");
                if inputs_given == 0 {
                    codes[pos as usize] = first_input;
                } else {
                    codes[pos as usize] = rx.recv().unwrap();
                }
                
                inputs_given += 1;
                i += 2;
            }
            4 => { // output value at position
                let pos = codes[i + 1];
                //println!("Output: {}", codes[pos as usize]);
                //println!("Sent!");
                tx.send(codes[pos as usize]); // should not do anything if this fails.
                result_tx.send(codes[pos as usize]).unwrap();
                i += 2;
            }
            99 => {
                //println!("Done");
                break;
            }
            _ => {
                panic!("Wrong instruction: {}", code);
            }
        }
    }
}

fn thruster_signal(codes: &[i32], phase_setting: [i32; 5]) -> i32
{
    let (result_tx, result_rx) = mpsc::channel();

    let mpscs = [(); 5].map(|_| mpsc::channel());

    let (txs, rxs): (Vec<_>, Vec<_>) = mpscs.into_iter().unzip();

    txs[0].send(0).unwrap();

    let mut handles = vec![];

    for (i, rx) in rxs.into_iter().enumerate()
    {
        let c = codes.to_owned();
        let inp = phase_setting[i];
        let res_tx = result_tx.clone();
        let tx = txs[(i+1)%5].clone();
        handles.push(thread::spawn( move || {
            calc_intcode(c, inp, tx, rx, res_tx);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
    drop(result_tx);

    result_rx.iter().last().unwrap()
    
}


pub fn part_2()
{
    let mut codes: Vec<i32>;
    {

        let contents = std::fs::read_to_string("src/day7.txt")
            .expect("Should have been able to read the file");
        
        codes = contents.split(',').map(|num_str| num_str.parse().unwrap()).collect();
    }
    println!("codes: {:?}", codes);

    let mut biggest = 0;

    for n0 in 5..=9 {
        for n1 in 5..=9 {
            for n2 in 5..=9 {
                for n3 in 5..=9 {
                    for n4 in 5..=9 {
                        let setting = [n0,n1,n2,n3,n4];
                        let unique = {
                            let mut h = HashSet::new();
                            for sett in setting {h.insert(sett);}
                            h.len() == 5
                        };
                        if unique
                        {
                            let sig = thruster_signal(&codes, setting);
                            if sig > biggest  {
                                biggest = sig;
                                println!("val: {}, matric: {:?}", biggest, setting);
                            }
                        }
                    }
                }
            }
        }
    }
    //println!("val: {}", thruster_signal(&codes, [9,8,7,6,5]));

    println!("signal: {}", biggest);
    


}