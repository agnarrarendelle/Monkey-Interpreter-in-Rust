use std::{collections::HashMap, rc::Rc, cell::RefCell};

use super::Object;

pub type Env = Rc<RefCell<Environment>>; 

pub struct Environment{
    store: HashMap<String, Rc<Object>>
}

impl Environment{
    pub fn new()->Self{
        Self { store: HashMap::new() }
    }

    pub fn get(&self,name: &str)->Option<Rc<Object>>{
        match self.store.get(name){
            Some(obj)=>Some(Rc::clone(obj)),
            None=>None
        }
    }

    pub fn set(&mut self, name:&str, object:Rc<Object>){
        self.store.insert(name.to_string(), object);
    }
}