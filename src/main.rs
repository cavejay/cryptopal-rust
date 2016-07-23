mod bh64;
mod cryptopals;

fn main() {
    let mut s : i32 = 1;
    cryptopals::new_set(&mut s);
    //                    #, expected, result
    cryptopals::challenge(1, "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t", c1());
    cryptopals::challenge(2, "746865206b696420646f6e277420706c6179", c2());
    // cryptopals::challenge(3, "",c3());

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
    bh64::value_to_hex(bh64::xor_hex_arrays(bh64::hex_to_value(xor), bh64::hex_to_value(input)))
}

fn c3() -> String {
    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".to_string();
    let b = bh64::hex_to_value(input);
    // println!()
    "hi".to_string()
}