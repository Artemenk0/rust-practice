use std::io;

fn main()
{
    println!("Введіть число:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Помилка читання");

    let size: i32 = input.trim().parse().expect("Помилка перетворення числа");

    print_convert(size);
}

fn print_convert(size: i32)
{
    let h: i32 = size;
    let w: i32 = size;

    for y in 0.. h
    {
        for x in 0..w
        {

            if x == y || x + y == size - 1 /* border- */|| x == 0 || y == 0 || x == size - 1 || y == size - 1
            {
                print!("*");
            }
            else
            {
                print!(" ");
            }
        }

        println!();
    }

}
