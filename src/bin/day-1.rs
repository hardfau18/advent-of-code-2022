use std::io::BufRead;

fn get_max(input: &[Vec<usize>]) -> Vec<usize> {
    let mut sums: Vec<usize> = input.iter().map(|cals| cals.iter().sum()).collect();
    sums.sort_by(|a, b| b.partial_cmp(a).unwrap());
    sums
}

fn main() {
    let mut buf = String::new();
    let mut inp = Vec::new();
    let mut stdin = std::io::stdin().lock();
    let mut cur_buf = Vec::new();
    while let Ok(n) = stdin.read_line(&mut buf) {
        if n == 0 {
            break;
        }
        if let Ok(num) = buf.trim().parse() {
            cur_buf.push(num);
        } else {
            inp.push(cur_buf);
            cur_buf = Vec::new();
        }
        buf.clear();
    }
    inp.push(cur_buf);
    let cals = get_max(&inp);
    println!(
        "Max amount of Calories available is {}, top 3 {}",
        cals[0],
        &cals[0..3].iter().sum::<usize>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_elems() {
        assert_eq!(
            get_max(&[vec![1, 2, 3], vec![4, 5], vec![8], vec![6, 3]]),
            vec![9, 9, 8, 6]
        );
        assert_eq!(
            get_max(&[
                vec![1000, 2000, 3000,],
                vec![4000],
                vec![5000, 6000,],
                vec![7000, 8000, 9000,],
                vec![10000]
            ]),
            vec![24000, 11000, 10000, 6000, 4000]
        )
    }
    #[test]
    fn empty() {
        assert_eq!(get_max(&[vec![], vec![]]), vec![0, 0]);
    }
}
