fn is_prime(n: &u32) -> bool {
    if *n < 2 {
        return false;
    }

    let mut i = 2;
    while i * i <= *n {
        if *n % i == 0 {
            return false;
        }
        i += 1;
    }

    true
}

fn main() {
    let test_data = [
        (0, false),
        (1, false),
        (2, true),
        (3, true),
        (4, false),
        (5, true),
        (100, false),
        (10007, true),
    ];

    for (n, expected) in test_data.iter() {
        let result = is_prime(n);
        println!(
            "is_prime({}) = {}, result = {}, {}",
            n,
            result,
            expected,
            if result == *expected { "+" } else { "-" }
        );
    }
}

