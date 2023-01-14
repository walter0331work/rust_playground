use fancy_regex::Regex;
use lazy_static::lazy_static;

struct RegexFilter {
    regex: Regex,
}

impl RegexFilter {
    fn new(re: &str) -> Self {
        RegexFilter {
            regex: Regex::new(&re).unwrap(),
        }
    }
    fn is_match(&self, text: &str) -> bool {
        // let x = *&self.regex.is_match(text).unwrap_or(false)

        match self.regex.is_match(text) {
            Ok(r) => {
                println!("{:?} is_match {} = {}", text, self.regex, r);
                r
            }
            _ => {
                println!("{:?} is_match {} = {}, Error!", text, self.regex, false);

                false
            }
        }
    }
}

pub fn is_serial(a: i32, b: i32, increment: i32) -> bool {
    (a + increment) == b
}

pub fn get_serial_number_count(tokenized_numbers: Vec<u32>, increment: i32) -> i32 {
    let max = tokenized_numbers.len() - 1;
    let mut count = 0;

    for i in 0..max {
        let current = tokenized_numbers[i] as i32;
        let next = tokenized_numbers[i + 1] as i32;

        if is_serial(current, next, increment) {
            count += 1;
        } else if count != 0 {
            break;
        }
    }

    if count > 0 {
        count += 1;
    }

    count
}

// const getSerialNumberCount = (tokenizedNumbers, increment) => {
//   const l = tokenizedNumbers.length;
//   let count = 0;

//   let i = 0
//   while (i < l) {
//     const currentNum = tokenizedNumbers[i];
//     const nextNum = tokenizedNumbers[i + 1]

//     if (isSerial(currentNum, nextNum, increment)) {
//       const amount = i == 0 ? 2 : 1;
//       count += amount;
//     } else {
//       if (count !== 0) {
//         break;
//       }
//     }
//     i++;
//   }

//   return count
// }

// struct SerialNumberFilter {
//     digits: Vec<i8>,
// }

// impl SerialNumberFilter {
//     fn new(digits: Vec<i8>) -> Self {
//         SerialNumberFilter { digits }
//     }
//     fn is_match(&self, serial_number_count: i8) -> bool {
//         // let x = *&self.regex.is_match(text).unwrap_or(false)

//         match self.regex.is_match(text) {
//             Ok(r) => {
//                 println!("{:?} is_match {} = {}", text, self.regex, r);
//                 r
//             }
//             _ => {
//                 println!("{:?} is_match {} = {}, Error!", text, self.regex, false);

//                 false
//             }
//         }
//     }
// }

lazy_static! {
    static ref R2D: RegexFilter = RegexFilter::new(r"(\d{2}).*?\1");
    static ref R3D: RegexFilter = RegexFilter::new(r"(\d{3}).*?\1");
    static ref R4D: RegexFilter = RegexFilter::new(r"(\d{4}).*?\1");

    //updated
    static ref R2D_X3: RegexFilter = RegexFilter::new(r"(\d{2}).*?\1.*?\1");
    static ref R2_MIRROR: RegexFilter = RegexFilter::new(r"(\d)(\d)\s\2\1");
    static ref R3_MIRROR: RegexFilter = RegexFilter::new(r"(\d)(\d)(\d)\s\3\2\1");
    static ref R4_MIRROR: RegexFilter = RegexFilter::new(r"(\d)(\d)(\d)(\d)\s\4\3\2\1");

    // const R2MirrorReverse = withFilterProp('number', createRegexFilter(/(\d)(\d)\s\1\2/));
    // const R3MirrorReverse = withFilterProp('number', createRegexFilter(/(\d)(\d)(\d)\s\1\2\3/));
    // const R4MirrorReverse = withFilterProp('number', createRegexFilter(/(\d)(\d)(\d)(\d)\s\1\2\3\4/));

    //static ref SERIAL_2 =
    // const SERIAL2 = withFilterProp('tokenized', createSerialNumberFilter(2, 1));
    // const SERIAL3 = withFilterProp('tokenized', createSerialNumberFilter(3, 1));
    // const SERIAL4 = withFilterProp('tokenized', createSerialNumberFilter(4, 1));
    // const SERIAL3R = withFilterProp('tokenized', createSerialNumberFilter(3, -1));
    // const SERIAL4R = withFilterProp('tokenized', createSerialNumberFilter(4, -1));

    // const SIM4 = withFilterProp('tokenized', createSimilarNumberFilter(4));
    // const SIM3 = withFilterProp('tokenized', createSimilarNumberFilter(3));
    // const CARRY = withFilterProp('tokenized', carry);
    // const SAME2 = withFilterProp('number', createRegexFilter(/888\s.*?(\d)(\1)+/));
    // const SAME3 = withFilterProp('number', createRegexFilter(/888\s.*?(\d)(\1){2}/));
    // const SAME4 = withFilterProp('number', createRegexFilter(/888\s.*?(\d)(\1){3}/));

}

#[cfg(test)]
mod test {
    use crate::filters::*;

    const N_R2D: &str = "+888 0012 0034";
    const N_R3D: &str = "+888 0001 0002";
    const N_R4D: &str = "+888 0000 0000";
    const N_R2D_X3: &str = "+888 0000 0012";
    const N_R2_MIRROR: &str = "+888 0089 9800";
    const N_R3_MIRROR: &str = "+888 0789 9870";
    const N_R4_MIRROR: &str = "+888 6789 9876";

    #[tokio::test]
    async fn test_filters() {
        // let is_match = R2D.is_match("+888 0011 2233");
        println!("is_match: {}", R2D.is_match(N_R2D));
        assert_eq!(R2D.is_match(N_R2D), true);
        assert_eq!(R3D.is_match(N_R3D), true);
        assert_eq!(R4D.is_match(N_R4D), true);
        assert_eq!(R2D_X3.is_match(N_R2D_X3), true);
        assert_eq!(R2_MIRROR.is_match(N_R2_MIRROR), true);
        assert_eq!(R3_MIRROR.is_match(N_R3_MIRROR), true);
        assert_eq!(R4_MIRROR.is_match(N_R4_MIRROR), true);
    }

    #[tokio::test]
    async fn test_get_serial_number_count() {
        assert_eq!(get_serial_number_count(vec![1, 2, 3, 4, 0, 0, 0, 0], 1), 4);
        assert_eq!(get_serial_number_count(vec![4, 3, 2, 1, 9, 9, 9, 9], -1), 4);
    }
}
