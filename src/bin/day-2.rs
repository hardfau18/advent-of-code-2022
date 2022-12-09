use std::io::BufRead;
#[derive(Debug, PartialEq)]

enum Choice {
    Rock,
    Paper,
    Scizzor,
}

/// results of an rock,paper,Scizzor game
enum Res {
    Win,
    Lose,
    Draw,
}

/// Takes opp choice and players choice and calculates points according to whats given in question
fn calc_point(choices: (Choice, Choice)) -> usize {
    match choices {
        (Choice::Rock, Choice::Paper) => 8,
        (Choice::Paper, Choice::Scizzor) => 9,
        (Choice::Scizzor, Choice::Rock) => 7,
        (Choice::Rock, Choice::Rock) => 4,
        (Choice::Paper, Choice::Paper) => 5,
        (Choice::Scizzor, Choice::Scizzor) => 6,
        (Choice::Rock, Choice::Scizzor) => 3,
        (Choice::Paper, Choice::Rock) => 1,
        (Choice::Scizzor, Choice::Paper) => 2,
    }
}
/// takes input as choice and results of the game to calculate points
fn calc_point2(choice: Choice, res: Res) -> usize {
    match (choice, res) {
        // if opp choses rock and res is draw then player has choosen rock also, result would be Rock = 1 draw =3
        (Choice::Rock, Res::Draw) => 4,
        (Choice::Paper, Res::Draw) => 5,
        (Choice::Scizzor, Res::Draw) => 6,
        // if opp choses rock and res is win then player has choosen paper , result would be paper = 2 win =6
        (Choice::Rock, Res::Win) => 8,
        (Choice::Paper, Res::Win) => 9,
        (Choice::Scizzor, Res::Win) => 7,
        // if opp choses rock and res is lose then player has choosen Scizzor , result would be scizzor = 3 win =0
        (Choice::Rock, Res::Lose) => 3,
        (Choice::Paper, Res::Lose) => 1,
        (Choice::Scizzor, Res::Lose) => 2,
    }
}

/// parses the line and generates the choices
fn parse_input(line: &str) -> Option<(Choice, Choice)> {
    let ch1 = match line.chars().next()? {
        'A' => Choice::Rock,
        'B' => Choice::Paper,
        'C' => Choice::Scizzor,
        _ => return None,
    };
    let ch2 = match line.chars().nth(2)? {
        'X' => Choice::Rock,
        'Y' => Choice::Paper,
        'Z' => Choice::Scizzor,
        _ => return None,
    };
    Some((ch1, ch2))
}
/// parses the line and generates the choices and results for part2
fn parse_input2(line: &str) -> Option<(Choice, Res)> {
    let ch1 = match line.chars().next()? {
        'A' => Choice::Rock,
        'B' => Choice::Paper,
        'C' => Choice::Scizzor,
        _ => return None,
    };
    let res = match line.chars().nth(2)? {
        'X' => Res::Lose,
        'Y' => Res::Draw,
        'Z' => Res::Win,
        _ => return None,
    };
    Some((ch1, res))
}

fn main() {
    let mut total = 0;
    let mut buf = String::new();
    let mut stdin = std::io::stdin().lock();
    while let Ok(n) = stdin.read_line(&mut buf) {
        if n == 0 {
            break;
        }
        if let Some((opp, me)) = parse_input2(&buf) {
            total += calc_point2(opp, me);
        }
        buf.clear();
    }
    println!("Total Points earned {total}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_check() {
        assert_eq!(parse_input("A Y"), Some((Choice::Rock, Choice::Paper)));
        assert_eq!(parse_input("B Z"), Some((Choice::Paper, Choice::Scizzor)));
        assert_eq!(parse_input("C X"), Some((Choice::Scizzor, Choice::Rock)));
        assert_eq!(parse_input("C N"), None);
        assert_eq!(parse_input("D Z"), None);
        assert_eq!(parse_input(""), None);
    }
    #[test]
    fn calculate() {
        assert_eq!(calc_point((Choice::Rock, Choice::Paper)), 8);
        assert_eq!(calc_point((Choice::Paper, Choice::Scizzor)), 9);
        assert_eq!(calc_point((Choice::Scizzor, Choice::Rock)), 7);
        assert_eq!(calc_point((Choice::Rock, Choice::Rock)), 4);
        assert_eq!(calc_point((Choice::Paper, Choice::Paper)), 5);
        assert_eq!(calc_point((Choice::Scizzor, Choice::Scizzor)), 6);
        assert_eq!(calc_point((Choice::Rock, Choice::Scizzor)), 3);
        assert_eq!(calc_point((Choice::Paper, Choice::Rock)), 1);
        assert_eq!(calc_point((Choice::Scizzor, Choice::Paper)), 2);
    }
}
