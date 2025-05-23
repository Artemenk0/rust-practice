#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rectangle {
    a: Point,
    b: Point,
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle { a: Point { x: 2, y: 9 }, b: Point { x: 5, y: 3 } },  
        Rectangle { a: Point { x: 1, y: 8 }, b: Point { x: 11, y: 6 } }, 
        Rectangle { a: Point { x: 9, y: 10 }, b: Point { x: 13, y: 2 } }, 
    ]
}

fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut grid = vec![vec![false; 100]; 100]; 
    for rect in xs {
        let x_start = rect.a.x.min(rect.b.x);
        let x_end = rect.a.x.max(rect.b.x);
        let y_start = rect.a.y.min(rect.b.y);
        let y_end = rect.a.y.max(rect.b.y);

        for x in x_start..x_end {
            for y in y_start..y_end {
                grid[x as usize][y as usize] = true;
            }
        }
    }

    let mut count = 0;
    for x in 0..100 {
        for y in 0..100 {
            if grid[x][y] {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let data = test_data();
    let occupied = area_occupied(&data);
    println!("Площа: {}", occupied);
}
