mod bh64;
mod cryptopals;

fn main() {
    let mut s : i32 = 1;
    cryptopals::new_set(&mut s);
    //                    #, expected, result
    cryptopals::challenge(1, "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t", c1());
    // cryptopals::challenge(2, "746865206b696420646f6e277420706c6179", c2());
    c2();
    // cryptopals::new_set(&mut s);
}

// Hex to 64
fn c1() -> String {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string();
    bh64::hex_to_base64(input)
}

fn c2() {
    let input = "1c0111001f010100061a024b53535009181c".to_string();
    let xor = "686974207468652062756c6c277320657965".to_string();

    
    println!("{:?}", bh64::hex_to_binary(input));
}
