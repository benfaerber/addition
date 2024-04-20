#![allow(dead_code)]
/// ! Boolean logic implemented in Rust to show the power of pattern matching
/// ! Not allowed to use any boolean operators

#[derive(Debug, PartialEq)]
enum Boolean {
    True,
    False,
}
use Boolean::*;

impl Boolean {
    fn and(&self, other: Self) -> Self {
        match (self, other) {
            (True, True) => True,
            _ => False,
        }
    }

    fn or(&self, other: Self) -> Self {
        match (self, other) {
            (_, True) | (True, _) => True,
            _ => False,
        }
    }

    fn not(&self) -> Self {
        match self {
            True => False,
            False => True,
        }
    }

    fn eq(&self, other: Self) -> Self {
        match (self, other) {
            (True, True) | (False, False) => True,
            _ => False,
        }
    }

    fn neq(&self, other: Self) -> Self {
        self.eq(other).not()
    }
}

/// Implement addition with no + 
#[derive(Copy, Clone, Debug)]
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

struct Number(Vec<Digit>);
impl Number {
    fn add(self, other: Self) {
        let self_larger = self.0.len() > other.0.len(); 

        let nums_a: Vec<Digit> = if self_larger {
            self.0.clone()
        } else {
            fill_to_len(&self.0, other.0.len())
        };

        let nums_b: Vec<Digit> = if !self_larger {
            other.0
        } else {
            fill_to_len(&other.0, self.0.len())
        }; 

        println!("Adding:");
        println!("{nums_a:?}");
        println!("{nums_b:?}");
        
        let mut sol = nums_a.clone();
        let mut range: Vec<usize> = (0..nums_a.len()).collect();
        range.reverse();
        for i in range { 
            let num_a = nums_a.get(i).unwrap();
            let num_b = nums_b.get(i).unwrap();
            
            let (this_base_change, new_digit) = num_a.add_many(num_b);
            let mut base_change = this_base_change;
            let mut curr = if base_change {i-1} else {0}; 
            while base_change {
                println!("Base change");
                let (next_base, next_base_digit) = sol[curr].add_one();
                
                base_change = next_base;
                sol[curr] = next_base_digit; 
                println!("Sol {sol:?}");
                if curr > 0 {
                    curr -= 1;
                }
            }

            sol[i] = new_digit;
        }

        println!("Solution:");
        println!("{sol:?}");
        // println!("A: {a:?}");
        // println!("B: {b:?}");
    }


}


impl Digit {
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


    // fn add_onex(&self, other: &Self) -> Number { 
    //     Number(match (*self, *other) {
    //         (Zero, x) | (x, Zero )=> vec![x], 
    //         (One, One) => vec![Two], 
    //         (One, Two) | (Two, One) => vec![Three], 
    //         (One, Three) | (Three, One) => vec![Four], 
    //         (One, Four) | (Four, One) => vec![Five], 
    //         (One, Five) | (Five, One) => vec![Six], 
    //         (One, Six) | (Six, One) => vec![Seven], 
    //         (One, Seven) | (Seven, One) => vec![Eight],
    //         (One, Eight) | (Eight, One) => vec![Nine], 
    //         (One, Nine) | (Nine, One) => vec![One, Zero], 
    //         _ => todo!(),
    //     })
    // }
}


#[cfg(test)]
mod test {
    use super::Boolean::*;
    use super::*;


    #[test]
    fn number_test() {
        let a = Number(vec![One, Zero, Three]);
        let b = Number(vec![Four]);

        let sol = a.add(b);


        let a = Number(vec![One, Zero, Six]);
        let b = Number(vec![Five]);
        let sol = a.add(b);
    }

    #[test]
    fn boolean_logic_test() {
        // And
        assert_eq!(True.and(False), False);
        assert_eq!(True.and(True), True);
        assert_eq!(False.and(False), False);
        
        // Or
        assert_eq!(True.or(False), True);
        assert_eq!(False.or(True), True);
        assert_eq!(False.or(False), False);
       
        // Not
        assert_eq!(False.not(), True);
        assert_eq!(True.not(), False);
       
        // Equal
        assert_eq!(False.eq(False), True);
        assert_eq!(True.eq(True), True);
        assert_eq!(False.eq(True), False);
        assert_eq!(True.eq(False), False);
        
        // Neq
        assert_eq!(False.neq(True), True);
        assert_eq!(True.neq(True), False);
        assert_eq!(True.neq(False), True);
    }
}
