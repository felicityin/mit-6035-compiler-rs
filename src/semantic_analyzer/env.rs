use std::collections::HashMap;

use super::ir::{ For, While, IfElse, MethodDecl, VarDecl };
use super::errors::SemanticCheckError;

pub struct EnvStack {
    pub envs: Vec<Env>,
    pub methods: HashMap<String, MethodDecl>,
}

pub struct Env {
    pub type_: EnvType,
    pub table: HashMap<String, VarDecl>,
}

#[derive(Debug, Clone)]
pub enum EnvType {
    Global,
    Method(MethodDecl),
    For(For),
    While(While),
    If(IfElse),
    Else(IfElse),
}

impl EnvStack {
    pub fn new() -> Self {
        Self {
            envs: Vec::new(),
            methods: HashMap::new(),
        }
    }

    pub fn push(&mut self, t: EnvType) {
        self.envs.push(Env::new(t));
    }

    pub fn pop(&mut self) {
        self.envs.pop();
    }

    pub fn add_var(&mut self, v: &VarDecl) -> Result<(), SemanticCheckError> {
        let var = v.clone();

        if var.borrow().arr_len.is_some() && var.borrow().arr_len.unwrap() <= 0 {
            return Err(SemanticCheckError::ArrayLenShouldPositive(var.borrow().id.clone()));
        }

        println!("{:?}", v.borrow().id.clone());
        match self.envs.last_mut().unwrap().table.insert(v.borrow().id.clone(), var) {
            Some(e) => Err(SemanticCheckError::DuplicatedVar(e)),
            None => Ok(()),
        }
    }

    pub fn add_method(&mut self, m: &MethodDecl) -> Result<(), SemanticCheckError> {
        let method = m.clone();
        match self.methods.insert(m.borrow().name.clone(), method) {
            Some(e) => Err(SemanticCheckError::DuplicatedMethod(e)),
            None => Ok(()),
        }
    }

    pub fn get_var_decl(&self, name: &String) -> Option<VarDecl> {
        for env in self.envs.iter().rev() {
            match env.table.get(name) {
                Some(v) => return Some(v.clone()),
                None => (),
            }
        }
        None
    }

    pub fn get_method_decl(&self, name: &String) -> Option<MethodDecl> {
        self.methods.get(name).cloned()
    }

    pub fn get_cur_scope_method_decl(&self) -> Option<MethodDecl> {
        for env in self.envs.iter().rev() {
            if let EnvType::Method(m) = &env.type_ {
                return Some(m.clone());
            }
        }
        None
    }

    pub fn get_cur_scope_for(&self) -> Option<For> {
        for env in self.envs.iter().rev() {
            if let EnvType::For(f) = &env.type_ {
                return Some(f.clone());
            }
        }
        None
    }

    pub fn get_cur_scope_while(&self) -> Option<While> {
        for env in self.envs.iter().rev() {
            if let EnvType::While(w) = &env.type_ {
                return Some(w.clone());
            }
        }
        None
    }
}

impl Env {
    pub fn new(t: EnvType) -> Self {
        Self {
            type_: t,
            table: HashMap::new(),
        }
    }
}
