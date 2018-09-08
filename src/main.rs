fn main() {
    loop {
            println!("Hello, world!");
            break;
    }

    let mut n = 3;

    while n > 0 {
        println!("{}", n);
        n = n - 1;
    }

    let a = [1,2,3,4,5,6,7,8];

    for elem in a.iter() {
        println!("elem={}", elem);
    }
}
