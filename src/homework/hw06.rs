fn main() 
{
    CreateTree(5); 
}

fn CreateTree(triangleCount: i32) 
{
    for i in 1..=triangleCount + 1
    {
        ShowTriangle(i * 2 - 1); 
    }
}

fn ShowTriangle(maxCount: i32) 
{
    let maxWidth = maxCount;

    for count in (1..=maxCount).step_by(2) 
    {
        let spaces = (maxWidth - count) / 2;

        for _ in 0..spaces 
        {
            print!(" ");
        }

        for _ in 0..count 
        {
            print!("*");
        }

        println!();
    }
}