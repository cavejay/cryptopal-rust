///
/// # Contains all the functions used for manipulating hex and base64
///

fn hex_char_to_val(c: char) -> usize {
    let c2 = c.to_uppercase().collect::<Vec<_>>()[0];
    let hex_val = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];
    hex_val.iter().position(|&r| r == c2).unwrap()
}

fn val_to_hex_char(v: usize) -> char {
    let hex_val = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];
    hex_val[v]
}

fn base64_char_to_val(c: char) -> usize {
    let base64_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    base64_chars.find(c).unwrap()
}

fn val_to_base64_char(v: usize) -> char {
    let base64_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    base64_chars.chars().nth(v).unwrap()
}

fn hex3_b642(a: char, b: char, c: char) -> (char, char) {
    let mut x = 0;
    x += hex_char_to_val(a);
    x = x << 4;
    x += hex_char_to_val(b);
    x = x << 4;
    x += hex_char_to_val(c);

    let char1 = val_to_base64_char(x >> 6);
    let char2 = val_to_base64_char(x & 0x3F);
    (char1, char2)
}

pub fn hex_to_base64(s: String) -> String {
    let mut sol = String::new();
    let mut s = s.clone();
    let mut s_itr = s.chars();
    loop {
        match s_itr.next() {
            Some(x) => {
                let out = hex3_b642(x, s_itr.next().unwrap(), s_itr.next().unwrap());
                sol.push(out.0); sol.push(out.1);
            }, None => {
                break;
            } 
        }
        // println!("{}", sol);
    }
    sol
}

// produces an array of 16bit binary numbers that when put together create a single binary number 
pub fn hex_to_value(s: String) -> Vec<usize> {
    let mut n = vec![0; 1];
    let mut i = 0;
    let mut s_itr = s.chars();
    loop {
        match s_itr.next() {
            Some(j) => {
                println!("{}",j);
                n[i] = n[i] + hex_char_to_val(j);
                n.push(0);
                i += 1;
            }, 
            None => { break }
        }
    }
    n
}

pub fn xor_hex_arrays(vec1: Vec<usize>, vec2: Vec<usize>) -> Vec<usize> {
    let mut xor = vec![0; vec1.len()];
    for (i, part) in vec1.iter().enumerate() {
        xor[i] = *part ^ vec2[i];
        // println!("{:b} = {:b} ^ {:b}", xor[i], *part, vec2[i]);
    }
    xor
}

pub fn value_to_hex(v: Vec<usize>) -> String {
    let mut ans = String::new();
    for part in v.iter() {
        ans.push(val_to_hex_char(*part));
    }
    ans.pop();
    ans
}