

fn main() 
{

    let test1 = vec![1, 1, 1, 1, 6];
    let test2 = vec![9, 3, 7, 2, 9];
    let test3 = vec![8, 2, 2, 4, 4];
    let test4 = vec![1, 1, 1, 1, 6];

    println!("Test 1: {}", minimum_moves(test1)); // 4
    println!("Test 2: {}", minimum_moves(test2)); // 7
    println!("Test 3: {}", minimum_moves(test3)); // 4
    println!("Test 4: {}", minimum_moves(test4)); // 4
}

fn minimum_moves(weights: Vec<i32>) -> i32 
{
    let n = weights.len();
    let total: i32 = weights.iter().sum();

    if total % n as i32 != 0 
    {
        return -1;
    }

    let average = total / n as i32;
    let mut moves = 0;

    for &w in &weights 
    {
        if w > average 
        {
            moves += w - average;
        }
    }

    moves
}