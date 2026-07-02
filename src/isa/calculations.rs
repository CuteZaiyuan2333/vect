#[derive(Clone)]
pub enum Calculation{
    NumberCal(NumberCalculation),
    BoolCal(BoolCalculation),
    Compare(Compare),
}

#[derive(Clone)]
pub enum NumberCalculation{
    Add,
    Sub,
    Mul,
    Div,
    Qtt,
    Rmd,
}

#[derive(Clone)]
pub enum BoolCalculation{
    And,
    Or,
    Not,
}

#[derive(Clone)]
pub enum Compare{
    Lgr,
    Les,
    Eql,
    Eqlgr,
    Eqles,
}