fn main() {
    let digits = [1, 2, 3, 4, 5, 6, 7, 8];
    let mut count = 0;

    let permutations = permute(digits.to_vec());

    for perm in permutations {
        let m = perm[0];
        let u = perm[1];
        let x = perm[2];
        let a = perm[3];
        let s = perm[4];
        let l = perm[5];
        let o = perm[6];
        let n = perm[7];

        let muxa = 1000 * m + 100 * u + 10 * x + a;
        let slon = 1000 * s + 100 * l + 10 * o + n;

        if muxa * a == slon {
            count += 1;

            // Виводимо у форматі, як у задачі:
            println!("{:>6}", muxa);
            println!("x{:>5}", a);
            println!("------");
            println!("{:>6}", slon);
            println!();
        }
    }

    println!("Загальна кількість рішень: {}", count);
}

// Створюємо всі перестановки вручну (рекурсія)
fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut results = Vec::new();
    backtrack(0, &mut nums.clone(), &mut results);
    results
}

fn backtrack(start: usize, nums: &mut Vec<i32>, results: &mut Vec<Vec<i32>>) {
    if start == nums.len() {
        results.push(nums.clone());
        return;
    }
    for i in start..nums.len() {
        nums.swap(start, i);
        backtrack(start + 1, nums, results);
        nums.swap(start, i);
    }
}
