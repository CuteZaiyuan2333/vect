use crate::isa::{
    calculations::*,
    numbers::*,
};

#[derive(Clone)]
pub enum Instructions{
    // Add,
    Cal(Calculation),
    Jump,
    Larger,
}



#[derive(Clone)]
pub enum Items{
    Element(Instructions),
    Number(Types),
    Recursion(Box<Vec<Items>>),
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