#![allow(dead_code)]
/// ! Implement Addition with no plus sign

const DEBUG_OUTPUT: bool = true;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}
use Digit::*;

impl Digit {
    fn from_char(digit: char) -> Self {
        match digit {
            '0' => Zero,
            '1' => One,
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            _ => panic!("Invalid digit"),
        }
    }

    fn as_ones(&self) -> Vec<Digit> {
        match self {
            Zero => vec![],
            One => vec![One],
            Two => vec![One, One],
            Three => vec![One, One, One],
            Four => vec![One, One, One, One],
            Five => vec![One, One, One, One, One],
            Six => vec![One, One, One, One, One, One],
            Seven => vec![One, One, One, One, One, One, One],
            Eight => vec![One, One, One, One, One, One, One, One],
            Nine => vec![One, One, One, One, One, One, One, One, One],
        }
    }

    fn add_one(&self) -> (bool, Digit) {
        let digit = match self {
            Zero => One,
            One => Two,
            Two => Three,
            Three => Four,
            Four => Five,
            Five => Six,
            Six => Seven,
            Seven => Eight,
            Eight => Nine,
            Nine => Zero,
        };

        let base_change = matches!(self, Nine);
        (base_change, digit)
    }

    fn add_many(&self, other: &Digit) -> (bool, Digit) {
        let ones = other.as_ones();
        let mut has_base_change = false;
        let mut digit = *self;
        for _ in ones {
            let (base_change, new_digit) = digit.add_one();
            digit = new_digit;
            if base_change {
                has_base_change = true;
            }
        }
        (has_base_change, digit)
    }
}

fn fill_to_len(lst: &Vec<Digit>, size: usize) -> Vec<Digit> {
    let mut reved = lst.clone();
    reved.reverse();
    for _ in 0..(size - lst.len()) {
        reved.push(Zero);
    }
    reved.reverse();
    reved
}

#[derive(PartialEq, Debug)]
struct Number(Vec<Digit>);

impl Number {
    fn add(self, other: Self) -> Number {
        let self_len = self.0.len();
        let other_len = other.0.len();
        
        let (nums_a, nums_b) = if self_len > other_len {
            (self.0, fill_to_len(&other.0, self_len))
        } else {
            (other.0, fill_to_len(&self.0, other_len))
        };

        if DEBUG_OUTPUT {
            println!("Adding:");
            println!("{nums_a:?}");
            println!("{nums_b:?}");
        }

        let mut sol = nums_a; 
        for i in (0..sol.len()).rev() {
            let num_a = sol.get(i).unwrap();
            let num_b = nums_b.get(i).unwrap();

            let (this_base_change, new_digit) = num_a.add_many(num_b);
            let mut base_change = this_base_change;
            let mut curr = i;
            while base_change {
                curr -= 1;
                // println!("Base change");
                let (next_base, next_base_digit) = sol[curr].add_one();

                base_change = next_base;
                sol[curr] = next_base_digit;
                // println!("Sol {sol:?}");
            }

            sol[i] = new_digit;
        }

        if DEBUG_OUTPUT {
            println!("Solution:");
            println!("{sol:?}");
        }

        Number(sol)
    }
}


macro_rules! add_from_int_impl {
    ($($t:ty)*) => ($(
        impl From<$t> for Number {
            fn from(value: $t) -> Self {
                let digits: Vec<_> = value
                    .to_string()
                    .chars()
                    .into_iter()
                    .map(Digit::from_char)
                    .collect();
                Self(digits)
            }
        }
    )*)
}

add_from_int_impl! { isize i8 i16 i32 i64 i128 }


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn number_test() {
        {
            let a = Number::from(3_i8);
            let b = Number::from(5_i8);
            assert_eq!(a.add(b), Number::from(8));
        }
        {
            let a = Number::from(10);
            let b = Number::from(5);
            assert_eq!(a.add(b), Number::from(15));
        }
        {
            let a: Number = 103.into();
            let b: Number = 4.into();
            assert_eq!(a.add(b), Number::from(107));
        }
        {
            let a: Number = 106.into();
            let b: Number = 5.into();
            assert_eq!(a.add(b), Number::from(111));
        }
        {
            let a: Number = 106.into();
            let b: Number = 54.into();
            assert_eq!(a.add(b), Number::from(160));
        }
    }
}
