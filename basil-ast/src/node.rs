use crate::operators::Operator;
use basil_core::span::Span;
use std::fmt::Display;

#[derive(Debug)]
pub struct Node {
    span: Span,
    node_type: Box<NodeType>,
}

/// The type of the node, which determines the children of the node
#[derive(Debug)]
pub enum NodeType {
    Identifier(String),
    QualifiedIdentifier {
        parent: Node,
        child: Node,
    },
    Assignment {
        lhs: Node,
        var_type: Option<Node>,
        rhs: Node,
    },
    Function {
        name: Node,
        parameters: Vec<Node>,
    },
    FunctionCall {
        name: Node,
        parameters: Vec<Node>,
    },
    BinaryExpression {
        lhs: Node,
        rhs: Node,
        op: Node,
    },
    UnaryExpression {
        val: Node,
        op: Node,
    },
    Operator(Operator),
    If {
        condition: Node,
        block: Node,
        r#else: Option<Node>,
    },
    While {
        condition: Node,
        block: Node,
    },
    For {
        identifier: Node,
        iterator: Node,
    },
    Break,
    Yield(Node),
    Return(Node),
    Class {
        name: Node,
        parent: Option<Node>,
        defs: Vec<Node>,
    },
    Block(Vec<Node>),
    Import(Node),
}
