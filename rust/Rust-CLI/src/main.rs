fn main() {
    let ans = test(299);
    println!("{}", ans);

    fn test(x: i32) -> i32 {
        let ans = if x > 100 {
            100
        } else if x < 0 {
            0
        } else {
            x
        };
        ans
    }
    let a = Sample::new(3);
    println!("{}", a.x);
    let ans = a.inc();
    println!("{}", ans);
    let ans = a.add(10);
    println!("{}", ans);
    let add = |x, y| x + y;
    let num = 10;
    let add_num = |x| x + num;
    println!("added num = {}", add_num(8));
    println!("added = {}", add(3, 7));
}

struct Sample {
    x: i32,
}

impl Sample {
    fn new(x: i32) -> Sample {
        Sample { x: x }
    }
    fn inc(&self) -> i32 {
        self.x + 1
    }
    fn add(&self, x: i32) -> i32 {
        &self.x + x
    }
}
