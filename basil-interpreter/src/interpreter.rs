use std::borrow::Borrow;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::ops::Deref;

use petgraph::graph::NodeIndex;

use basil_core::class::Class;
use basil_core::code_block::CodeBlock;
use basil_core::dictionary::Dictionary;
use basil_core::exception::Exception;
use basil_core::expression::{Expression, ExpressionTail};
use basil_core::function::Function;
use basil_core::object::Object;
use basil_core::primitive::Primitive;
use basil_core::statements::Statement;
use basil_core::type_id::TypeId;
use basil_core::variable::{IntoVariable, Variable};
use basil_frontend::span::Span;

use crate::context::{Context, ContextGraph, Entry};
use crate::frame::Frame;

pub struct Interpreter {
    context_graph: ContextGraph,
    type_to_context_node: HashMap<TypeId, NodeIndex>,
    frame_stack: Vec<Frame>,
}

macro_rules! basil {
    ($interpreter:expr, $variable:expr) => {
        $interpreter
            .context_graph
            .current_context()
            .get(stringify!($variable))
            .unwrap()
    };
    ($variable:ident $(.$member:ident)+) => {
        {
            let mut var: Result<Variable, Exception> = Ok($variable.clone());
            $(
                if let Ok(var2) = var {
                    let member = basil_core::object::Object::from(stringify!($member));
                    let next = var2.get_member(member, Object::basic_hash, Object::basic_eq);
                    var = next;
                }
            )*
            var
        }
    };
    ($variable:ident $(.$member:ident)+ = $value:expr) => {
        {
            let mut var: Result<Variable, Exception> = Ok($variable.clone());
            $(
                if let Ok(var2) = var {
                    let member = basil_core::object::Object::from(stringify!($member));
                    let next = var2.get_member(member, basil_core::object::Object::basic_hash, basil_core::object::Object::basic_eq);
                    var = next;
                }
            )*
            if let Ok(var) = var {
                let mut borrowed = var;
                borrowed.set_object($value);
            } else {
                panic!("{} is not a member of {}, so its value can't be set", stringify!($($member).*), stringify!($variable))
            }
        }
    };
    ($variable:ident $([$member:expr])+) => {
        {
            let mut var: Result<Variable, Exception> = Ok($variable.clone());
            $(
                if let Ok(var2) = var {
                    let member = basil_core::object::Object::from($member);
                    let next = var2.get_member(member, Object::basic_hash, Object::basic_eq);
                    var = next;
                }
            )*
            var
        }
    };
    ($variable:ident $([$member:expr])+ = $value:expr) => {
        {
            let mut var: Result<Variable, Exception> = Ok($variable.clone());
            $(
                if let Ok(var2) = var {
                    let member = basil_core::object::Object::from($member);
                    let next = var2.get_member(member, basil_core::object::Object::basic_hash, basil_core::object::Object::basic_eq);
                    var = next;
                }
            )*
            if let Ok(var) = var {
                let mut borrowed = var;
                borrowed.set_object($value);
            } else {
                panic!("{} is not a member of {}, so its value can't be set", stringify!($($member).*), stringify!($variable))
            }
        }
    };
    ({}) => {

        basil_core::dictionary::Dictionary::new().into_variable()


     };
}

impl Interpreter {
    pub fn new(context_graph: ContextGraph) -> Self {
        Interpreter {
            context_graph,
            type_to_context_node: Default::default(),
            frame_stack: vec![],
        }
    }

    pub fn current_frame(&self) -> &Frame {
        self.frame_stack
            .last()
            .expect("There must always be a frame on the stack while running")
    }

    fn current_frame_mut(&mut self) -> &mut Frame {
        self.frame_stack
            .last_mut()
            .expect("There must always be a frame on the stack while running")
    }

    fn new_frame(&mut self, name: String, span: Span) {
        self.frame_stack.push(Frame::new(name, span))
    }

    fn pop_frame(&mut self) -> Option<Frame> {
        self.frame_stack.pop()
    }

    pub fn execute_block(&mut self, block: &CodeBlock) -> Result<Variable, Exception> {
        let size = block.statements().len();
        for i in 0..(size - 1) {
            let statement = &block.statements()[i];
            match self.execute_statement(statement) {
                Ok(_) => {}
                Err(e) => {
                    let inner_var = e.inner().clone();
                    return Interpreter::to_block_output(e, inner_var);
                }
            }
        }
        if let Some(last) = block.statements().last() {
            match self.execute_statement(last) {
                Ok(o) => Ok(o),
                Err(e) => {
                    let inner_var = e.inner().clone();
                    Interpreter::to_block_output(e, inner_var)
                }
            }
        } else {
            Ok(Primitive::None.into_variable())
        }
    }

    fn to_block_output(e: Exception, inner_var: Variable) -> Result<Variable, Exception> {
        if let Ok(is_return) = basil!(inner_var.__is_return__) {
            let x = is_return.to_inner().get();
            let object = x.get();
            match bool::try_from(object.as_primitive()) {
                Ok(is_return) => basil!(inner_var.__return_val__),
                Err(e) => Err(e),
            }
        } else {
            Err(e)
        }
    }

    pub fn execute_statement(&mut self, statement: &Statement) -> Result<Variable, Exception> {
        match statement {
            Statement::Assignment(left, right) => {
                let variable = self.evaluate_expression(right)?;
                let mut current_context = self.context_graph.current_context();
                let mut entry = current_context.entry(left.clone());

                match entry {
                    Entry::Occupied(o) => {
                        let mut value = o.get();
                        *value = variable.clone();
                        Ok(variable)
                    }
                    Entry::Vacant(v) => {
                        let x = v.insert(variable);
                        Ok(x.clone())
                    }
                }
            }
            Statement::If {
                condition,
                block,
                elifs,
                r#else,
            } => {
                unimplemented!()
            }
            Statement::While { .. } => {
                unimplemented!()
            }
            Statement::Expression(_) => {
                unimplemented!()
            }
            Statement::Return(ret) => {
                let value = self.evaluate_expression(ret)?;
                let mut dictionary =
                    Dictionary::with_entries(&["__is_return__", "__return_val__"]).into_variable();
                basil!(dictionary.__is_return__ = true);
                Ok(dictionary)
            }
            Statement::Raise(_) => {
                unimplemented!()
            }
        }
    }

    pub fn repr(&mut self, var: &Variable) -> String {
        unimplemented!()
    }

    pub fn evaluate_expression(
        &mut self,
        mut expression: &Expression,
    ) -> Result<Variable, Exception> {
        let mut head = expression.head();
        let tail = expression.tail();

        if tail.is_none() {
            return Ok(head.clone());
        }

        let tail = tail.unwrap();
        match tail {
            ExpressionTail::GetMember(member) => {
                let inner = head.to_inner().get();
                let inner = inner.get();
                let mut member_primitive: Object = Primitive::from(member).into();
                if let Primitive::Dictionary(dict) = inner.as_primitive() {
                    let member =
                        dict.get(&mut member_primitive, Object::basic_hash, Object::basic_eq);

                    unimplemented!()
                } else {
                    Err(format!(
                        "{} is not a member of {}",
                        self.repr(&member.into_variable()),
                        self.repr(head)
                    ))?
                }
            }
            ExpressionTail::CallMethod { positional, named } => {
                unimplemented!()
            }
        }
    }

    pub fn attach_class(&mut self, class: &Class) {
        if class.created() {
            return;
        }

        let id = TypeId::Explicit(class.id());
        let mut dictionary = Dictionary::new();
        for (key, val) in class.definitions() {
            dictionary.insert(
                Object::from(key),
                Variable::new(val.clone()),
                Object::basic_hash,
                Object::basic_eq,
            );
        }
        let new_context = Context::from(&dictionary);
        let node = self.context_graph.add_new_context(new_context);

        self.type_to_context_node.insert(id, node); // Adds entry into class

        for parent in class.parents() {
            let parent_id = TypeId::Explicit(*parent);
            let parent_index = &self.type_to_context_node[&parent_id];
            self.context_graph.set_parent(&node, parent_index); // will remove reference to global context
        }

        class.set_created();
    }

    fn find_method<S: AsRef<str>>(&self, name: S, var: &Variable) -> Result<Variable, Exception> {
        self.find_method_helper(name.as_ref(), var)
    }

    fn find_method_helper(&self, name: &str, var: &Variable) -> Result<Variable, Exception> {
        let inner = var.to_inner().get();
        let inner = inner.get();
        let primitive = inner.as_primitive();
        let mut name_object: Object = Primitive::from(name).into();

        if let Primitive::Dictionary(dict) = primitive {
            if let Some(var) = dict.get(&mut name_object, Object::basic_hash, Object::basic_eq) {
                if var.to_inner().get().get().as_primitive().is_function() {
                    return Ok(var.clone());
                } else {
                    Err(format!("{} is not a function in {:?}", name, var))?
                }
            } else {
                Err(format!("No entry {} in {:?}", name, var))?
            }
        } else {
            Err("Can't find a method on a type that isn't backed by a dictionary")?
        }
    }

    fn call_method(
        &mut self,
        name: String,
        object: &Variable,
        function: &Function,
        positional_arguments: Vec<Variable>,
        keywords: Vec<(String, Variable)>,
    ) -> Result<Variable, Exception> {
        let object = object.to_inner().get();
        let object = object.get_mut();
        let type_id = object.type_id();
        let node_index = self.type_to_context_node[&type_id];

        self.context_graph.shift_to_scope(node_index); // shifts to the class scope
        self.context_graph.higher_scope();

        let mut context = self.context_graph.current_context();

        for (capture, value) in function.captures() {
            context.insert(capture.clone(), value.clone());
        }

        let block = function.code_block();

        // self.new_frame(name, )

        self.context_graph.pop();
        self.context_graph.pop();

        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_member() {
        let mut dict = Dictionary::with_entries(&["var_name"]).into_variable();
        println!("Dict: {:?}", dict);
        basil!(dict.var_name = true);
        println!("Dict: {:?}", dict);
        let var_name = basil!(dict.var_name).unwrap();
        let ptr = var_name.get_object();
        let inner = ptr.get();
        let object = ptr.get();
        let primitive = object.as_primitive();
        if let Primitive::Boolean(bl) = primitive {
            assert_eq!(bl, &true);
        } else if let b = primitive {
            panic!(
                "dict.var_name wasn't set to a boolean, instead set to {:?}",
                b
            );
        }
    }

    #[test]
    fn variables_separate() {
        let mut dict = Dictionary::with_entries(&["var1", "var2"]).into_variable();
        let val = 0i64.into_variable();
        basil!(dict.var1 = val.clone());
        basil!(dict.var2 = val);
        println!("{:?}", dict);
        basil!(dict.var1 = 1i32);
        println!("{:?}", dict);
        let dict_var1 = basil!(dict.var1).unwrap();
        basil!(dict.var2 = dict_var1);
        println!("{:?}", dict);
        basil!(dict.var1 = 2i32);
        println!("{:?}", dict);
        let dict_var1: i32 = basil!(dict["var1"]).unwrap().try_into().unwrap();
        let dict_var2: i32 = basil!(dict["var2"]).unwrap().try_into().unwrap();
        assert_eq!(dict_var1, 2);
        assert_eq!(dict_var2, 1);
    }
}
