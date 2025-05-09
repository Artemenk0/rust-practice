

fn rotate(s: String, n: isize) -> String 
{
    let len = s.len() as isize;

    // якщо рядок пустий, просто повертаємо його
    if len == 0 
    {
        return s;
    }

    // нормалізуємо зсув, щоб не виходив за межі довжини
    let mut shift = n % len;
    if shift < 0 
    {
        shift += len;
    }

    let shift = shift as usize;

    // перетворюємо рядок у список символів
    let chars: Vec<char> = s.chars().collect();

    // розбиваємо на дві частини і міняємо місцями
    let mut result = String::new();
    for i in shift..chars.len() 
    {
        result.push(chars[i]);
    }
    for i in 0..shift 
    {
        result.push(chars[i]);
    }

    result
}

fn main() 
{
    let s = String::from("abcdefgh");
    let shifts = [
        (0,  "abcdefgh"),
        (8,  "abcdefgh"),
        (-8, "abcdefgh"),
        (1,  "habcdefg"),
        (2,  "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10,"cdefghab"),
    ];

    for (n, expected) in shifts 
    {
        let result = rotate(s.clone(), n);
        println!(
            "rotate({}, {}) = {} => {}",
            s,
            n,
            result,
            if result == expected { "OK" } else { "FAIL" }
        );
    }
}
