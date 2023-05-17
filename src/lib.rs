pub fn squoze<T: Into<String>>(input: T) -> Vec<u16> {
    let mut input = input.into();
    let len = input.len();
    if len % 3 != 0 {
        input = format!("{: >width$}", input, width = len + 3 - (len % 3));
    }
    input
        .chars()
        .map(|c| {
            // http://rabbit.eng.miami.edu/info/decchars.html
            match c {
                c if c.is_ascii_alphabetic() => 11 + (c.to_ascii_uppercase() as u8 - b'A') as u8,
                c if c.is_ascii_digit() => 1 + (c as u8 - b'0') as u8,
                c if c.is_whitespace() => 0,
                '.' => 37,
                '$' => 38,
                '%' => 39,
                _ => 0,
            }
        })
        .collect::<Vec<u8>>()
        .chunks(3)
        .map(|chunk| chunk.iter().fold(0u16, |acc, &c| acc * 40 + c as u16))
        .collect::<Vec<u16>>()
}

pub fn desquoze(input: Vec<u16>) -> String {
    if input.is_empty() {
        String::new()
    } else {
        input
            .iter()
            .map(|&x| {
                let mut chunk = String::new();
                let mut x = x;
                while x > 0 {
                    let r = (x % 40) as u8;
                    let ch = match r {
                        0 => ' ',
                        1..=10 => (b'0' + r - 1) as char,
                        11..=36 => (b'A' + r - 11) as char,
                        37 => '.',
                        38 => '$',
                        39 => '%',
                        _ => ' ',
                    };
                    chunk = format!("{}{}", ch, chunk);
                    x = x / 40;
                }
                format!("{: >3}", chunk)
            })
            .collect::<String>()
    }
}

#[cfg(test)]
mod squoze_tests {
    use super::*;

    #[test]
    fn test_squoze_len_0() {
        assert_eq!(Vec::<u16>::new(), squoze(""));
    }

    #[test]
    fn test_squoze_should_pad_input_left_to_mulitple_of_3() {
        assert_eq!(squoze("  0"), squoze("0"));
        assert_eq!(squoze(" 10"), squoze("10"));
    }

    #[test]
    fn test_squoze_len_1() {
        assert_eq!(vec![0], squoze(" "));
        assert_eq!(vec![6], squoze("5"));
        assert_eq!(vec![10], squoze("9"));
        assert_eq!(vec![11], squoze("A"));
        assert_eq!(vec![12], squoze("b"));
        assert_eq!(vec![36], squoze("Z"));
        assert_eq!(vec![36], squoze("z"));
        assert_eq!(vec![37], squoze("."));
        assert_eq!(vec![38], squoze("$"));
        assert_eq!(vec![39], squoze("%"));
        assert_eq!(vec![0], squoze("\t"));
        assert_eq!(vec![0], squoze("\n"));
        assert_eq!(vec![0], squoze(";"));
        assert_eq!(vec![0], squoze(","));
        assert_eq!(vec![0], squoze("?"));
    }

    #[test]
    fn test_squoze_len_2() {
        assert_eq!(vec![40 * 1 + 2], squoze("01"));
        assert_eq!(vec![40 * 11 + 12], squoze("AB"));
    }

    #[test]
    fn test_squoze_len_3() {
        assert_eq!(vec![1600 * 0 + 40 * 0 + 0], squoze("   "));
        assert_eq!(vec![1600 * 1 + 40 * 0 + 0], squoze("0  "));
        assert_eq!(vec![1600 * 0 + 40 * 1 + 0], squoze(" 0 "));
        assert_eq!(vec![1600 * 0 + 40 * 0 + 1], squoze("  0"));
        assert_eq!(vec![1600 * 0 + 40 * 0 + 11], squoze("  A"));
        assert_eq!(vec![1600 * 11 + 40 * 12 + 13], squoze("ABC"));
        assert_eq!(vec![1600 * 36 + 40 * 35 + 34], squoze("ZYX"));
        assert_eq!(vec![1600 * 24 + 40 * 11 + 23], squoze("NAM"));
    }

    #[test]
    fn test_squoze_len_4() {
        assert_eq!(
            vec![1600 * 0 + 40 * 0 + 18, 1600 * 15 + 40 * 22 + 22],
            squoze("HELL")
        );
    }

    #[test]
    fn test_squoze_len_5() {
        assert_eq!(
            vec![1600 * 0 + 40 * 18 + 15, 1600 * 22 + 40 * 22 + 25],
            squoze("HELLO")
        );
    }

    #[test]
    fn test_squoze_len_6() {
        assert_eq!(
            vec![1600 * 11 + 40 * 22 + 22, 1600 * 25 + 40 * 18 + 11],
            squoze("ALLOHA")
        );
    }
}

#[cfg(test)]
mod desquoze_tests {
    use super::*;

    #[test]
    fn test_desquoze() {
        assert_eq!("".to_string(), desquoze(vec![]));
        assert_eq!("   ".to_string(), desquoze(vec![0]));
        assert_eq!("  0".to_string(), desquoze(vec![1]));
        assert_eq!("  A".to_string(), desquoze(vec![11]));
        assert_eq!("  %".to_string(), desquoze(vec![39]));
        assert_eq!(" AB".to_string(), desquoze(vec![452]));
        assert_eq!("ABC".to_string(), desquoze(vec![18093]));
        assert_eq!("  HELL".to_string(), desquoze(vec![18, 24902]));
    }
}
