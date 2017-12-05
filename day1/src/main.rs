use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let result = sum2(&input);
    println!("{}", result);
}

fn sum(input: &str) -> u32 {
    let mut values = input.chars().map(|v| v.to_digit(10).unwrap());
    let mut acc = 0;
    let first = values.next().unwrap();
    let mut prev = first;
    while let Some(next) = values.next() {

        if prev == next {
            acc += next;
        }

        prev = next;
    }

    if prev == first {
        acc += first;
    }
    
    acc
}

fn sum2(input: &str) -> u32 {
    let values: Vec<u32> = input.chars().map(|v| v.to_digit(10).unwrap()).collect();
    let values2 = values.iter().cycle().skip(values.len() / 2);
    let mut acc = 0;
    
    for (i, j) in values.iter().zip(values2) {
        if i == j {
            acc += i;
        }
    }
    
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "1122";
        let expected = 3;
        let result = sum(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test2() {
        let input = "1111";
        let expected = 4;
        let result = sum(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test3() {
        let input = "1234";
        let expected = 0;
        let result = sum(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test4() {
        let input = "91212129";
        let expected = 9;
        let result = sum(input);
        assert_eq!(expected, result);
    }


    #[test]
    fn test21() {
        let input = "1212";
        let expected = 6;
        let result = sum2(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test22() {
        let input = "1221";
        let expected = 0;
        let result = sum2(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test23() {
        let input = "123425";
        let expected = 4;
        let result = sum2(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test24() {
        let input = "123123";
        let expected = 12;
        let result = sum2(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test25() {
        let input = "12131415";
        let expected = 4;
        let result = sum2(input);
        assert_eq!(expected, result);
    }
}