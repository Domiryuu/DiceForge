#[derive(Debug)]
pub enum DivideByZeroError {
    DivideByZero,
}

impl std::fmt::Display for DivideByZeroError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: Attempted to divide by 0")
    }
}
#[derive(Debug)]
pub enum InvalidExpressionError {
    InvalidExpression,
    InvalidDie,
    InvalidToken(char),
}

impl std::fmt::Display for InvalidExpressionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidExpressionError::InvalidExpression => write!(
                f,
                "Error: The expression could not be parsed, incorrect format?"
            ),
            InvalidExpressionError::InvalidDie => {
                write!(f, "Error: Die expression could not be parsed.")
            }
            InvalidExpressionError::InvalidToken(t) => {
                write!(
                    f,
                    "Error: Unexpected token \'{}\' found while parsing",
                    InvalidExpressionError::InvalidToken(*t)
                )
            }
        }
    }
}
