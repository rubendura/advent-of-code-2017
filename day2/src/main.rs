use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let result = checksum2(&input);
    println!("{}", result);
}

fn checksum(input: &str) -> i32 {
    let mut acc = 0;
    let lines = input.lines();

    for line in lines {
        let values: Vec<i32> = line.split_whitespace().map(|v| v.parse::<i32>().unwrap()).collect();
        let min = values.iter().min().unwrap();
        let max = values.iter().max().unwrap();
        acc += max - min;
    }
    
    acc
}

fn checksum2(input: &str) -> i32 {
    let mut acc = 0;
    let lines = input.lines();

    for line in lines {
        let values: Vec<i32> = line.split_whitespace().map(|v| v.parse::<i32>().unwrap()).collect();
        'outer: for (ii, i) in values.iter().enumerate() {
            for (ij, j) in values.iter().enumerate() {
                if ii == ij { continue }
                if i % j == 0 {
                    acc += i / j;
                    break 'outer;
                }
            }
        }
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "5 1 9 5\n7 5 3\n2 4 6 8";
        let expected = 18;
        let result = checksum(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test2() {
        let input = "5 9 2 8\n9 4 7 3\n3 8 6 5";
        let expected = 9;
        let result = checksum2(input);
        assert_eq!(expected, result);
    }
}