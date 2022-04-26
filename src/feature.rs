use super::{attribute::Attributes, skill::Skill};


pub trait Feature{
    fn get_requirments(&self) -> Attributes;
    fn add_requirments(&self, attributes: &mut Attributes){
        attributes.add(&self.get_requirments());
    }

    fn get_inate_skills(&self) -> Option<Box<dyn Skill>>{
        None
    }
    
    fn get_tag(&self) -> &str;
}
