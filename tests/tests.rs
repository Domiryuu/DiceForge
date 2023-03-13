#[cfg(test)]
mod tests {
    use dice_forge::roll;
    use dice_forge::Equation;
    #[test]
    //#[should_panic(expected = "Divide by zero error")]
    fn devide_zero() {
        match Equation::new("5/0").expect("test").roll() {
            Ok(_v) => {}
            Err(e) => {
                assert_eq!("Error: Attempted to divide by 0", format!("{}", e));
            }
        }
    }
    #[test]
    fn spaces() {
        Equation::new("1 d 5 + 2").expect("test").roll().unwrap();
    }
    #[test]
    fn basic_math() {
        assert_eq!(13, Equation::new("3+2*5").expect("test").roll().unwrap());
    }

    #[test]
    fn basic_math_add() {
        assert_eq!(5, Equation::new("3+2").expect("test").roll().unwrap());
    }

    #[test]
    fn basic_math_sub() {
        assert_eq!(-3, Equation::new("2-5").expect("test").roll().unwrap());
    }

    #[test]
    fn basic_math_mult() {
        assert_eq!(10, Equation::new("2*5").expect("test").roll().unwrap());
    }

    #[test]
    fn basic_math_div() {
        assert_eq!(5, Equation::new("10/2").expect("test").roll().unwrap());
    }

    #[test]
    fn basic_math_exp() {
        assert_eq!(25, Equation::new("5^2").expect("test").roll().unwrap());
    }
    #[test]
    fn order_of_op() {
        assert_eq!(
            26,
            Equation::new("2(1 - 5) ^ 2 + -3 * 2")
                .expect("test")
                .roll()
                .unwrap()
        );
    }
    #[test]
    fn order_of_op2() {
        assert_eq!(
            38,
            Equation::new("2(1 - 5) ^ 2 + 3 * 2")
                .expect("test")
                .roll()
                .unwrap()
        );
    }
    #[test]
    fn low_roll() {
        assert_eq!(10, Equation::new("10d20").expect("test").low().unwrap());
    }
    #[test]
    fn high_roll() {
        assert_eq!(200, Equation::new("10d20").expect("test").high().unwrap());
    }
    #[test]
    fn range_of_roll() {
        assert_eq!(
            (10, 200),
            Equation::new("10d20").expect("test").range().unwrap()
        );
    }
    #[test]
    fn average_roll() {
        assert_eq!(
            105,
            Equation::new("10d20").expect("test").average().unwrap()
        );
    }
    #[test]
    fn test_roll() {
        let q = Equation::new("1d2").expect("test");
        for _n in 0..100 {
            match q.roll() {
                Ok(1) => {}
                Ok(2) => {}
                _ => panic!(),
            }
        }
    }
    #[test]
    fn roll_call_ok() {
        match roll::roll("1d4+5") {
            Ok(_v) => {}
            Err(_e) => panic!(),
        }
    }
    #[test]
    fn roll_call_err_0() {
        match roll::roll("1/0") {
            Ok(_v) => {}
            Err(e) => assert_eq!("Error: Attempted to divide by 0", format!("{}", e)),
        }
    }
    #[test]
    fn roll_call_err_v() {
        match roll::roll("test") {
            Ok(_v) => {}
            Err(e) => assert_eq!(
                "Error: Unexpected token \'t\' found while parsing",
                format!("{}", e)
            ),
        }
    }
    #[test]
    fn roll_with_advantage() {
        let my_eq = Equation::new("d20").unwrap();
        let _roll = my_eq.advantage().unwrap();
    }
    #[test]
    fn roll_with_disadvantage() {
        let my_eq = Equation::new("d20").unwrap();
        let _roll = my_eq.disadvantage().unwrap();
    }
}
