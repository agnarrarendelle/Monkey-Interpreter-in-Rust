pub struct EvalError(String);

impl fmt::Display for EvalError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", self.0);
    }
}

impl EvalError{
    pub fn new(err: String)->Self{
        Self(err)
    }
}