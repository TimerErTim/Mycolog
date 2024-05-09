use std::fmt::Debug;

use surrealdb_core::sql;
use surrealdb_core::sql::statements::{
    BeginStatement, CancelStatement, CommitStatement, CreateStatement, DefineStatement,
    DeleteStatement, IfelseStatement, InfoStatement, InsertStatement, KillStatement, LiveStatement,
    OptionStatement, OutputStatement, RelateStatement, RemoveStatement, SelectStatement,
    SetStatement, UpdateStatement, UseStatement,
};
use surrealdb_core::sql::{parse, Statement, Statements};

pub trait IntoStatements: Debug {
    /// Converts itself into an SQL query.
    fn into_statements(self) -> anyhow::Result<Vec<Statement>>;
}

impl IntoStatements for sql::Query {
    fn into_statements(self) -> anyhow::Result<Vec<Statement>> {
        let sql::Query(Statements(statements)) = self;
        Ok(statements)
    }
}

impl IntoStatements for Statements {
    fn into_statements(self) -> anyhow::Result<Vec<Statement>> {
        let Statements(statements) = self;
        Ok(statements)
    }
}

impl IntoStatements for Vec<Statement> {
    fn into_statements(self) -> anyhow::Result<Vec<Statement>> {
        Ok(self)
    }
}

impl IntoStatements for Statement {
    fn into_statements(self) -> anyhow::Result<Vec<Statement>> {
        Ok(vec![self])
    }
}

impl IntoStatements for UseStatement {
    fn into_statements(self) -> anyhow::Result<Vec<Statement>> {
        Ok(vec![Statement::Use(self)])
    }
}

impl IntoStatements for SetStatement {
    fn into_statements(self) -> anyhow::Result<Vec<Statement>> {
        Ok(vec![Statement::Set(self)])
    }
}

impl IntoStatements for InfoStatement {
    fn into_statements(self) -> anyhow::Result<Vec<Statement>> {
        Ok(vec![Statement::Info(self)])
    }
}

impl IntoStatements for LiveStatement {
    fn into_statements(self) -> anyhow::Result<Vec<Statement>> {
        Ok(vec![Statement::Live(self)])
    }
}

impl IntoStatements for KillStatement {
    fn into_statements(self) -> anyhow::Result<Vec<Statement>> {
        Ok(vec![Statement::Kill(self)])
    }
}

impl IntoStatements for BeginStatement {
    fn into_statements(self) -> anyhow::Result<Vec<Statement>> {
        Ok(vec![Statement::Begin(self)])
    }
}

impl IntoStatements for CancelStatement {
    fn into_statements(self) -> anyhow::Result<Vec<Statement>> {
        Ok(vec![Statement::Cancel(self)])
    }
}

impl IntoStatements for CommitStatement {
    fn into_statements(self) -> anyhow::Result<Vec<Statement>> {
        Ok(vec![Statement::Commit(self)])
    }
}

impl IntoStatements for OutputStatement {
    fn into_statements(self) -> anyhow::Result<Vec<Statement>> {
        Ok(vec![Statement::Output(self)])
    }
}

impl IntoStatements for IfelseStatement {
    fn into_statements(self) -> anyhow::Result<Vec<Statement>> {
        Ok(vec![Statement::Ifelse(self)])
    }
}

impl IntoStatements for SelectStatement {
    fn into_statements(self) -> anyhow::Result<Vec<Statement>> {
        Ok(vec![Statement::Select(self)])
    }
}

impl IntoStatements for CreateStatement {
    fn into_statements(self) -> anyhow::Result<Vec<Statement>> {
        Ok(vec![Statement::Create(self)])
    }
}

impl IntoStatements for UpdateStatement {
    fn into_statements(self) -> anyhow::Result<Vec<Statement>> {
        Ok(vec![Statement::Update(self)])
    }
}

impl IntoStatements for RelateStatement {
    fn into_statements(self) -> anyhow::Result<Vec<Statement>> {
        Ok(vec![Statement::Relate(self)])
    }
}

impl IntoStatements for DeleteStatement {
    fn into_statements(self) -> anyhow::Result<Vec<Statement>> {
        Ok(vec![Statement::Delete(self)])
    }
}

impl IntoStatements for InsertStatement {
    fn into_statements(self) -> anyhow::Result<Vec<Statement>> {
        Ok(vec![Statement::Insert(self)])
    }
}

impl IntoStatements for DefineStatement {
    fn into_statements(self) -> anyhow::Result<Vec<Statement>> {
        Ok(vec![Statement::Define(self)])
    }
}

impl IntoStatements for RemoveStatement {
    fn into_statements(self) -> anyhow::Result<Vec<Statement>> {
        Ok(vec![Statement::Remove(self)])
    }
}

impl IntoStatements for OptionStatement {
    fn into_statements(self) -> anyhow::Result<Vec<Statement>> {
        Ok(vec![Statement::Option(self)])
    }
}

impl IntoStatements for &str {
    fn into_statements(self) -> anyhow::Result<Vec<Statement>> {
        parse(self)?.into_statements()
    }
}

impl IntoStatements for &String {
    fn into_statements(self) -> anyhow::Result<Vec<Statement>> {
        parse(self)?.into_statements()
    }
}

impl IntoStatements for String {
    fn into_statements(self) -> anyhow::Result<Vec<Statement>> {
        parse(&self)?.into_statements()
    }
}

impl IntoStatements for () {
    fn into_statements(self) -> anyhow::Result<Vec<Statement>> {
        Ok(Vec::new())
    }
}
