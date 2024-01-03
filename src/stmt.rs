use crate::expr::Expr;
use crate::errors::*;
use crate::tokens::*;

pub enum Stmt {
    Block(BlockStmt),
    Expression(ExpressionStmt),
    Print(PrintStmt),
    Var(VarStmt),
}

impl Stmt {
    pub fn accept<T>(&self, stmt_visitor: &dyn StmtVisitor<T>) -> Result<T , Error> {
        match self {
            Stmt::Block(v) => v.accept(stmt_visitor),
            Stmt::Expression(v) => v.accept(stmt_visitor),
            Stmt::Print(v) => v.accept(stmt_visitor),
            Stmt::Var(v) => v.accept(stmt_visitor),
        }
    }
}

pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}

pub struct ExpressionStmt {
    pub expression: Expr,
}

pub struct PrintStmt {
    pub expression: Expr,
}

pub struct VarStmt {
    pub name: Token,
    pub initializer: Option<Expr>,
}

pub trait StmtVisitor<T> {
    fn visit_block_stmt(&self, expr: &BlockStmt) ->  Result<T , Error>;
    fn visit_expression_stmt(&self, expr: &ExpressionStmt) ->  Result<T , Error>;
    fn visit_print_stmt(&self, expr: &PrintStmt) ->  Result<T , Error>;
    fn visit_var_stmt(&self, expr: &VarStmt) ->  Result<T , Error>;
}

impl BlockStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) ->  Result<T , Error> {
        visitor.visit_block_stmt(self)
    }
}

impl ExpressionStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) ->  Result<T , Error> {
        visitor.visit_expression_stmt(self)
    }
}

impl PrintStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) ->  Result<T , Error> {
        visitor.visit_print_stmt(self)
    }
}

impl VarStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) ->  Result<T , Error> {
        visitor.visit_var_stmt(self)
    }
}

