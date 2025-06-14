fn test(x: i32) -> i32 {
    return x + 1;
}
fn  main() {
    let y = {
        1 + 1
    };
    println!("{y}");
    println!("{}", test(5));
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let a = [5; 10];

    let mut sum = 0;

    print!("{:?}\n", a);
    for x in a {
        println!("{x}");

        sum += x;

    }

    println!("{sum}");

    let mut x = 0;

    'a: loop {
        x += 1;
        'b: loop {
            if x > 10 {
                break 'b;
            } else {
                println!("x = {x}");
                continue 'a;
            }
        }
        break;       

    }
    println!("x = {x}");
}