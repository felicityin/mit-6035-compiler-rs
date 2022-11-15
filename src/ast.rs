#[derive(Debug)]
pub struct Program {
    pub import_decls: Vec<ImportDecl>,
    pub field_decls: Vec<FieldDecl>,
    pub method_decls: Vec<MethodDecl>,
}

#[derive(Debug)]
pub struct ImportDecl {
    pub id: Id,
}

#[derive(Debug)]
pub struct FieldDecl {
    pub type_: Type,
    pub field_ids: Vec<FieldDeclId>,
}

#[derive(Debug)]
pub struct FieldDeclId {
    pub id: Id,
    pub arr_len: Option<i32>,
}

#[derive(Debug)]
pub struct MethodDecl {
    pub return_type: ReturnType,
    pub id: Id,
    pub args: Vec<MethodArg>,
    pub block: Block,
}

#[derive(Debug, Clone)]
pub enum ReturnType {
    Type(Type),
    Void,
}

#[derive(Debug)]
pub struct MethodArg {
    pub type_: Type,
    pub id: Id,
}

#[derive(Debug)]
pub struct Block {
    pub field_decls: Vec<FieldDecl>,
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Bool,
}

#[derive(Debug)]
pub enum Statement {
    Assign(Assign),
    MethodCall(MethodCall),
    IfElse(IfElse),
    Loop(Loop),
    While(While),
    Return(Return),
    Break,
    Continue,
}

#[derive(Debug)]
pub struct Assign {
    pub location: Location,
    pub assign_expr: AssignExpr,
}

#[derive(Debug)]
pub struct IfElse {
    pub expr: Expr,
    pub if_block: Block,
    pub else_block: Option<Block>,
}

#[derive(Debug)]
pub struct Loop {
    pub id: Id,
    pub init_expr: Expr,
    pub incre_expr: Expr,
    pub update: ForUpdate,
    pub block: Block,
}

#[derive(Debug)]
pub struct While {
    pub expr: Expr,
    pub block: Block,
}

#[derive(Debug)]
pub struct Return {
    pub expr: Option<Expr>,
}

#[derive(Debug)]
pub struct ForUpdate {
    pub location: Location,
    pub update_expr: ForUpdateExpr,
}

#[derive(Debug)]
pub enum ForUpdateExpr {
    AssignExpr(ForUpdateAssignExpr),
    Increment(Increment),
}

#[derive(Debug)]
pub struct ForUpdateAssignExpr {
    pub compound_assign_op: CompoundAssignOp,
    pub expr: Expr,
}

#[derive(Debug)]
pub enum AssignExpr {
    AssignOpExpr(AssignOpExpr),
    Increment(Increment),
}

#[derive(Debug)]
pub struct AssignOpExpr {
    pub assign_op: AssignOp,
    pub expr: Expr,
}

#[derive(Debug, Clone)]
pub enum AssignOp {
    Assign,
    CompoundAssignOp(CompoundAssignOp),
}

#[derive(Debug, Clone)]
pub enum CompoundAssignOp {
    AddAssign,
    SubAssign,
}

#[derive(Debug, Clone)]
pub enum Increment {
    SelfAdd,
    SelfSub,
}

#[derive(Debug)]
pub enum MethodCall {
    Method(MethodCall0),
    CallOut(MethodCall1),
}

#[derive(Debug)]
pub struct MethodCall0 {
    pub name: MethodName,
    pub args: Vec<Expr>,
}

#[derive(Debug)]
pub struct MethodCall1 {
    pub name: MethodName,
    pub args: Vec<ImportArg>,
}

#[derive(Debug)]
pub struct MethodName {
    pub id: Id,
}

#[derive(Debug)]
pub enum Location {
    Id(Id),
    IdExpr(IdExpr),
}

#[derive(Debug)]
pub struct IdExpr {
    pub id: Id,
    pub expr: Expr,
}

pub type Expr = Box<Expr_>;

#[derive(Debug)]
pub enum Expr_ {
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

#[derive(Debug)]
pub enum ImportArg {
    Expr(Expr),
    StringLiteral(StringLiteral),
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    ArithOp(ArithOp),
    RelOp(RelOp),
    EqOp(EqOp),
    CondOp(CondOp),
}

#[derive(Debug, Clone)]
pub enum ArithOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug, Clone)]
pub enum RelOp {
    Greater,
    Less,
    LessEq,
    GreaterEq,
}

#[derive(Debug, Clone)]
pub enum EqOp {
    EQ,
    NE,
}

#[derive(Debug, Clone)]
pub enum CondOp {
    And,
    Or,
}

#[derive(Debug, Clone)]
pub enum Literal {
    IntLiteral(IntLiteral),
    CharLiteral(CharLiteral),
    BoolLiteral(BoolLiteral),
}

pub type IntLiteral = i32;
pub type DecimalLiteral = i32;
pub type HexLiteral = i32;

#[derive(Debug, Clone)]
pub enum BoolLiteral {
    True,
    False,
}
pub type CharLiteral = char;
pub type StringLiteral = String;

pub type Id = String;
