use std::{collections::HashMap, rc::Rc, cell::RefCell};

use super::Object;

pub type Env = Rc<RefCell<Environment>>; 

#[derive(PartialEq, Debug)]
pub struct Environment{
    store: HashMap<String, Rc<Object>>,
    outer:Option<Env>
}

impl Environment{
    pub fn new()->Self{
        Self { store: HashMap::new(), outer:None }
    }

    pub fn new_enclosed_environment(outer: Env)->Self{
        let mut env =  Self::new();
        env.outer = Some(outer.clone());
        return env;
    }

    pub fn get(&self,name: &str)->Option<Rc<Object>>{
        match self.store.get(name){
            Some(obj)=>Some(Rc::clone(obj)),
            None=>{
                match &self.outer{
                    Some(outer)=>outer.borrow().get(name),
                    None=>None
                }
            }
        }
    }

    pub fn set(&mut self, name:&str, object:Rc<Object>){
        self.store.insert(name.to_string(), object);
    }
}