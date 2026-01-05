use std::collections::HashMap;
use serde_derive::*;

pub type PointT = f64;

pub enum PointTypes {
    HATE,
    LOVE,
    FRIEND,
    SILLY
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct SRelPoints {

// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SRelation {
    p_hate: PointT,
    p_love: PointT,
    p_friend: PointT,
    p_silly: PointT,
}

#[allow(dead_code)]
pub struct Kitten {
    pub age: i16,
    pub silly: PointT,
    pub name: &'static str,
    pub relations: HashMap<String, SRelation>,
    pub names: Vec<String>
}

#[allow(dead_code)]
impl Kitten {
    pub fn new(name: &'static str, age: i16, silly: PointT) -> Kitten {
        Self {
            name     : name,
            age      : age,
            silly    : silly,
            relations: HashMap::new(),
            names    : Vec::new(),
        }
    }

    pub fn new_relation(&mut self, name: &str) {
        self.relations.insert(name.to_string(), SRelation {
            p_hate: 0.0,
            p_love: 0.0,
            p_friend: 0.0,
            p_silly: 0.0,
        });
        self.names.push(name.to_string());
    }

    pub fn set_relation(&mut self, name: &str, data: SRelation) {
        self.relations.insert(name.to_string(), data);
    }

    pub fn get_relation(&mut self, name: &str) -> std::option::Option<&SRelation> {
        self.relations.get(name)
    }

    pub fn set_points(&mut self, name: &str, ptype: PointTypes, value: PointT) {
        let mut rel = self.get_relation(name).unwrap().clone();
        match ptype {
            PointTypes::HATE    => rel.p_hate   = value,
            PointTypes::SILLY   => rel.p_silly  = value,
            PointTypes::FRIEND  => rel.p_friend = value,
            PointTypes::LOVE    => rel.p_love   = value,
        }
        self.set_relation(name, rel);
    }

    pub fn add_points(&mut self, name: &str, ptype: PointTypes, value: PointT) {
        let mut rel = self.get_relation(name).unwrap().clone();
        match ptype {
            PointTypes::HATE    => rel.p_hate   = rel.p_hate + value,
            PointTypes::SILLY   => rel.p_silly  = rel.p_silly + value,
            PointTypes::FRIEND  => rel.p_friend = rel.p_friend + value,
            PointTypes::LOVE    => rel.p_love   = rel.p_love + value,
        }
        self.set_relation(name, rel);
    }

    pub fn sub_points(&mut self, name: &str, ptype: PointTypes, value: PointT) {
        let mut rel = self.get_relation(name).unwrap().clone();
        match ptype {
            PointTypes::HATE    => rel.p_hate   = rel.p_hate - value,
            PointTypes::SILLY   => rel.p_silly  = rel.p_silly - value,
            PointTypes::FRIEND  => rel.p_friend = rel.p_friend - value,
            PointTypes::LOVE    => rel.p_love   = rel.p_love - value,
        }
        self.set_relation(name, rel);
    }

    pub fn get_points(&mut self, name: &str, ptype: PointTypes) -> PointT {
        let points: PointT;
        let rel = self.get_relation(name).unwrap();
        match ptype {
            PointTypes::HATE    => points = rel.p_hate,
            PointTypes::SILLY   => points = rel.p_silly,
            PointTypes::FRIEND  => points = rel.p_friend,
            PointTypes::LOVE    => points = rel.p_love,
        }
        points
    }

}
