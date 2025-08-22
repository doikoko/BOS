#![no_std]

pub enum Result{
    Ok,
    Err
}
impl Result{
    pub fn unwrap(&self){
        if let Self::Err = self{
            panic!("");
        }
    }
    pub fn expect(&self, msg: &str){
        if let Self::Err = self{
            panic!("{}", msg);
        }
    }
    pub fn unwrap_or_else<F: Fn()>(func: F){
        func();
    }
}