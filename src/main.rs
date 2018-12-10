trait Apply {
    fn apply(&self, i: i32) -> String;
}

trait Rule {
    fn resolve(&self, i: i32) -> Option<String>;
}

impl<R: Rule> Apply for R {
    fn apply(&self, i: i32) -> String {
        self.resolve(i).unwrap()
    }
}

trait Compose: Sized {
    fn compose<C>(self, c: C) -> (Self, C) {
        (self, c)
    }
}
impl<T> Compose for T {}

impl<A: Rule, B: Rule> Rule for (A, B) {
    fn resolve(&self, i: i32) -> Option<String> {
        self.0.resolve(i).or_else(|| self.1.resolve(i))
    }
}


struct AndRule<A: Rule, B: Rule>(A, B);
impl<A: Rule, B: Rule> Rule for AndRule<A, B> {
    fn resolve(&self, i: i32) -> Option<String> {
        Some(format!("{}{}", self.0.resolve(i)?, self.1.resolve(i)?))
//        match (self.0.resolve(i), self.1.resolve(i)) {
//            (Some(m0), Some(m1)) => Some(format!("{}{}", m0, m1)),
//            _ => None
//        }
//        self.0.resolve(i).and_then(|first| self.1.resolve(i).map(|second| format!("{}{}", first, second)))
    }
}

struct DivisorTag {
    divisor: i32,
    tag: String
}

impl Rule for DivisorTag {
    fn resolve(&self, i: i32) -> Option<String> {
        match i%self.divisor {
            0 => Some(self.tag.clone()),
            _ => None
        }
    }
}

struct StrNumber;

impl Rule for StrNumber {
    fn resolve(&self, i: i32) -> Option<String> {
        Some(format!("{}", i))
    }
}

fn fizz() -> impl Rule {
    DivisorTag { divisor: 3, tag: "Fizz".to_string() }
}

fn buzz() -> impl Rule {
    DivisorTag { divisor: 5, tag: "Buzz".to_string() }
}

fn fizz_buzz() -> impl Rule {
    AndRule(fizz(), buzz())
}

fn str_number() -> impl Rule {
    StrNumber
}

pub fn fizzbuzz() -> impl Apply {
    fizz_buzz()
        .compose(fizz())
        .compose(buzz())
        .compose(str_number())
}

fn main() {
    let fb = fizzbuzz();

    for i in 1..101 {
        println!("{}", fb.apply(i));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn otherwise_should_return_the_number() {
        let fb = fizzbuzz();

        assert_eq!("2", &fb.apply(2));
        assert_eq!("8", &fb.apply(8));
    }

    #[test]
    fn should_return_fizz_for_multiple_of_3() {
        let fb = fizzbuzz();

        assert_eq!("Fizz", &fb.apply(3));
        assert_eq!("Fizz", &fb.apply(36));
    }

    #[test]
    fn should_return_buzz_for_multiple_of_5() {
        let fb = fizzbuzz();

        assert_eq!("Buzz", &fb.apply(5));
        assert_eq!("Buzz", &fb.apply(25));
    }

    #[test]
    fn should_return_fizzbuzz_for_multiple_of_3_and_5() {
        let fb = fizzbuzz();

        assert_eq!("FizzBuzz", &fb.apply(15));
        assert_eq!("FizzBuzz", &fb.apply(45));
    }
}
