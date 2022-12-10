use crate::ast;

use super::errors::{ SemanticCheckError };

pub fn check_main(p: &ast::Program) -> Result<(), SemanticCheckError> {
    let methods: Vec<&ast::MethodDecl> = p.method_decls.iter().filter(|m| m.id == "main").collect();
    if methods.len() != 1 {
        return Err(SemanticCheckError::MainMethodShouldOnlyOne);
    }

    if !methods[0].args.is_empty() {
        return Err(SemanticCheckError::MainMethodArgsShouldEmpty);
    }

    if methods[0].return_type != ast::ReturnType::Void {
        return Err(SemanticCheckError::MainMethodShouldReturnVoid);
    }

    Ok(())
}
