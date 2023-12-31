use std::char;

pub fn cal_from_str(s: &str) -> u32 {
    let list = s
        .chars()
        .into_iter()
        .filter(|x| x.to_digit(10).is_some())
        .collect::<Vec<char>>();

    let f = list.first().unwrap();
    let l = list.last().unwrap_or(f);
    u32::from_str_radix(&format!("{f}{l}"), 10).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn parse_test(s: &str, expect: u32) {
        assert_eq!(cal_from_str(s), expect);
    }

    #[test]
    fn do_tests() {
        parse_test("1abc2", 12);
        parse_test("pqr3stu8vwx", 38);
        parse_test("a1b2c3d4e5f", 15);
        parse_test("treb7uchet", 77);
    }
}
