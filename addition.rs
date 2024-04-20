#![allow(dead_code)]
/// ! Implement Addition with no plus sign 

/// My boolean implementation
use super::boolean::Boolean;
use super::boolean::Boolean::*;

const DEBUG_OUTPUT: Boolean = False;

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

fn fill_to_len(lst: &Vec<Digit>, size: usize) -> Vec<Digit> {
    let iterations = size - lst.len();
    let mut reved = lst.clone();
    reved.reverse();
    for _ in 0..iterations {
        reved.push(Zero); 
    }
    reved.reverse();
    reved
}

#[derive(PartialEq, Debug)]
struct Number(Vec<Digit>);

impl Number {
    fn from(number: i32) -> Self {
        let digits: Vec<_> = number.to_string()
            .chars()
            .into_iter()
            .map(Digit::from_char)
            .collect();
        Self(digits)
    }

    fn add(self, other: Self) -> Number {
        let self_larger = self.0.len() > other.0.len(); 
        let self_len = self.0.len();
        let other_len = other.0.len();

        let nums_a = if self_larger {
            self.0
        } else {
            fill_to_len(&self.0, other_len) 
        };

        let nums_b = if !self_larger {
            other.0
        } else {
            fill_to_len(&other.0, self_len) 
        }; 
    
        if DEBUG_OUTPUT == True {
            println!("Adding:");
            println!("{nums_a:?}");
            println!("{nums_b:?}");
        }
        
        let mut sol = nums_a.clone();
        for i in (0..nums_a.len()).rev() { 
            let num_a = sol.get(i).unwrap();
            let num_b = nums_b.get(i).unwrap();
            
            let (this_base_change, new_digit) = num_a.add_many(num_b);
            let mut base_change = this_base_change;
            let mut curr = i; 
            while base_change == True {
                curr -= 1;
                // println!("Base change");
                let (next_base, next_base_digit) = sol[curr].add_one();
                
                base_change = next_base;
                sol[curr] = next_base_digit;
                // println!("Sol {sol:?}");
            }
            
            sol[i] = new_digit;
        }

        if DEBUG_OUTPUT == True {
            println!("Solution:");
            println!("{sol:?}");
        }
        
        Number(sol)
    }
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number::from(value) 
    }
}


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
            _ => panic!("Invalid digit")
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

    fn add_one(&self) -> (Boolean, Digit) {
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

        let base_change = if matches!(self, Nine) {True} else {False}; 
        (base_change, digit)
    }

    fn add_many(&self, other: &Digit) -> (Boolean, Digit) {
        let ones = other.as_ones();
        let mut has_base_change = False;
        let mut digit = *self;
        for _ in ones {
            let (base_change, new_digit) = digit.add_one();
            digit = new_digit;
            if base_change == True {
                has_base_change = True;
            }
        }
        (has_base_change, digit) 
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn number_test() {
        let a = Number::from(3);
        let b = Number::from(5);
        assert_eq!(a.add(b), Number::from(8));

        let a = Number::from(10);
        let b = Number::from(5);
        assert_eq!(a.add(b), Number::from(15));

        let a: Number = 103.into();
        let b: Number = 4.into();
        assert_eq!(a.add(b), Number::from(107)); 

        let a: Number = 106.into();
        let b: Number = 5.into();
        assert_eq!(a.add(b), Number::from(111)); 

        let a: Number = 106.into();
        let b: Number = 54.into();
        assert_eq!(a.add(b), Number::from(160));
    }
}
