mod bh64;
mod cryptopals;
mod crypt;

fn main() {
    let mut s : i32 = 1;
    cryptopals::new_set(&mut s);
    //                    #, expected, result
    cryptopals::challenge(1, "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t", c1());
    cryptopals::challenge(2, "746865206b696420646f6e277420706c6179", c2());
    cryptopals::challenge(3, "",c3());

    // cryptospals::new_set(&mut s);
}

// Hex to 64
fn c1() -> String {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string();
    bh64::hex_to_base64(input)
}

fn c2() -> String {
    let input = "1c0111001f010100061a024b53535009181c".to_string();
    let xor = "686974207468652062756c6c277320657965".to_string();
    bh64::value_to_hex(&bh64::xor_hex_arrays(&bh64::hex_to_value(&xor), &bh64::hex_to_value(&input)))
}

fn c3() -> String {
    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".to_string();
    let base64_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let common: char = crypt::most_common_char(&input);

    for c in 0x20..0x7F {
        
        let x = common as u8;
        let cc = c as u8;
        println!("Checking {:x} against {:x}", x^cc, 'e' as u8);
        if x ^ cc == ('e' as u8) {
            println!("matching on 'e' the key is: {}", c);
            break;
        } 
        println!("Checking {:x} against {:x}", x^cc, 't' as u8);
        if x ^ cc == ('t' as u8) {
            println!("matching on 't' the key is: {}", c);
            break;
        }
        println!("Checking {:x} against {:x}", x^cc, 'a' as u8);
        if x ^ cc == ('a' as u8) {
            println!("matching on 'a' the key is: {}", c);
            break;
        }
        println!("Checking {:x} against {:x}", x^cc, 'o' as u8);
        if x ^ cc == ('o' as u8) {
            println!("matching on 'o' the key is: {}", c);
            break;
        }
        println!("Checking {:x} against {:x}", x^cc, 'i' as u8);
        if x ^ cc == ('i' as u8) {
            println!("matching on 'i' the key is: {}", c);
            break;
        }
        println!("Checking {:x} against {:x}", x^cc, 'n' as u8);
        if x ^ cc == ('n' as u8) {
            println!("matching on 'n' the key is: {}", c);
            break;
        }
    }

    println!("most common char: {}", common);
    "hi".to_string()
}
