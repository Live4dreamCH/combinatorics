use itertools::Itertools;
use std::fmt;

struct Permutation {
    charset: Vec<char>,
}

impl Permutation {
    pub fn from_str(s: &str) -> Result<Permutation, &'static str> {
        let chars: Vec<char> = s.chars().collect();
        Permutation::from(chars)
    }

    pub fn from_slice(s: &[char]) -> Result<Permutation, &'static str> {
        let chars: Vec<char> = s.to_vec();
        Permutation::from(chars)
    }

    fn from(mut chars: Vec<char>) -> Result<Permutation, &'static str> {
        let s_len = chars.len();
        chars.sort();
        chars.dedup();
        match chars.len() {
            0 => Err("Given no char, so no permutation!"),
            l if s_len == l => Ok(Permutation { charset: chars }),
            _ => Err("Given duplicated chars!"),
        }
    }

    pub fn len(&self) -> usize {
        self.charset.len()
    }
}

/// 中介数
enum LehmerCodeValue {
    /// 12345678
    Increse(Vec<i32>),
    /// 87654321
    Decrese(Vec<i32>),
}

impl LehmerCodeValue {
    pub fn from_dicimal(
        mut num: usize,
        charset: &Permutation,
        is_increse: bool,
    ) -> Result<LehmerCodeValue, &'static str> {
        let l = charset.len();
        let mut value = vec![];
        for radix in if is_increse { l..2 } else { 2..l } {
            let remainder = num % radix; //余数
            num = num / radix; // 商
            value.push(remainder as i32);
        }
        if num
            >= match is_increse {
                true => 2,
                false => l,
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
                write!(f, "LehmerCodeValue(inc): ")?;
                v
            }
            LehmerCodeValue::Decrese(v) => {
                write!(f, "LehmerCodeValue(dec): ")?;
                v
            }
        };
        write!(f, "{}", value.iter().format(" "))
    }
}

#[cfg(test)]
mod lehmer_code_value_tests {
    use super::*;
    #[test]
    fn from_dicimal() {
        let charset = Permutation::from_str("12345").unwrap();
        let v = LehmerCodeValue::from_dicimal(0, &charset, false).unwrap();
        assert_eq!(v.to_string(), "LehmerCodeValue(dec): 0 0 0 0");
    }
}

fn main() {
    let charset = Permutation::from_str("12345").unwrap();
    for i in 0..120 {
        println!(
            "{}",
            LehmerCodeValue::from_dicimal(i, &charset, false).unwrap()
        );
    }

    let s = String::from("你好，世界！");
    let p1 = Permutation::from_str(&s).unwrap();
    println!("{:?}", p1.charset);

    let v: Vec<char> = s.chars().collect();
    let p2 = Permutation::from_slice(&v).unwrap();
    println!("{:?}", p2.charset);
}
