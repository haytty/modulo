use std::str::FromStr;
use rust_decimal::{Decimal, Error};

pub struct Calculator {
    modend1: Decimal,
    modend2: Decimal,
}

impl Calculator {
    pub fn from_str(modend1: String, modend2: String) -> Result<Self, Error> {
        let mod1 = Decimal::from_str(&modend1)?;
        let mod2 = Decimal::from_str(&modend2)?;
        let c = Self {
            modend1: mod1,
            modend2: mod2,
        };
        Ok(c)
    }

    pub fn calc(&self) -> Decimal {
        self.modend1 % self.modend2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::*;

    #[test]
    fn test_from_str_success() {
        let c = Calculator::from_str("100.5".into(), "50.5".into());

        assert!(c.is_ok());
        let c = c.unwrap();

        assert_eq!(c.modend1, dec!(100.5));
        assert_eq!(c.modend2, dec!(50.5));
    }

    #[test]
    fn test_from_str_failure() {
        let c = Calculator::from_str("abc".into(), "50.5".into());
        assert!(c.is_err());
    }

    #[test]
    fn test_calc() {
        let c = Calculator::from_str("100.5".to_string(), "50.5".to_string())
            .expect("Calculator initialization failed");

        assert_eq!(c.calc(), dec!(50.0));
    }
}