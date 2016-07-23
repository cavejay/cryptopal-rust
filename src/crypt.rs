use bh64;

pub fn most_common_char(s: &String) -> char {
    // First find most common char
    let mut mapC = vec![0; 1];
    let mut mapN = vec![0; 1];
    let mut best = (0, 0);
    let mut itr = (*s).chars();
    loop {
        match itr.next() {
            Some(x1) => {
                let x2 = itr.next().unwrap();
                let x = (bh64::hex_char_to_val(x1) << 4) + bh64::hex_char_to_val(x2);
                // println!("{}{} -> {:x} -> {} -> {:b}{:b}", x1,x2,x,x,bh64::hex_char_to_val(x1),bh64::hex_char_to_val(x2));
                if mapC.contains(&x) {
                    let index = mapC.iter().position(|&r| r == x).unwrap();
                    mapN[index] += 1;

                    if mapN[index] > best.1 {
                        best = (mapC[index], mapN[index]);
                    }
                } else {
                    mapC.push(x);
                    mapN.push(1);
                }
            }, 
            None => break,
        }
    }

    println!("{:x}, {:?}", best.0, best.1);
    best.0 as u8 as char
}