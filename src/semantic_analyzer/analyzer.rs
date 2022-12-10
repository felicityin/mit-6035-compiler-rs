use std::cell::RefCell;
use std::rc::Rc;

use crate::ast;

use super::env::{ EnvStack, EnvType };
use super::ir;
use super::errors::{ IRResult, SemanticCheckError };
use super::pre_check::{ check_main };

pub struct SemanticAnalyzer {
    envs: RefCell<EnvStack>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            envs: RefCell::new(EnvStack::new()),
        }
    }

    pub fn create_ir(&self, p: ast::Program) -> IRResult<ir::IRRoot> {
        if let Err(errors) = self.pre_check(&p) {
            return Err(errors);
        }
        return self.construct_ir(p);
    }

    pub fn pre_check(&self, p: &ast::Program) -> IRResult<()> {
        let passes = Vec::from([
            check_main,
        ]);
        let errors: Vec<SemanticCheckError> = passes
            .iter()
            .map(|f| f(p))
            .filter(|e| e.is_err())
            .map(|e| e.err().unwrap())
            .collect();
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn construct_ir(&self, p: ast::Program) -> IRResult<ir::IRRoot> {
        self.envs.borrow_mut().push(EnvType::Global);
        let mut errors = Vec::new();

        let import_decls = self.get_ir_import_decls(p.import_decls);
        let import_decls = match import_decls {
            Err(e) => {
                errors.extend(e);
                None
            },
            Ok(imports) => Some(imports),
        };

        let field_decls = self.get_ir_field_decls(p.field_decls);
        let field_decls = match field_decls {
            Err(e) => {
                errors.extend(e);
                None
            },
            Ok(fields) => Some(fields),
        };

        let method_decls = self.get_ir_method_decls(p.method_decls);
        let method_decls = match method_decls {
            Err(e) => {
                errors.extend(e);
                None
            },
            Ok(methods) => Some(methods),
        };

        self.envs.borrow_mut().pop();

        if errors.is_empty() {
            Ok(ir::IRRoot {
                root: ir::ProgramDecl {
                    import_decls: import_decls.unwrap(),
                    field_decls: field_decls.unwrap(),
                    method_decls: method_decls.unwrap(),
                }
            })
        } else {
            Err(errors)
        }
    }

    fn get_ir_import_decls(&self, imports: Vec<ast::ImportDecl>) -> IRResult<Vec<ir::VarDecl>> {
        let mut errors = Vec::new();
        let mut imports_decls = Vec::new();

        for import_decl in imports {
            let import = create_rc(ir::VarDecl0 {
                type_: ir::Type::Int,
                id: import_decl.id.clone(),
                arr_len: None,
            });
            if let Err(e) = self.envs.borrow_mut().add_var(&import) {
                errors.push(e);
            } else {
                imports_decls.push(import);
            }
        }

        if errors.is_empty() {
            Ok(imports_decls)
        } else {
            Err(errors)
        }
    }

    fn get_ir_field_decls(&self, fields: Vec<ast::FieldDecl>) -> IRResult<Vec<ir::VarDecl>> {
        let mut errors = Vec::new();
        let mut field_decls: Vec<ir::VarDecl> = Vec::new();

        for ids in fields {
            for id in ids.field_ids {
                let var = create_rc(ir::VarDecl0 {
                    type_: ir::Type::from(&ids.type_),
                    id: id.id,
                    arr_len: id.arr_len,
                });
                if let Err(e) = self.envs.borrow_mut().add_var(&var) {
                    errors.push(e);
                } else {
                    field_decls.push(var);
                }
            }
        }

        if errors.is_empty() {
            Ok(field_decls)
        } else {
            Err(errors)
        }
    }

    fn get_ir_method_decls(&self, methods: Vec<ast::MethodDecl>) -> IRResult<Vec<ir::MethodDecl>> {
        let mut errors = Vec::new();
        let mut method_decls: Vec<ir::MethodDecl> = Vec::new();

        for method in methods {
            match self.get_ir_method_decl(method) {
                Err(e) => errors.extend(e),
                Ok(m) => method_decls.push(m),
            }
        }
        
        if errors.is_empty() {
            Ok(method_decls)
        } else {
            Err(errors)
        }
    }

    fn get_ir_method_decl(&self, method: ast::MethodDecl) -> IRResult<ir::MethodDecl> {
        let mut errors = Vec::new();
        let args: Vec<ir::VarDecl> = method.args.iter().map(|arg| self.get_ir_method_arg(arg)).collect();

        let method_decl = create_rc(ir::MethodDecl0 {
            return_type: ir::ReturnType::from(&method.return_type),
            name: method.id.clone(),
            args: args.clone(),
            block: None,
        });

        if let Err(e) = self.envs.borrow_mut().add_method(&method_decl) {
            errors.push(e);
        }

        self.envs.borrow_mut().push(EnvType::Method(method_decl.clone()));

        for arg in &method_decl.borrow().args {
            if let Err(e) = self.envs.borrow_mut().add_var(arg) {
                errors.push(e);
            }
        }

        let block = self.get_ir_block(method.block);
        if let Err(e) = block {
            errors.extend(e);
            return Err(errors);
        }

        self.envs.borrow_mut().pop();

        Ok(create_rc(ir::MethodDecl0 {
            return_type: ir::ReturnType::from(&method.return_type),
            name: method.id.clone(),
            args,
            block: Some(block.unwrap()),
        }))
    }

    fn get_ir_method_arg(&self, arg: &ast::MethodArg) -> ir::VarDecl {
        create_rc(ir::VarDecl0 {
            type_: ir::Type::from(&arg.type_),
            id: arg.id.clone(),
            arr_len: None
        })
    }

    fn get_ir_block(&self, block: ast::Block) -> IRResult<ir::Block> {
        let mut errors = Vec::new();

        let field_decls = match self.get_ir_field_decls(block.field_decls) {
            Err(e) => {
                errors.extend(e);
                None
            },
            Ok(f) => Some(f),
        };

        let mut statements = Vec::new();
        for statement in block.statements {
            match self.get_ir_statement(statement) {
                Err(e) => errors.extend(e),
                Ok(s) => statements.push(s),
            }
        }

        if errors.is_empty() {
            Ok(ir::Block {
                field_decls: field_decls.unwrap(),
                statements,
            })
        } else {
            Err(errors)
        }
    }

    fn get_ir_statement(&self, statement: ast::Statement) -> IRResult<ir::Statement> {
        match statement {
            ast::Statement::Assign(assign) => match self.get_ir_assign(assign) {
                Ok(a) => Ok(create_rc(ir::Statement0::Assign(a))),
                Err(e) => Err(e),
            },
            ast::Statement::Break => match self.get_ir_break() {
                Ok(b) => Ok(create_rc(ir::Statement0::Break(b))),
                Err(e) => Err(e),
            },
            ast::Statement::Continue => match self.get_ir_continue() {
                Ok(c) => Ok(create_rc(ir::Statement0::Continue(c))),
                Err(e) => Err(e),
            },
            ast::Statement::IfElse(if_else) => match self.get_ir_if_else(if_else) {
                Ok(ie) => Ok(create_rc(ir::Statement0::IfElse(ie))),
                Err(e) => Err(e),
            },
            ast::Statement::Loop(fo) => match self.get_ir_for(fo) {
                Ok(f) => Ok(create_rc(ir::Statement0::For(f))),
                Err(e) => Err(e),
            },
            ast::Statement::MethodCall(method) => match self.get_ir_method_call(method) {
                Ok(m) => Ok(create_rc(ir::Statement0::MethodCall(m))),
                Err(e) => Err(e),
            },
            ast::Statement::Return(ret) => match self.get_ir_return(ret) {
                Ok(r) => Ok(create_rc(ir::Statement0::Return(r))),
                Err(e) => Err(e),
            },
            ast::Statement::While(whl) => match self.get_ir_while(whl) {
                Ok(w) => Ok(create_rc(ir::Statement0::While(w))),
                Err(e) => Err(e),
            },
        }
    }

    fn get_ir_assign(&self, assign: ast::Assign) -> IRResult<ir::Assign> {
        let dst = match self.get_ir_location(assign.dst) {
            Ok(d) => d,
            Err(e) => {
                return Err(e);
            }
        };

        let assign_expr = match self.get_ir_assign_expr(assign.assign_expr) {
            Ok(a) => a,
            Err(e) => {
                return Err(e);
            }
        };

        match assign_expr.clone() {
            ir::AssignExpr::AssignOpExpr(a) => {
                match a.assign_op {
                    ir::AssignOp::CompoundAssignOp(_) => {
                        if dst.id.borrow().type_ != ir::Type::Int {
                            return Err(vec![SemanticCheckError::OperandsTypeMismatch]);
                        }
                    },
                    ir::AssignOp::Assign => {
                        if dst.id.borrow().type_ != a.expr.borrow().type_ {
                            return Err(vec![SemanticCheckError::OperandsTypeMismatch]);
                        }
                    },
                }
            },
            ir::AssignExpr::Increment(_) => {
                if dst.id.borrow().type_ != ir::Type::Int {
                    return Err(vec![SemanticCheckError::OperandsTypeMismatch]);
                }
            },
        }

        Ok(ir::Assign {
            dst,
            assign_expr,
        })
    }

    fn get_ir_location(&self, dst: ast::Location) -> IRResult<ir::Location> {
        let mut errors = Vec::new();

        let id = match dst.clone() {
            ast::Location::Id(id) => id,
            ast::Location::IdExpr(arr) => arr.id,
        };

        let var = self.envs.borrow().get_var_decl(&id);
        if var.is_none() {
            errors.push(SemanticCheckError::UsedBeforeDeclared(id.clone()));
            return Err(errors);
        }
        let var = var.unwrap();

        let array_len = match dst {
            ast::Location::Id(_) => {
                if var.borrow().arr_len.is_some() {
                    errors.push(SemanticCheckError::LocationTypeMismatch);
                }
                None
            }
            ast::Location::IdExpr(arr) => {
                match self.get_ir_expr(arr.array_len.clone()) {
                    Ok(i) => {
                        let type_ = i.borrow().type_;
                        let arr_len = &var.borrow().arr_len;
                        if type_ == ir::Type::Int {
                            if arr_len.is_some() {
                                Some(i)
                            } else {
                                errors.push(SemanticCheckError::LocationTypeMismatch);
                                None
                            }
                        } else {
                            errors.push(SemanticCheckError::TypeOfExprMustInt(id.clone()));
                            None
                        }
                    }
                    Err(e) => {
                        errors.extend(e);
                        None
                    }
                }
            }
        };

        if errors.is_empty() {
            Ok(ir::Location {
                id: var,
                array_len,
            })
        } else {
            Err(errors)
        }
    }

    fn get_ir_expr(&self, expr: ast::Expr) -> IRResult<ir::Expr> {    
        let expr = match *expr {
            ast::Expr_::Location(l) => match self.get_ir_location(l) {
                Ok(l) => ir::ExprType::Location(l),
                Err(e) => return Err(e),
            },
            ast::Expr_::MethodCall(m) => match self.get_ir_method_call(m) {
                Ok(m) => ir::ExprType::MethodCall(m),
                Err(e) => return Err(e),
            },
            ast::Expr_::Literal(l) => ir::ExprType::Literal(ir::Literal::from(&l)),
            ast::Expr_::LenId(l) => match self.get_ir_len_id(l) {
                Ok(l) => ir::ExprType::LenId(l),
                Err(e) => return Err(e),
            },
            ast::Expr_::Binary(b) => match self.get_ir_binary(b) {
                Ok(b) => ir::ExprType::Binary(b),
                Err(e) => return Err(e),
            },
            ast::Expr_::Unary(u) => match self.get_ir_unary(u) {
                Ok(u) => ir::ExprType::Unary(u),
                Err(e) => return Err(e),
            }
        };

        let type_ = self.get_ir_expr_type(&expr);

        Ok(create_rc(ir::Expr0 {
            type_,
            expr,
        }))
    }

    fn get_ir_expr_type(&self, e: &ir::ExprType) -> ir::Type {
        match e {
            ir::ExprType::Location(l) => l.id.borrow().type_,
            ir::ExprType::MethodCall(m) => match m {
                ir::MethodCall::Method(m) => match m.decl.borrow().return_type {
                    ir::ReturnType::Void => ir::Type::Void,
                    ir::ReturnType::Type(t) => t,
                }
                ir::MethodCall::Callout(_) => ir::Type::Int,
            },
            ir::ExprType::Literal(l) => match l {
                ir::Literal::IntLiteral(_) => ir::Type::Int,
                ir::Literal::BoolLiteral(_) => ir::Type::Bool,
                ir::Literal::CharLiteral(_) => ir::Type::Char,
            }
            ir::ExprType::LenId(_) => ir::Type::Int,
            ir::ExprType::Binary(b) => b.op.get_return_type(),
            ir::ExprType::Unary(u) => u.expr.borrow().type_,
        }
    }

    fn get_ir_len_id(&self, id: String) -> IRResult<String> {
        let mut errors = Vec::new();
        
        let var = self.envs.borrow().get_var_decl(&id);
        if var.is_none() {
            errors.push(SemanticCheckError::UsedBeforeDeclared(id));
            return Err(errors);
        }
        let var = var.unwrap();

        let arr_len = &var.borrow().arr_len;
        if arr_len.is_some() {
            Ok(id)
        } else {
            errors.push(SemanticCheckError::IdMustArray(var.borrow().id.clone()));
            Err(errors)
        }
    }

    fn get_ir_binary(&self, binary: ast::Binary) -> IRResult<ir::Binary> {
        let mut errors = Vec::new();
        let lhs = match self.get_ir_expr(binary.lhs) {
            Ok(l) => Some(l),
            Err(e) => {
                errors.extend(e);
                None
            }
        };
        let rhs = match self.get_ir_expr(binary.rhs) {
            Ok(r) => Some(r),
            Err(e) => {
                errors.extend(e);
                None
            }
        };

        if !errors.is_empty() {
            return Err(errors);
        }

        let lhs = lhs.unwrap();
        let rhs = rhs.unwrap();

        if lhs.borrow().type_ != rhs.borrow().type_ {
            errors.push(SemanticCheckError::OperandsTypeMismatch);
            return Err(errors);
        }

        let op = ir::BinaryOp::from(&binary.op);
        let operand_type = lhs.borrow().type_;
        match op {
            ir::BinaryOp::Add
            | ir::BinaryOp::Sub
            | ir::BinaryOp::Mul
            | ir::BinaryOp::Div
            | ir::BinaryOp::Mod
            | ir::BinaryOp::GT
            | ir::BinaryOp::GE
            | ir::BinaryOp::LT
            | ir::BinaryOp::LE
                if operand_type == ir::Type::Int => (),
            ir::BinaryOp::EQ | ir::BinaryOp::NE => (),
            ir::BinaryOp::Or | ir::BinaryOp::And if operand_type == ir::Type::Bool => (),
            _ => {
                errors.push(SemanticCheckError::OperandsTypeMismatch);
                return Err(errors);
            }
        }
        Ok(ir::Binary{ lhs, rhs, op })
    }

    fn get_ir_unary(&self, unary: ast::Unary) -> IRResult<ir::Unary> {
        let expr = match self.get_ir_expr(unary.expr) {
            Ok(u) => u,
            Err(e) => return Err(e),
        };
        let type_ = expr.borrow().type_;
        match unary.op {
            ast::UnaryOp::NegInt if type_ == ir::Type::Int => Ok(ir::Unary {
                expr,
                op: ir::UnaryOp::NegInt,
            }),
            ast::UnaryOp::NegBool if type_ == ir::Type::Bool => Ok(ir::Unary {
                expr,
                op: ir::UnaryOp::NegBool,
            }),
            _ => Err(vec![SemanticCheckError::OperandsTypeMismatch])
        }
    }

    fn get_ir_assign_expr(&self, expr: ast::AssignExpr) -> IRResult<ir::AssignExpr> {
        match expr {
            ast::AssignExpr::AssignOpExpr(a) => {
                let op = ir::AssignOp::from(&a.assign_op);
                let expr = match self.get_ir_expr(a.expr) {
                    Ok(expr) => expr,
                    Err(e) => return Err(e),
                };
                Ok(ir::AssignExpr::AssignOpExpr(ir::AssignOpExpr {
                    assign_op: op,
                    expr,
                }))
            }
            ast::AssignExpr::Increment(inc) => Ok(ir::AssignExpr::Increment(ir::Increment::from(&inc))),
        }
    }

    fn get_ir_break(&self) -> IRResult<ir::Break> {
        let for_ = self.envs.borrow().get_cur_scope_for();
        let while_ = self.envs.borrow().get_cur_scope_while();

        if for_.is_none() && while_.is_none() {
            Err(vec![SemanticCheckError::InvalidBreak])
        } else if !for_.is_none() {
            Ok(ir::Break::For(for_.unwrap()))
        } else {
            Ok(ir::Break::While(while_.unwrap()))
        }
    }

    fn get_ir_continue(&self) -> IRResult<ir::Continue> {
        let for_ = self.envs.borrow().get_cur_scope_for();
        let while_ = self.envs.borrow().get_cur_scope_while();

        if for_.is_none() && while_.is_none() {
            Err(vec![SemanticCheckError::InvalidContinue])
        } else if !for_.is_none() {
            Ok(ir::Continue::For(for_.unwrap()))
        } else {
            Ok(ir::Continue::While(while_.unwrap()))
        }
    }

    fn get_ir_if_else(&self, if_else: ast::IfElse) -> IRResult<ir::IfElse> {
        let cond = match self.get_ir_expr(if_else.cond) {
            Ok(c) => c,
            Err(e) => return Err(e),
        };

        if cond.borrow().type_ != ir::Type::Bool {
            return Err(vec![SemanticCheckError::ConditionTypeShouldBool]);
        }

        let tmp = create_rc(ir::IfElse0 {
            cond: cond.clone(),
            if_block: None,
            else_block: None,
        });

        self.envs.borrow_mut().push(EnvType::If(tmp.clone()));

        let if_block = match self.get_ir_block(if_else.if_block) {
            Ok(b) => b,
            Err(e) => return Err(e),
        };

        self.envs.borrow_mut().pop();
        self.envs.borrow_mut().push(EnvType::Else(tmp.clone()));

        let else_block = match if_else.else_block {
            Some(b) => {
                match self.get_ir_block(b) {
                    Ok(b) => Some(b),
                    Err(e) => return Err(e),
                }
            }
            None => None,
        };

        self.envs.borrow_mut().pop();

        Ok(create_rc(ir::IfElse0 {
            cond,
            if_block: Some(if_block),
            else_block,
        }))
    }

    fn get_ir_for(&self, lop: ast::Loop) -> IRResult<ir::For> {
        let id = match self.envs.borrow().get_var_decl(&lop.id) {
            Some(id) => id,
            None => return Err(vec![SemanticCheckError::UsedBeforeDeclared(lop.id.clone())]),
        };

        let init_expr = match self.get_ir_expr(lop.init_expr) {
            Ok(i) => i,
            Err(e) => return Err(e),
        };
        if init_expr.borrow().type_ != ir::Type::Int {
            return Err(vec![SemanticCheckError::OperandsTypeMismatch]);
        }

        let incre_expr = match self.get_ir_expr(lop.incre_expr) {
            Ok(i) => i,
            Err(e) => return Err(e),
        };

        let update = match self.get_ir_for_update(lop.update) {
            Ok(u) => u,
            Err(e) => return Err(e),
        };

        let for_ = create_rc(ir::For0 {
            id: id.clone(),
            init_expr: init_expr.clone(),
            incre_expr: incre_expr.clone(),
            update: update.clone(),
            block: None,
        });

        self.envs.borrow_mut().push(EnvType::For(for_.clone()));

        let block = match self.get_ir_block(lop.block) {
            Ok(b) => b,
            Err(e) => return Err(e),
        };

        self.envs.borrow_mut().pop();

        Ok(create_rc(ir::For0 {
            id,
            init_expr,
            incre_expr,
            update,
            block: Some(block),
        }))
    }

    fn get_ir_for_update(&self, update: ast::ForUpdate) -> IRResult<ir::ForUpdate> {
        let id = match self.get_ir_location(update.location) {
            Ok(l) => l,
            Err(e) => return Err(e),
        };

        let update_expr = match update.update_expr {
            ast::ForUpdateExpr::AssignExpr(a) => {
                let compound_assign_op = ir::CompoundAssignOp::from(&a.compound_assign_op);
                let expr = match self.get_ir_expr(a.expr) {
                    Ok(expr) => expr,
                    Err(e) => return Err(e),
                };
                ir::ForUpdateExpr::AssignExpr(ir::ForUpdateAssignExpr { compound_assign_op, expr })
            },
            ast::ForUpdateExpr::Increment(i) => {
                ir::ForUpdateExpr::Increment(ir::Increment::from(&i))
            },
        };

        Ok(ir::ForUpdate {
            id,
            update_expr
        })
    }

    fn get_ir_method_call(&self, method: ast::MethodCall) -> IRResult<ir::MethodCall> {
        match method {
            ast::MethodCall::Method(m) => match self.get_ir_method(m) {
                Ok(m) => Ok(ir::MethodCall::Method(m)),
                Err(e) => Err(e),
            },
            ast::MethodCall::Callout(c) => match self.get_ir_callout(c) {
                Ok(c) => Ok(ir::MethodCall::Callout(c)),
                Err(e) => Err(e),
            }
        }
    }

    fn get_ir_method(&self, method: ast::MethodCall0) -> IRResult<ir::MethodCall0> {
        let decl = match self.envs.borrow().get_method_decl(&method.name.id) {
            Some(m) => m,
            None => return Err(vec![SemanticCheckError::UsedBeforeDeclared(method.name.id.clone())]),
        };

        let mut args = Vec::new();
        for arg in method.args {
            match self.get_ir_expr(arg) {
                Ok(a) => args.push(a),
                Err(e) => return Err(e),
            }
        }

        if args.len() != decl.borrow().args.len() {
            return Err(vec![SemanticCheckError::MethodSignatureMismatch]);
        }

        for (arg, arg_decl) in args.iter().zip(decl.borrow().args.iter()) {
            if arg.borrow().type_ != arg_decl.borrow().type_ {
                return Err(vec![SemanticCheckError::InvalidMethodArgs]);
            }
        }

        Ok(ir::MethodCall0 {
            decl,
            args,
        })
    }

    fn get_ir_callout(&self, callout: ast::MethodCall1) -> IRResult<ir::MethodCall1> {
        let mut args = Vec::new();
        for arg in callout.args {
            match arg {
                ast::ImportArg::Expr(e) => {
                    match self.get_ir_expr(e) {
                        Ok(a) => args.push(ir::ImportArg::Expr(a)),
                        Err(e) => return Err(e),
                    }
                },
                ast::ImportArg::StringLiteral(s) => args.push(ir::ImportArg::StringLiteral(s)),
            }
        }

        Ok(ir::MethodCall1 {
            name: callout.name.id.clone(),
            args,
        })
    }

    fn get_ir_return(&self, ret: ast::Return) -> IRResult<ir::Return> {
        let val = match ret.expr {
            Some(expr) => match self.get_ir_expr(expr) {
                Ok(expr) => Some(expr),
                Err(e) => return Err(e),
            },
            None => None,
        };
        let method = match self.envs.borrow().get_cur_scope_method_decl() {
            Some(m) => m,
            None => return Err(vec![SemanticCheckError::InvalidReturn]),
        };
        let method_return_type = method.borrow().return_type.clone();
        let return_type = ir::ReturnType::to_type(&method_return_type);
        match val {
            Some(v) if method_return_type != ir::ReturnType::Void && return_type == v.borrow().type_ => Ok(ir::Return {
                func: method,
                val: Some(v),
            }),
            None if method_return_type == ir::ReturnType::Void => Ok(ir::Return {
                func: method,
                val: None,
            }),
            _ => Err(vec![SemanticCheckError::ReturnTypeMismatch]),
        }
    }

    fn get_ir_while(&self, whl: ast::While) -> IRResult<ir::While> {
        let cond = match self.get_ir_expr(whl.expr) {
            Ok(expr) => expr,
            Err(e) => return Err(e),
        };

        let w = create_rc(ir::While0 {
            cond: cond.clone(),
            block: None,
        });

        self.envs.borrow_mut().push(EnvType::While(w.clone()));

        let block = match self.get_ir_block(whl.block, ) {
            Ok(b) => b,
            Err(e) => return Err(e),
        };

        self.envs.borrow_mut().pop();

        Ok(create_rc(ir::While0 {
            cond,
            block: Some(block),
        }))
    }
}

fn create_rc<T>(x: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(x))
}
