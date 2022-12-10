use super::ir::{ VarDecl, MethodDecl };

#[derive(Debug)]
pub enum SemanticCheckError {
    DuplicatedVar(VarDecl), // rule 1.1
    DuplicatedMethod(MethodDecl), // rule 1.1
    UsedBeforeDeclared(String), // rule 2, 10, 11
    MainMethodShouldOnlyOne, // rule 3.1
    MainMethodArgsShouldEmpty, // rule 3.2
    MainMethodShouldReturnVoid, // rule 3.3
    ArrayLenShouldPositive(String), // rule 4, 13
    IdMustArray(String), // rule 12.1
    TypeOfExprMustInt(String), // rule 12.2
    MethodSignatureMismatch, // rule 5
    InvalidMethodArgs, // rule 7
    InvalidReturn, // ruile 8
    ReturnTypeMismatch, // rule 6, 9
    ConditionTypeShouldBool, // rule 14
    OperandsTypeMismatch, // rule 15, 16, 17
    LocationTypeMismatch, // rule 18, 19
    InvalidBreak, // rule 20.1
    InvalidContinue, // rule 20.2
}

pub type IRResult<T> = Result<T, Vec<SemanticCheckError>>;
