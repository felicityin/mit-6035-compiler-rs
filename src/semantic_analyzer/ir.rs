use crate::ast;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct IRRoot {
    pub root: ProgramDecl,
}

#[derive(Debug)]
pub struct ProgramDecl {
    pub import_decls: Vec<VarDecl>,
    pub field_decls: Vec<VarDecl>,
    pub method_decls: Vec<MethodDecl>,
}

pub type VarDecl = Rc<RefCell<VarDecl0>>;

#[derive(Debug)]
pub struct VarDecl0 {
    pub type_: Type,
    pub id: Id,
    pub arr_len: Option<i32>,
}

pub type MethodDecl = Rc<RefCell<MethodDecl0>>;

#[derive(Debug)]
pub struct MethodDecl0 {
    pub return_type: ReturnType,
    pub name: Id,
    pub args: Vec<VarDecl>,
    pub block: Option<Block>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReturnType {
    Type(Type),
    Void,
}

impl ReturnType {
    pub fn from(t: &ast::ReturnType) -> Self {
        match t {
            ast::ReturnType::Type(tt) => Self::Type(Type::from(tt)),
            ast::ReturnType::Void => Self::Void,
        }
    }

    pub fn to_type(t: &ReturnType) -> Type {
        match t {
            ReturnType::Type(tt) => tt.to_owned(),
            ReturnType::Void => Type::Void,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Bool,
    Char,
    Void,
}

impl Type {
    pub fn from(t: &ast::Type) -> Self {
        match t {
            ast::Type::Int => Self::Int,
            ast::Type::Bool => Self::Bool,
        }
    }
}

#[derive(Debug)]
pub struct Block {
    pub field_decls: Vec<VarDecl>,
    pub statements: Vec<Statement>,
}

pub type Statement = Rc<RefCell<Statement0>>;

#[derive(Debug)]
pub enum Statement0 {
    Assign(Assign),
    MethodCall(MethodCall),
    IfElse(IfElse),
    For(For),
    While(While),
    Return(Return),
    Break(Break),
    Continue(Continue),
}

#[derive(Debug)]
pub struct Assign {
    pub dst: Location,
    pub assign_expr: AssignExpr,
}

#[derive(Debug, Clone)]
pub struct Location {
    pub id: VarDecl,
    pub array_len: Option<Expr>,
}

#[derive(Debug, Clone)]
pub enum AssignExpr {
    AssignOpExpr(AssignOpExpr),
    Increment(Increment),
}

#[derive(Debug, Clone)]
pub struct AssignOpExpr {
    pub assign_op: AssignOp,
    pub expr: Expr,
}

#[derive(Debug, Clone)]
pub enum AssignOp {
    Assign,
    CompoundAssignOp(CompoundAssignOp),
}

impl AssignOp {
    pub fn from(t: &ast::AssignOp) -> Self {
        match t {
            ast::AssignOp::Assign => Self::Assign,
            ast::AssignOp::CompoundAssignOp(tt) => Self::CompoundAssignOp(CompoundAssignOp::from(tt)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum CompoundAssignOp {
    AddAssign,
    SubAssign,
}

impl CompoundAssignOp {
    pub fn from(t: &ast::CompoundAssignOp) -> Self {
        match t {
            ast::CompoundAssignOp::AddAssign => Self::AddAssign,
            ast::CompoundAssignOp::SubAssign => Self::SubAssign,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Increment {
    SelfAdd,
    SelfSub,
}

impl Increment {
    pub fn from(i: &ast::Increment) -> Self {
        match i {
            ast::Increment::SelfAdd => Self::SelfAdd,
            ast::Increment::SelfSub => Self::SelfSub,
        }
    }
}

#[derive(Debug)]
pub enum MethodCall {
    Method(MethodCall0),
    Callout(MethodCall1),
}

#[derive(Debug)]
pub struct MethodCall0 {
    pub decl: MethodDecl,
    pub args: Vec<Expr>,
}

#[derive(Debug)]
pub struct MethodCall1 {
    pub name: String,
    pub args: Vec<ImportArg>,
}

#[derive(Debug)]
pub enum ImportArg {
    Expr(Expr),
    StringLiteral(String),
}

pub type IfElse = Rc<RefCell<IfElse0>>;

#[derive(Debug)]
pub struct IfElse0 {
    pub cond: Expr,
    pub if_block: Option<Block>,
    pub else_block: Option<Block>,
}

pub type For = Rc<RefCell<For0>>;

#[derive(Debug)]
pub struct For0 {
    pub id: VarDecl,
    pub init_expr: Expr,
    pub incre_expr: Expr,
    pub update: ForUpdate,
    pub block: Option<Block>,
}

#[derive(Debug, Clone)]
pub struct ForUpdate {
    pub id: Location,
    pub update_expr: ForUpdateExpr,
}

#[derive(Debug, Clone)]
pub enum ForUpdateExpr {
    AssignExpr(ForUpdateAssignExpr),
    Increment(Increment),
}

#[derive(Debug, Clone)]
pub struct ForUpdateAssignExpr {
    pub compound_assign_op: CompoundAssignOp,
    pub expr: Expr,
}

pub type While = Rc<RefCell<While0>>;

#[derive(Debug)]
pub struct While0 {
    pub cond: Expr,
    pub block: Option<Block>,
}

#[derive(Debug)]
pub struct Return {
    pub func: MethodDecl,
    pub val: Option<Expr>,
}

#[derive(Debug)]
pub enum Break {
    For(For),
    While(While),
}

#[derive(Debug)]
pub enum Continue {
    For(For),
    While(While),
}

pub type Expr = Rc<RefCell<Expr0>>;

#[derive(Debug)]
pub struct Expr0 {
    pub type_: Type,
    pub expr: ExprType,
}

#[derive(Debug)]
pub enum ExprType {
    Location(Location),
    MethodCall(MethodCall),
    Literal(Literal),
    LenId(Id),
    Unary(Unary),
    Binary(Binary),
}

#[derive(Debug)]
pub struct Unary {
    pub expr: Expr,
    pub op: UnaryOp,
}

#[derive(Debug)]
pub enum UnaryOp {
    NegInt,
    NegBool,
}

#[derive(Debug)]
pub struct Binary {
    pub lhs: Expr,
    pub rhs: Expr,
    pub op: BinaryOp,
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    // ArithOp
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    // RelOp
    GT,
    LT,
    LE,
    GE,
    // EqOp
    EQ,
    NE,
    // CondOp
    And,
    Or,
}

impl BinaryOp {
    pub fn from(t: &ast::BinaryOp) -> Self {
        match t {
            ast::BinaryOp::ArithOp(a) => match a {
                ast::ArithOp::Add => BinaryOp::Add,
                ast::ArithOp::Sub => BinaryOp::Sub,
                ast::ArithOp::Mul => BinaryOp::Mul,
                ast::ArithOp::Div => BinaryOp::Div,
                ast::ArithOp::Mod => BinaryOp::Mod,
            },
            ast::BinaryOp::RelOp(a) => match a {
                ast::RelOp::Greater => BinaryOp::GT,
                ast::RelOp::GreaterEq => BinaryOp::GE,
                ast::RelOp::Less => BinaryOp::LT,
                ast::RelOp::LessEq => BinaryOp::LE,
            },
            ast::BinaryOp::EqOp(a) => match a {
                ast::EqOp::EQ => BinaryOp::EQ,
                ast::EqOp::NE => BinaryOp::NE,
            },
            ast::BinaryOp::CondOp(a) => match a {
                ast::CondOp::Or => BinaryOp::Or,
                ast::CondOp::And => BinaryOp::And,
            },
        }
    }

    pub fn get_return_type(&self) -> Type {
        match self {
            Self::Or  => Type::Bool, // logical or
            Self::And => Type::Bool, // logical and
            Self::EQ  => Type::Bool, // ==
            Self::NE  => Type::Bool, // !=
            Self::GT  => Type::Bool, // >
            Self::LT  => Type::Bool, // <
            Self::GE  => Type::Bool, // >=
            Self::LE  => Type::Bool, // <=
            Self::Add => Type::Int, // +
            Self::Sub => Type::Int, // -
            Self::Mul => Type::Int, // *
            Self::Div => Type::Int, // /
            Self::Mod => Type::Int, // %
        } 
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    IntLiteral(i32),
    CharLiteral(char),
    BoolLiteral(bool),
}

impl Literal {
    pub fn from(l: &ast::Literal) -> Self {
        match l {
            ast::Literal::IntLiteral(i) => Literal::IntLiteral(i.to_owned()),
            ast::Literal::BoolLiteral(b) => match b {
                ast::BoolLiteral::True => Literal::BoolLiteral(true),
                ast::BoolLiteral::False => Literal::BoolLiteral(false),
            },
            ast::Literal::CharLiteral(c) => Literal::CharLiteral(c.to_owned()),
        }
    }
}

pub type Id = String;
