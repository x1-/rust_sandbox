/**
 * http://rustbyexample.com/trait/iter.html
 */
struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 1, next: 1 }
}

pub fn take4() {
    println!("\n*---*---*---*---*---*");
    println!("fibonacci().take(4)");
    println!("*---*---*---*---*---*");

    let fib4 = fibonacci().take(4);
    println!("{:?}", fib4.collect::<Vec<_>>());

}

pub fn skip4take10() {
    println!("\n*---*---*---*---*---*");
    println!("fibonacci().skip(4).take(10)");
    println!("*---*---*---*---*---*");

    let fib10 = fibonacci().skip(10).take(10);
    println!("{:?}", fib10.collect::<Vec<_>>());
}
