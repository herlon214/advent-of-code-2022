#[derive(Debug, PartialEq, Eq)]
pub enum Operation {
    Multiply(Option<u128>),
    Add(Option<u128>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Test {
    Divisible(u128),
}
