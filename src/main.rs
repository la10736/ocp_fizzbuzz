trait Apply {
    fn apply(&self, i: i32) -> String;
}

struct DefaultApply;

impl Apply for DefaultApply {
    fn apply(&self, _i: i32) -> String {
        format!("{}", "Maybe you should replace DefaultApply by something else")
    }
}

pub fn fizzbuzz() -> impl Apply {
    DefaultApply
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

//    #[test]
//    //#[should_panic]
//    fn should_return_fizz_for_multiple_of_3() {
//        let fb = fizzbuzz();
//
//        assert_eq!("Fizz", &fb.apply(3));
//        assert_eq!("Fizz", &fb.apply(36));
//    }

//    #[test]
//    //#[should_panic]
//    fn should_return_buzz_for_multiple_of_5() {
//        let fb = fizzbuzz();
//
//        assert_eq!("Buzz", &fb.apply(5));
//        assert_eq!("Buzz", &fb.apply(25));
//    }

//    #[test]
//    //#[should_panic]
//    fn should_return_fizzbuzz_for_multiple_of_3_and_5() {
//        let fb = fizzbuzz();
//
//        assert_eq!("FizzBuzz", &fb.apply(15));
//        assert_eq!("FizzBuzz", &fb.apply(45));
//    }
}
