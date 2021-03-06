use pest::Parser;

use crate::parser::build_ast;
use crate::std_t::Builtin;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;
use std::rc::Rc;

use crate::errors::*;
use crate::expr::ident::Ident;
use crate::std_t::function::BuiltinFunction;
use crate::value::Function;
use crate::value::Type;
use crate::value::Value;
use crate::value::Var;
use crate::parser::ExprParser;

pub trait Evaluateur {
    fn eval(&self, vm: &mut Vm) -> Result<Value, Error>;
}

pub fn function<T: Evaluateur + 'static + Debug>(body: T) -> Function {
    Function(Rc::new(
        move |args: HashMap<String, Var>, vm: Vm| -> Result<Value, Error> {
            let mut vm = vm.clone();
            for i in args.iter() {
                vm.set_ident(Ident(i.0.clone()), i.1.clone());
            }
            body.eval(&mut vm)
        },
    ))
}

#[derive(Debug, Clone)]
pub struct Vm(pub std::collections::HashMap<Ident, Var>);

impl Vm {
    pub fn new() -> Self {
        let mut vm = Vm(HashMap::new());
        vm.use_builtin_function();
        vm
    }

    pub fn use_builtin_function(&mut self) {
        let map = BuiltinFunction::build();
        for i in map.iter() {
            self.set_ident(
                Ident(i.0.clone()),
                Var {
                    value: Value::Function {
                        name: i.0.clone(),
                        func: Function(i.1 .0.clone()),
                        args: i.1 .1.clone(),
                    },
                    type_: Type::Func,
                    mutable: false,
                },
            );
        }
    }

    pub fn set_ident(&mut self, ident: Ident, value: Var) {
        self.0.insert(ident.clone(), value);
    }

    pub fn get_ident(&self, ident: Ident) -> Option<&Var> {
        self.0.get(&ident)
    }

    pub fn iadd(&mut self, a: String, b: Value) -> Result<Value, Error> {
        match b {
            Value::Number(b) => {
                if self.exists(Ident(a.clone())) {
                    let v = self.get_ident(Ident(a.clone())).unwrap().clone();
                    if !v.mutable {
                        return Err(Error::ItsAConstant(ItsAConstantError { var_name: a }));
                    }

                    let r = match v.value {
                        Value::Number(n) => {
                            self.set_ident(
                                Ident(a),
                                Var {
                                    value: Value::Number(n + b),
                                    type_: v.clone().type_,
                                    mutable: v.clone().mutable,
                                },
                            );
                            Ok(Value::None)
                        }
                        _ => Err(Error::TypeMismatch(TypeMismatchError {
                            expected: Type::Int,
                            found: v.value.get_type(),
                        })),
                    }?;
                    if r.get_type() != v.clone().type_ {
                        return Err(Error::TypeMismatch(TypeMismatchError {
                            expected: v.type_,
                            found: r.get_type(),
                        }));
                    } else {
                        Ok(r)
                    }
                } else {
                    return Err(Error::VarNotFound(VarNotFoundError { var_name: a }));
                }
            }
            _ => Err(Error::TypeMismatch(TypeMismatchError {
                expected: Type::Int,
                found: Type::None,
            })),
        }
    }

    pub fn isub(&mut self, a: String, b: Value) -> Result<Value, Error> {
        match b {
            Value::Number(b) => {
                if self.exists(Ident(a.clone())) {
                    let v = self.get_ident(Ident(a.clone())).unwrap().clone();
                    if !v.mutable {
                        return Err(Error::ItsAConstant(ItsAConstantError { var_name: a }));
                    }
                    let r = match v.value {
                        Value::Number(n) => {
                            self.set_ident(
                                Ident(a),
                                Var {
                                    value: Value::Number(n - b),
                                    type_: v.clone().type_,
                                    mutable: v.clone().mutable,
                                },
                            );
                            Ok(Value::None)
                        }
                        _ => Err(Error::TypeMismatch(TypeMismatchError {
                            expected: Type::Int,
                            found: v.value.get_type(),
                        })),
                    }?;
                    if r.get_type() != v.clone().type_ {
                        return Err(Error::TypeMismatch(TypeMismatchError {
                            expected: v.type_,
                            found: r.get_type(),
                        }));
                    } else {
                        Ok(r)
                    }
                } else {
                    return Err(Error::VarNotFound(VarNotFoundError { var_name: a }));
                }
            }
            _ => Err(Error::TypeMismatch(TypeMismatchError {
                expected: Type::Int,
                found: Type::None,
            })),
        }
    }

    pub fn imul(&mut self, a: String, b: Value) -> Result<Value, Error> {
        match b {
            Value::Number(b) => {
                if self.exists(Ident(a.clone())) {
                    let v = self.get_ident(Ident(a.clone())).unwrap().clone();
                    if !v.mutable {
                        return Err(Error::ItsAConstant(ItsAConstantError { var_name: a }));
                    }
                    let r = match v.value {
                        Value::Number(n) => {
                            self.set_ident(
                                Ident(a),
                                Var {
                                    value: Value::Number(n * b),
                                    type_: v.clone().type_,
                                    mutable: v.clone().mutable,
                                },
                            );
                            Ok(Value::None)
                        }
                        _ => Err(Error::TypeMismatch(TypeMismatchError {
                            expected: Type::Int,
                            found: v.value.get_type(),
                        })),
                    }?;
                    if r.get_type() != v.clone().type_ {
                        return Err(Error::TypeMismatch(TypeMismatchError {
                            expected: v.type_,
                            found: r.get_type(),
                        }));
                    } else {
                        Ok(r)
                    }
                } else {
                    return Err(Error::VarNotFound(VarNotFoundError { var_name: a }));
                }
            }
            _ => Err(Error::TypeMismatch(TypeMismatchError {
                expected: Type::Int,
                found: Type::None,
            })),
        }
    }

    pub fn idiv(&mut self, a: String, b: Value) -> Result<Value, Error> {
        match b {
            Value::Number(b) => {
                if self.exists(Ident(a.clone())) {
                    let v = self.get_ident(Ident(a.clone())).unwrap().clone();
                    if !v.mutable {
                        return Err(Error::ItsAConstant(ItsAConstantError { var_name: a }));
                    }
                    let r = match v.value {
                        Value::Number(n) => {
                            self.set_ident(
                                Ident(a),
                                Var {
                                    value: Value::Number(n / b),
                                    type_: v.clone().type_,
                                    mutable: v.clone().mutable,
                                },
                            );
                            Ok(Value::None)
                        }
                        _ => Err(Error::TypeMismatch(TypeMismatchError {
                            expected: Type::Int,
                            found: v.value.get_type(),
                        })),
                    }?;
                    if r.get_type() != v.clone().type_ {
                        return Err(Error::TypeMismatch(TypeMismatchError {
                            expected: v.type_,
                            found: r.get_type(),
                        }));
                    } else {
                        Ok(r)
                    }
                } else {
                    return Err(Error::VarNotFound(VarNotFoundError { var_name: a }));
                }
            }
            _ => Err(Error::TypeMismatch(TypeMismatchError {
                expected: Type::Int,
                found: Type::None,
            })),
        }
    }

    pub fn exists(&self, ident: Ident) -> bool {
        self.0.contains_key(&ident)
    }
}

pub fn execute_file(file: &str) -> Result<Vm, String> {
    let content = fs::read_to_string(file).unwrap();
    let mut result = ExprParser::parse(crate::parser::Rule::program, &content);
    let mut vm = Vm::new();
    match result {
        Ok(ref mut e) => {
            for rule in e {
                
                match build_ast(rule) {
                    Ok(ast) => {
                        match ast.eval(&mut vm) {
                            Ok(e) => println!("[DEBUG] line 263 file 'mod.rs'(vm) : result = {:#?}", e),
                            Err(e) => {
                                println!("[ERROR] line 263 file 'mod.rs'(vm) : {:#?}", e);
                                return Err(format!("{:#?}", e));
                            }
                        }
                    }
                    Err(e) => {
                        return Err(e);
                    }
                };
            };

        },
        Err(e) => return Err(e.to_string())
    };
    Ok(vm)
}