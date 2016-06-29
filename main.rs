fn challenge(num: i32, expected: &str, ans: String) {
    println!("C-{}", num);
    println!("Expecting: {}", expected);
    println!("Answer: {}", ans);
    if ans.as_str() == expected {
        println!("Challenge Passed!");
    } else {
        println!("FAILED");
    }
}

fn new_set(s: i32) -> i32 {
    println!("\n\n### Set-{}", s);
    return s+1;
}

fn main() {
    let mut s = 1;
    s = new_set(s);
    challenge(1, "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t", c1());
    new_set(s);
}

fn c1() -> String {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let mut out = String::new();
    out.push('l');

    return out;
}
