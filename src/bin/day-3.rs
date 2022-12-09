fn batch_priorities(inp: &str) -> usize {
    let mut lines = inp.lines();
    let mut ret = 0;
    while let Some(line) = lines.next() {
        if let Some(l1) = lines.next() {
            if let Some(l2) = lines.next() {
                for chr in line.chars() {
                    if l1.contains(chr) && l2.contains(chr) {
                        ret += if chr.is_lowercase() {
                            chr as u32 - 'a' as u32 + 1
                        } else {
                            chr as u32 - 'A' as u32 + 27
                        };
                        break;
                    }
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }
    ret as usize
}
fn total_priorities(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (first, second) = line.split_at(line.len() / 2);
            let mut ret = 0;
            for chr in first.chars() {
                if second.contains(chr) {
                    ret = if chr.is_lowercase() {
                        chr as u32 - 'a' as u32 + 1
                    } else {
                        chr as u32 - 'A' as u32 + 27
                    };
                    break;
                }
            }
            ret as usize
        })
        .sum::<usize>()
}

fn main() {
    let data = std::fs::read_to_string("data/day3.txt").unwrap();
    println!("Total priorities {}", total_priorities(&data));
    println!("Batch priorities {}", batch_priorities(&data));
}

#[cfg(test)]
mod tests {
    use crate::{batch_priorities, total_priorities};

    #[test]
    fn sample() {
        let sample_data = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(total_priorities(sample_data), 157);
        assert_eq!(batch_priorities(sample_data), 70);
    }
}
