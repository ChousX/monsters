pub struct Attributes{
    pub intelligence: u32,
    pub strength: u32,
    pub dexterity: u32,
    pub perception: u32,
    pub magic: u32,
}

impl Attributes {
    pub fn add(&mut self, other: &Self){
        self.intelligence += other.intelligence;
        self.strength += other.strength;
        self.dexterity += other.dexterity;
        self.perception += other.perception;
        self.magic += other.magic;
    }
}

impl Default for Attributes{
    fn default() -> Self {
        Self{
            intelligence: 0,
            strength: 0,
            dexterity: 0,
            perception: 0,
            magic: 0,
        }
    }
}
