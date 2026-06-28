#[derive(Clone)]
pub enum Instructions{
    Add,
    Jump,
    Lgr,
}

#[derive(Clone)]
pub enum Items{
    Element(Instructions),
    Number(i32),
    Recursion(Box<Vec<Items>>),
    Error,
}