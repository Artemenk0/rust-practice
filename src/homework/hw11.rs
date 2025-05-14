use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let vec: Vec<i32> = (0..20)
        .map(|_| rng.gen_range(10..99))
        .collect();

    for i in 0..20 {
        if i < 10 {
            print!(" 0{} ", i);
        } else {
            print!(" {} ", i);
        }
    }
    println!();
    println!("{:?}", vec);
    
    result(&vec); 
}

fn showIndex(index: i32) 
{
    for i in 0..20
    {
        if i == index
        {
            println!(" \\_____/");
            return;
        }
        
        print!("    ");
    }
    
}

fn result(rnd: &Vec<i32>) 
{
    let mut min_value = rnd[0] + rnd[1];
    
    let mut minNumber1 = rnd[0];
    let mut minNumber2 = rnd[1];
    
    let mut minIndex1: usize = 0;
    let mut minIndex2: usize = 1;

    for n in 0..rnd.len() - 1 
    {
        let sum = rnd[n] + rnd[n + 1];
        if sum < min_value 
        {
            min_value = sum;
            
            minNumber1 = rnd[n];
            minNumber2 = rnd[n + 1];
            
            minIndex1 = n;
            minIndex2 = n + 1;
        }
    }
    
    showIndex(minIndex1 as i32); 
    println!("{} + {} = {} indexes: {}, {}", minNumber1, minNumber2, min_value, minIndex1, minIndex2);
}