
fn main() 
{
    let data = [
        (123, false),
        (121, true),
        (1221, true),
    ];

    for (n, expected) in data 
    {
        let result = is_palindrome(n);
        println!("is_palindrome({}) = {}", n, result);
    }
}

fn is_palindrome(x: u32) -> bool 
{
    let s = x.to_string(); 
    let reversed: String = s.chars().rev().collect(); 
    s == reversed 
}



