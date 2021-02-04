use crate::variable::Variable;

pub enum Statement {
    Assignment(String, Variable),
    If()
}