pub fn challenge(num: i32, expected: &str, ans: String) {
    println!("C-{}", num);
    println!("Expecting: {}", expected);
    println!("Answer:    {}", ans);
    if ans.as_str() == expected {
        println!("Challenge Passed!");
    } else {
        println!("FAILED");
    }
}

pub fn new_set(s : &mut i32) {
    println!("\n\n### Set-{}", *s);
    *s += 1;
}
