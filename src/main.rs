use itertools::Itertools;
use std::fmt;

/// 字符集，内部字符升序排列
#[derive(Debug)]
struct CharSet {
    chars: Vec<char>,
}

impl CharSet {
    pub fn from_str(s: &str) -> Result<CharSet, &'static str> {
        let chs: Vec<char> = s.chars().collect();
        CharSet::from(chs)
    }

    pub fn from_slice(s: &[char]) -> Result<CharSet, &'static str> {
        let chs: Vec<char> = s.to_vec();
        CharSet::from(chs)
    }

    fn from(mut chs: Vec<char>) -> Result<CharSet, &'static str> {
        let s_len = chs.len();
        chs.sort();
        chs.dedup();
        match chs.len() {
            0 => Err("Given no char, so no permutation!"),
            l if s_len == l => Ok(CharSet { chars: chs }),
            _ => Err("Given duplicated chars!"),
        }
    }

    pub fn len(&self) -> usize {
        self.chars.len()
    }
}

#[cfg(test)]
mod char_set_tests {
    use super::*;
    #[test]
    fn from_xxx() {
        let s = String::from("你好，世界！");
        let p1 = CharSet::from_str(&s).unwrap();

        let v: Vec<char> = vec!['你', '好', '，', '世', '界', '！'];
        let p2 = CharSet::from_slice(&v).unwrap();
        assert_eq!(p1.chars, p2.chars);

        assert_eq!(
            CharSet::from(vec![]).unwrap_err(),
            "Given no char, so no permutation!"
        );
        assert_eq!(
            CharSet::from(vec!['a', 'b', 'a']).unwrap_err(),
            "Given duplicated chars!"
        );
    }
}

/// 中介数
#[derive(Debug)]
enum LehmerCodeValue {
    /// 12345678
    Decrese(Vec<i32>),
    /// 87654321
    Increse(Vec<i32>),
}

impl LehmerCodeValue {
    pub fn from_dicimal(
        mut num: usize,
        charset: &CharSet,
        is_increse: bool,
    ) -> Result<LehmerCodeValue, &'static str> {
        let l = charset.len();
        if l == 1 {
            return Err("The Lehmer code of single char is meanless!");
        }
        let mut value = vec![];
        let (begin, end) = match is_increse {
            true => (2, l),
            false => (l, 2),
        };
        let mut radix = begin; // 进制 基数
        while radix != end {
            let remainder = num % radix; //余数
            num = num / radix; // 商
            value.push(remainder as i32);
            if is_increse {
                radix += 1
            } else {
                radix -= 1;
            }
        }
        if num
            >= match is_increse {
                true => l,
                false => 2,
            }
        {
            return Err("A dicimal larger than total number of this permutation!");
        }
        value.push(num as i32);
        value.reverse();
        let code = match is_increse {
            true => LehmerCodeValue::Increse(value),
            false => LehmerCodeValue::Decrese(value),
        };
        Ok(code)
    }
}

impl fmt::Display for LehmerCodeValue {
    // 这个 trait 要求 `fmt` 使用与下面的函数完全一致的函数签名
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 仅将 self 的第一个元素写入到给定的输出流 `f`。返回 `fmt:Result`，此
        // 结果表明操作成功或失败。注意 `write!` 的用法和 `println!` 很相似。
        let value = match self {
            LehmerCodeValue::Increse(v) => {
                write!(f, "L(inc): ")?;
                v
            }
            LehmerCodeValue::Decrese(v) => {
                write!(f, "L(dec): ")?;
                v
            }
        };
        write!(f, "{}", value.iter().format(" "))
    }
}

#[cfg(test)]
mod lehmer_code_value_tests {
    use super::*;
    use LehmerCodeValue as L;
    #[test]
    fn from_dicimal() {
        let charset = CharSet::from_str("12345").unwrap();
        let dicimal_lehmer = vec![
            (0, "0 0 0 0"),
            (1, "0 0 0 1"),
            (2, "0 0 1 0"),
            (3, "0 0 1 1"),
            (4, "0 0 2 0"),
            (117, "4 3 1 1"),
            (118, "4 3 2 0"),
            (119, "4 3 2 1"),
        ];
        for (d, l) in dicimal_lehmer {
            assert_eq!(
                L::from_dicimal(d, &charset, true).unwrap().to_string(),
                format!("{}{}", "L(inc): ", l)
            );
        }
        assert!(L::from_dicimal(120, &charset, true).is_err());
        assert!(L::from_dicimal(121, &charset, true).is_err());
        assert!(L::from_dicimal(120, &charset, false).is_err());
        assert!(L::from_dicimal(121, &charset, false).is_err());
        assert!(L::from_dicimal(usize::MAX, &charset, true).is_err());
        assert!(L::from_dicimal(usize::MAX, &charset, false).is_err());
        let dicimal_lehmer = vec![
            (0, "0 0 0 0"),
            (1, "0 0 0 1"),
            (2, "0 0 0 2"),
            (3, "0 0 0 3"),
            (4, "0 0 0 4"),
            (5, "0 0 1 0"),
            (6, "0 0 1 1"),
            (117, "1 2 3 2"),
            (118, "1 2 3 3"),
            (119, "1 2 3 4"),
        ];
        for (d, l) in dicimal_lehmer {
            assert_eq!(
                L::from_dicimal(d, &charset, false).unwrap().to_string(),
                format!("{}{}", "L(dec): ", l)
            );
        }

        let charset = CharSet::from_str("12").unwrap();
        assert_eq!(
            L::from_dicimal(0, &charset, true).unwrap().to_string(),
            format!("{}{}", "L(inc): ", "0")
        );
        assert_eq!(
            L::from_dicimal(1, &charset, true).unwrap().to_string(),
            format!("{}{}", "L(inc): ", "1")
        );
        assert_eq!(
            L::from_dicimal(2, &charset, true).unwrap_err(),
            "A dicimal larger than total number of this permutation!"
        );
        assert!(L::from_dicimal(2, &charset, false).is_err());

        let charset = CharSet::from_str("1").unwrap();
        assert_eq!(
            L::from_dicimal(0, &charset, true).unwrap_err(),
            "The Lehmer code of single char is meanless!"
        );
        assert_eq!(
            L::from_dicimal(1, &charset, true).unwrap_err(),
            "The Lehmer code of single char is meanless!"
        );
    }
}

fn main() {}
