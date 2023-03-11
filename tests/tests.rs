#[cfg(test)]
mod tests {
    use dice_forge::Equation;
    #[test]
    #[should_panic(expected = "Divide by zero error")]
    fn devide_zero() {
        Equation::new("5/0").expect("test").roll();
    }
    #[test]
    fn basic_math() {
        assert_eq!(13, Equation::new("3+2*5").expect("test").roll());
    }

    #[test]
    fn basic_math_add() {
        assert_eq!(5, Equation::new("3+2").expect("test").roll());
    }

    #[test]
    fn basic_math_sub() {
        assert_eq!(-3, Equation::new("2-5").expect("test").roll());
    }

    #[test]
    fn basic_math_mult() {
        assert_eq!(10, Equation::new("2*5").expect("test").roll());
    }

    #[test]
    fn basic_math_div() {
        assert_eq!(5, Equation::new("10/2").expect("test").roll());
    }

    #[test]
    fn basic_math_exp() {
        assert_eq!(25, Equation::new("5^2").expect("test").roll());
    }
    #[test]
    fn order_of_op() {
        assert_eq!(
            26,
            Equation::new("2(1 - 5) ^ 2 + -3 * 2").expect("test").roll()
        );
    }
    #[test]
    fn order_of_op2() {
        assert_eq!(
            38,
            Equation::new("2(1 - 5) ^ 2 + 3 * 2").expect("test").roll()
        );
    }
    #[test]
    fn low_roll() {
        assert_eq!(10, Equation::new("10d20").expect("test").low());
    }
    #[test]
    fn high_roll() {
        assert_eq!(200, Equation::new("10d20").expect("test").high());
    }
    #[test]
    fn range_of_roll() {
        assert_eq!((10, 200), Equation::new("10d20").expect("test").range());
    }
    #[test]
    fn average_roll() {
        assert_eq!(105, Equation::new("10d20").expect("test").average());
    }
    #[test]
    fn test_roll() {
        let q = Equation::new("1d2").expect("test");
        for _n in 0..100 {
            match q.roll() {
                1 => {}
                2 => {}
                _ => panic!(),
            }
        }
    }
}
