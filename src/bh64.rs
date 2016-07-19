///
/// # Contains all the functions used for manipulating hex and base64
///

fn hex_char_to_val(c: char) -> usize {
    let c2 = c.to_uppercase().collect::<Vec<_>>()[0];
    let hex_val = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];
    hex_val.iter().position(|&r| r == c2).unwrap()
}

fn val_to_hex_char(v: usize) -> char {
    let hex_val = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];
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
    let char2 = val_to_base64_char(x - ((x >> 6) << 6));
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
pub fn hex_to_binary(s: String) -> Vec<usize> {
    let mut n = vec![0; 1];
    let mut four = 0;
    let mut i = 0;
    let mut s_itr = s.chars();
    let mut cur = s_itr.next().unwrap();
    while true {
        match s_itr.next() {
            Some(j) => {
                n[i] = n[i] + hex_char_to_val(j);
                n[i] = n[i] << 4;
                four += 1;
                if four == 4 {
                    four = 0;
                    n.push(0);
                    i += 1;
                }
            }, 
            None => { break }
        }
    }
    n
}
