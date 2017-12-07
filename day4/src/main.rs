use std::io;
use std::io::Read;
use std::iter::FromIterator;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let result = count_valid(&input);
    println!("{}", result);
}

fn count_valid(input: &str) -> usize {
    input.lines().map(passphrase_check2).filter(|&x| x == true).count()
}

fn passphrase_check(passphrase: &str) -> bool {
    let words: Vec<&str> = passphrase.split_whitespace().collect();
    let mut clone = words.clone();
    clone.sort_unstable();
    clone.dedup();
    clone.len() == words.len()
}

fn passphrase_check2(passphrase: &str) -> bool {
    let words: Vec<String> = passphrase.split_whitespace().map(|word| {
        let mut chars: Vec<char> = word.chars().collect();
        chars.sort_unstable();
        String::from_iter(chars)     
    }).collect();
    let mut clone = words.clone();
    clone.sort_unstable();
    clone.dedup();
    clone.len() == words.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "aa bb cc dd ee";
        let expected = true;
        let result = passphrase_check(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test2() {
        let input = "aa bb cc dd aa";
        let expected = false;
        let result = passphrase_check(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test3() {
        let input = "aa bb cc dd aaa";
        let expected = true;
        let result = passphrase_check(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test21() {
        let input = "abcde fghij";
        let expected = true;
        let result = passphrase_check2(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test22() {
        let input = "abcde xyz ecdab";
        let expected = false;
        let result = passphrase_check2(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test23() {
        let input = "a ab abc abd abf abj";
        let expected = true;
        let result = passphrase_check2(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test24() {
        let input = "iiii oiii ooii oooi oooo";
        let expected = true;
        let result = passphrase_check2(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test25() {
        let input = "oiii ioii iioi iiio";
        let expected = false;
        let result = passphrase_check2(input);
        assert_eq!(expected, result);
    }

}