use super::{feature::Feature, attribute::Attributes};

//not sure if its the best idea to have the deafault of a check be true but here we are
pub trait Skill {
    fn check_features(&self, features: &Vec<Box<dyn Feature>>) -> bool {true}
    fn check_attributes(&self, attribute: &Attributes) -> bool {true}
}


pub trait Active: Skill {

}

pub trait Passive: Skill{

}


