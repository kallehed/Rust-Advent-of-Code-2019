
use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};

pub fn part_1() {

    let mut codes: Vec<i64>;
    {

        let contents = std::fs::read_to_string("src/day23.txt")
            .expect("Should have been able to read the file");
        
        codes = contents.split(',').map(|num_str| num_str.parse().unwrap()).collect();

        codes.resize(codes.len() + 1000, 0);
    }


    let mut txs = Vec::new();
    let mut rxs = Vec::new();

    for _ in 0..50 {
        let (tx, rx) = channel::<(i64,i64)>();
        txs.push(tx);
        rxs.push(rx);
    }

    let txs: [Sender<(i64, i64)>; 50] = txs.try_into().unwrap();

    thread::scope(|s| {
        for (i, r) in rxs.into_iter().enumerate() {
            let codes = &codes;
            let n_t = txs.clone();
            s.spawn(move || {
                compute(codes.clone(), i as _, n_t, r);
            });
            
        }
    });
}

fn asd(mut arr: [(Sender<i64>, String); 50]) {
    //let first: [_; 50] = std::array::from_fn(|idx| std::mem::take(&mut arr[idx].0));
}

#[derive(Debug)]
struct Kalle {
    i: i32,
}

impl Drop for Kalle {
    fn drop(&mut self) {
        println!("dropped: {}", self.i);
    }
}

pub fn part_3()
{
    let a = [(String::from("asd"),Kalle{i:0}), (String::from("kalle"), Kalle{i:1})];

    let _b = unzip_array_of_tuple(a);
}

fn unzip_array_of_tuple<T1, T2, const N: usize>(arr: [(T1,T2);N]) -> ([T1;N], [T2;N])
{
    use std::mem::{MaybeUninit, self};
    
    let mut first: [MaybeUninit<T1>; N] = unsafe {MaybeUninit::uninit().assume_init()};
    let mut second: [MaybeUninit<T2>; N] = unsafe {MaybeUninit::uninit().assume_init()};

    for (idx, a) in arr.into_iter().enumerate() {
        first[idx] = MaybeUninit::new(a.0);
        second[idx] = MaybeUninit::new(a.1);
    }
    
    // should be safe, as MaybeUninit doesn't have Drop
    unsafe { (mem::transmute_copy(&first), mem::transmute_copy(&second)) }
}

fn compute(mut codes: Vec<i64>, ntwk_addr: i64, txs: [Sender<(i64, i64)>; 50], rx: Receiver<(i64,i64)>)
{
    
    //println!("codes: {:?}", codes);
    
    let mut i: usize = 0;
    let mut relative_base = 0;

    let mut sent_addr = false;

    let mut output = [0,0,0];
    let mut output_idx = 0;

    let mut data_get = (0,0);
    let mut data_get_idx = 0;

    loop 
    {
        let code = codes[i];
        let op_code = code % 100;

        match op_code {
            op if ((1..=9).contains(&op)) =>
            {
                let first_mode = code % 1000 - op_code;
                let sec_mode = code % 10000 - (first_mode + op_code);
                let tri_mode = code % 100000 - (first_mode + op_code + sec_mode);

                // locations of values
                let mut loc0 = i + 1;
                let mut loc1 = i + 2;
                let mut loc2 = i + 3;

                match first_mode {
                    0 => loc0 = codes[loc0] as usize,
                    100 => (),
                    200 => loc0 = (codes[loc0] + relative_base) as usize,
                    _ => panic!("Wrong first mode: {}", code)
                }
                match sec_mode {
                    0 => loc1 = codes[loc1] as usize,
                    1000 => (),
                    2000 => loc1 = (codes[loc1] + relative_base) as usize,
                    _ => panic!("Wrong second mode: {}", code)
                }

                match tri_mode {
                    0 => loc2 = codes[loc2] as usize,
                    10000 => (),
                    20000 => loc2 = (codes[loc2] + relative_base) as usize,
                    _ => panic!("Wrong second mode: {}", code)
                }

                let val0 = codes[loc0];
                let mut val1 = 0;
                if loc1 < codes.len() {
                    val1 = codes[loc1];
                }
                
                match op {
                    1 => { // add
                        codes[loc2] = val0 + val1;
                        i += 4;
                    }
                    2 => { // multiply
                        codes[loc2] = val0 * val1;
                        i += 4;
                    }
                    3 => { // take input 1 and store it at next
                        let inp = if !sent_addr {
                            sent_addr = true;
                            ntwk_addr
                        } else if data_get_idx == 0 {
                            use rand::Rng;
                            match rx.recv_timeout(std::time::Duration::from_millis(10)) {
                                Ok(v) => {
                                    data_get_idx = 1;
                                    data_get = v;
                                    data_get.0
                                }
                                Err(_) => {
                                    -1
                                }
                            }
                        } else {
                            data_get_idx = 0;
                            data_get.1
                        };

                        //println!("Gave input: {}", inp);
                        codes[loc0] = inp;
                        i += 2;

                       //std::thread::sleep(std::time::Duration::from_secs(1));
                    }
                    4 => { // output value
                        
                        let out = codes[loc0];
                        println!("Output: {}", out);
                        output[output_idx] = out;
                        i += 2;

                        output_idx += 1;
                        if output_idx == 3 {
                            output_idx = 0;
                            let wher = output[0];
                            let data = (output[1], output[2]);
                            if wher == 255 {
                                println!("X and Y of sent to 255: {:?}", data);
                            } else {
                                txs[wher as usize].send(data).unwrap();
                            }
                        }
                    }
                    5 => { // jump if true
                        if val0 != 0 {
                            i = val1 as usize;
                        } else {
                            i += 3;
                        }
                    }
                    6 => { // jump if false
                        if val0 == 0 {
                            i = val1 as usize;
                        } else {
                            i += 3;
                        }
                    }
                    7 => { // less than
                        codes[loc2] = (val0 < val1) as i64;
                        i += 4;
                    }
                    8 => { // equal
                        codes[loc2] = (val0 == val1) as i64;
                        i += 4;
                    }
                    9 => { // increase relative base by first argument
                        relative_base += val0;
                        i += 2;
                    }
                    _ => {
                        panic!("very invalid: {}", code);
                    }
                }
            }
            
            99 => {
                break;
            }
            _ => {
                panic!("Wrong instruction: {}", code);
            }
        }
    }
}

pub fn part_2() {

    let mut codes: Vec<i64>;
    {

        let contents = std::fs::read_to_string("src/day23.txt")
            .expect("Should have been able to read the file");
        
        codes = contents.split(',').map(|num_str| num_str.parse().unwrap()).collect();

        codes.resize(codes.len() + 1000, 0);
    }


    let mut txs = Vec::new();
    let mut rxs = Vec::new();

    for _ in 0..50 {
        let (tx, rx) = channel::<(i64,i64)>();
        txs.push(tx);
        rxs.push(rx);
    }

    let txs: [Sender<(i64, i64)>; 50] = txs.try_into().unwrap();

    let (nat_tx, nat_rx) = channel::<(i64, i64)>();

    thread::scope(|s| {
        for (i, r) in rxs.into_iter().enumerate() {
            let codes = &codes;
            let n_t = txs.clone();
            let nat_tx = nat_tx.clone();
            s.spawn(move || {
                compute2(codes.clone(), i as _, n_t, r, nat_tx);
            });
            
        }
        // NAT
        let mut val = (-69,420);
        let mut prev_sent = (-59,520);
        thread::sleep(std::time::Duration::from_millis(200));
        loop
        {
            thread::sleep(std::time::Duration::from_millis(100)); // wait for all the threads to get idle (yes, very lazy)
            while let Ok(v) = nat_rx.try_recv() { // get the absolute last message
                val = v;
            }
            //println!("y val: {}", n.1);
            if val.1 == prev_sent.1 {
                println!("DOUBLE: {:?}", val.1);
                break;
            }
            prev_sent = val;
            txs[0].send(val).unwrap();

        }
    });
}

fn compute2(mut codes: Vec<i64>, ntwk_addr: i64, txs: [Sender<(i64, i64)>; 50], rx: Receiver<(i64,i64)>, nat_tx: Sender<(i64,i64)>)
{
    
    //println!("codes: {:?}", codes);
    
    let mut i: usize = 0;
    let mut relative_base = 0;

    let mut sent_addr = false;

    let mut output = [0,0,0];
    let mut output_idx = 0;

    let mut data_get = (0,0);
    let mut data_get_idx = 0;

    loop 
    {
        let code = codes[i];
        let op_code = code % 100;

        match op_code {
            op if ((1..=9).contains(&op)) =>
            {
                let first_mode = code % 1000 - op_code;
                let sec_mode = code % 10000 - (first_mode + op_code);
                let tri_mode = code % 100000 - (first_mode + op_code + sec_mode);

                // locations of values
                let mut loc0 = i + 1;
                let mut loc1 = i + 2;
                let mut loc2 = i + 3;

                match first_mode {
                    0 => loc0 = codes[loc0] as usize,
                    100 => (),
                    200 => loc0 = (codes[loc0] + relative_base) as usize,
                    _ => panic!("Wrong first mode: {}", code)
                }
                match sec_mode {
                    0 => loc1 = codes[loc1] as usize,
                    1000 => (),
                    2000 => loc1 = (codes[loc1] + relative_base) as usize,
                    _ => panic!("Wrong second mode: {}", code)
                }

                match tri_mode {
                    0 => loc2 = codes[loc2] as usize,
                    10000 => (),
                    20000 => loc2 = (codes[loc2] + relative_base) as usize,
                    _ => panic!("Wrong second mode: {}", code)
                }

                let val0 = codes[loc0];
                let mut val1 = 0;
                if loc1 < codes.len() {
                    val1 = codes[loc1];
                }
                
                match op {
                    1 => { // add
                        codes[loc2] = val0 + val1;
                        i += 4;
                    }
                    2 => { // multiply
                        codes[loc2] = val0 * val1;
                        i += 4;
                    }
                    3 => { // take input 1 and store it at next
                        let inp = if !sent_addr {
                            sent_addr = true;
                            ntwk_addr
                        } else if data_get_idx == 0 {
                            use rand::Rng;
                            match rx.try_recv() {
                                Ok(v) => {
                                    data_get_idx = 1;
                                    data_get = v;
                                    data_get.0
                                }
                                Err(_) => {
                                    std::thread::sleep(std::time::Duration::from_millis(1));
                                    -1
                                }
                            }
                        } else {
                            data_get_idx = 0;
                            data_get.1
                        };

                        //println!("Gave input: {}", inp);
                        codes[loc0] = inp;
                        i += 2;

                       //std::thread::sleep(std::time::Duration::from_secs(1));
                    }
                    4 => { // output value
                        
                        let out = codes[loc0];
                        //println!("Output: {}", out);
                        output[output_idx] = out;
                        i += 2;

                        output_idx += 1;
                        if output_idx == 3 {
                            output_idx = 0;
                            let wher = output[0];
                            let data = (output[1], output[2]);
                            if wher == 255 {
                                //println!("X and Y of sent to 255: {:?}", data);
                                nat_tx.send(data).unwrap();
                            } else {
                                txs[wher as usize].send(data).unwrap();
                            }
                        }
                    }
                    5 => { // jump if true
                        if val0 != 0 {
                            i = val1 as usize;
                        } else {
                            i += 3;
                        }
                    }
                    6 => { // jump if false
                        if val0 == 0 {
                            i = val1 as usize;
                        } else {
                            i += 3;
                        }
                    }
                    7 => { // less than
                        codes[loc2] = (val0 < val1) as i64;
                        i += 4;
                    }
                    8 => { // equal
                        codes[loc2] = (val0 == val1) as i64;
                        i += 4;
                    }
                    9 => { // increase relative base by first argument
                        relative_base += val0;
                        i += 2;
                    }
                    _ => {
                        panic!("very invalid: {}", code);
                    }
                }
            }
            
            99 => {
                break;
            }
            _ => {
                panic!("Wrong instruction: {}", code);
            }
        }
    }
}