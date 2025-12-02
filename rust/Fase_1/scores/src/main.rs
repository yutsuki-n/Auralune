use std::cmp::Ordering;

fn main() {
    let scores = vec![72, 85, 90, 55, 100, 67, 88, 92, 45, 79];

    let mut sum = 0;
    let mut max = 0;
    let mut min = 100;
    let mut counter = 0;

    for score in scores {
        counter += 1;
        sum += score; //&と*はGoみたいに扱う感じ

        //もっとシンプルにできる
        // match score.cmp(&max) {
        //     Ordering::Greater => max = score,
        //     // Ordering::Equal => continue,
        //     // Ordering::Less => continue,
        //     _ => {}
        // }

        // match score.cmp(min) {
        //     Ordering::Greater => continue,
        //     Ordering::Equal => continue,
        //     Ordering::Less => min = score,
        // }

        if score > max {
            max = score;
        }
        if score < min {
            min = score;
        }
    }
    let avg = sum as f64 / counter as f64;
    println!("{}, {}, {}", avg, max, min);
}
