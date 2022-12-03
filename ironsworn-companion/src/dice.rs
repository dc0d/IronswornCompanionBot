use rand::Rng;

pub fn roll_100() -> i64 {
    roll(100)
}

pub fn roll(dn: i64) -> i64 {
    roller_fn(dn)
}

//

fn roller_fn(n: i64) -> i64 {
    if n < 2 {
        panic!("number {} must not be less than 2", n);
    }

    let rounds = rand::thread_rng().gen_range(1..=4);
    for _ in 1..rounds {
        let _ = rand::thread_rng().gen_range(1..=n);
    }

    rand::thread_rng().gen_range(1..=n)
}

#[cfg(test)]
mod tests_roller_fn {
    use super::*;

    #[test]
    #[should_panic(expected = "number 1 must not be less than 2")]
    fn given_1_then_should_panic_for_less_than_2() {
        let _ = roller_fn(1);
    }

    #[test]
    #[should_panic(expected = "number 0 must not be less than 2")]
    fn given_0_then_should_panic_for_less_than_2() {
        let _ = roller_fn(0);
    }

    #[test]
    fn smoke_test() {
        let n: i64 = 20;
        let expected_range = 1..=n;
        for _ in 1..1000000 {
            let rn = roller_fn(n);
            assert!(
                expected_range.contains(&rn),
                "range does not contain {}",
                rn
            );
        }
    }
}
