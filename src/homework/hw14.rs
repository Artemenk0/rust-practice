// Можливо, я не вірно зрозумів умову задачі. Якщо це так, вкажіть будьласка про це в коментарях

fn gray(n: u8) -> Vec<String> {
    if n == 0 {
        return vec![String::new()];
    }

    let prev = gray(n - 1);
    let mut result = Vec::new();

    for code in &prev {
        result.push(format!("0{}", code));
    }

    for code in prev.iter().rev() {
        result.push(format!("1{}", code));
    }

    result
}

fn main() {
    for n in 0..=4 {
        let codes = gray(n);
        println!("Gray code for n = {}:", n);
        for code in codes {
            println!("{}", code);
        }
        println!();
    }
}

fn test() {
    let test_data = [
        (0, vec!["".to_string()]),
        (1, vec!["0".to_string(), "1".to_string()]),
        (2, vec!["00", "01", "11", "10"].iter().map(|s| s.to_string()).collect()),
        (3, vec![
            "000", "001", "011", "010", 
            "110", "111", "101", "100"
        ].iter().map(|s| s.to_string()).collect()),
        (4, vec![
            "0000", "0001", "0011", "0010", 
            "0110", "0111", "0101", "0100",
            "1100", "1101", "1111", "1110",
            "1010", "1011", "1001", "1000"
        ].iter().map(|s| s.to_string()).collect()),
    ];

    test_data.iter().for_each(|(n, out)| {
        assert_eq!(gray(*n), *out);
    });
}
