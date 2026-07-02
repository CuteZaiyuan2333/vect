use crate::isa::{
    calculations::*,
    numbers::*,
    logicflow::*,
};

#[derive(Clone)]
pub enum Instructions{
    Cal(Calculation),
    Jump(Logic),
    Push,
}

#[derive(Clone)]
pub enum Items{
    Element(Instructions),
    Number(Types),
    Recursion(Box<Vec<Items>>),
    Label(String),
    Empty,
    Error,
}impl Items{
    pub fn popnumber(self: &Self) ->Types{
        match self {
            Items::Number(number) =>{
                return number.clone();
            }
            _ =>{
                return Types::Error;
            }
        }
    }
}