use super::{attribute::Attributes, feature::Feature, skill::*};
use genetics::*;

pub struct Organism{
    attributes: Attributes,
    features: Vec<Box<dyn Feature>>,
    skills: Vec<Box<dyn Skill>>,
}

impl Organism{
    pub fn new() -> Self{
        Self::default()
    }

    pub fn add_feature(&mut self, feature: Box<dyn Feature>){
        self.features.push(feature);
    }

    fn init_attribute_from_features(&mut self){
        for feature in self.features.iter(){
            feature.add_requirments(&mut self.attributes);
        }
    }

    fn init_skills_from_features(&mut self){
        for feature in self.features.iter(){
            if let Some(skill) = feature.get_inate_skills(){
                self.skills.push(skill);
            }
        }
    }

    pub fn init(&mut self){
        self.init_attribute_from_features();
        self.init_skills_from_features();
    }
}

impl Genetic for Organism{
    fn compress(&self) -> GeneticType {
        
    }

    fn generait(input: GeneticType) -> Self {
    
    }
}

impl Default for Organism{
    fn default() -> Self {
        Self{
            attributes: Attributes::default(),
            features: Vec::new(),
            skills: Vec::new(),
        }
    }
}
