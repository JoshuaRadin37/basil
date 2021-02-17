use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::rc::Rc;
use std::sync::RwLock;

use petgraph::graph::node_index;
use petgraph::prelude::*;

use basil_core::class::Class;
use basil_core::dictionary::Dictionary;
use basil_core::object::Object;
use basil_core::primitive::Primitive;
use basil_core::variable::Variable;

/// Context structs
#[derive(Debug)]
pub struct ContextGraph {
    context_graph: StableGraph<Context, ()>,
    global_context: NodeIndex,
    context_stack: Vec<NodeIndex>,
}

impl ContextGraph {
    pub fn new() -> Self {
        let mut graph = StableGraph::new();
        let global_context = graph.add_node(Context::default());
        ContextGraph {
            context_graph: graph,
            global_context,
            context_stack: vec![global_context],
        }
    }

    pub fn global(&self) -> &Context {
        &self.context_graph[self.global_context]
    }

    pub fn global_mut(&mut self) -> &mut Context {
        &mut self.context_graph[self.global_context]
    }

    pub fn global_id(&self) -> &NodeIndex {
        &self.global_context
    }

    pub fn push(&mut self, context: Context) -> NodeIndex {
        let parent = *self
            .context_stack
            .last()
            .expect("The Global context should not have been popped");
        let new_node = self.context_graph.add_node(context);
        self.context_graph.add_edge(new_node, parent, ());
        self.context_stack.push(new_node);
        new_node
    }

    /// Creates a new context that has both the current scope and the scope labeled by the context
    /// to the top
    pub fn push_id(&mut self, context: NodeIndex) -> NodeIndex {
        let new_node = self.higher_scope();
        self.context_graph.add_edge(new_node, context, ());
        new_node
    }

    pub fn pop(&mut self) {
        let node = self
            .context_stack
            .pop()
            .expect("The Global context should not have been popped");
        self.context_graph.remove_node(node);
    }

    fn peek(&self) -> &Context {
        let parent = *self
            .context_stack
            .last()
            .expect("The Global context should not have been popped");
        &self.context_graph[parent]
    }

    fn peek_mut(&mut self) -> &mut Context {
        let parent = *self
            .context_stack
            .last()
            .expect("The Global context should not have been popped");
        &mut self.context_graph[parent]
    }

    pub fn higher_scope(&mut self) -> NodeIndex {
        self.push(Context::default())
    }

    /// Creates a new scope on top of the scope stack that only has the global scope as a parent
    pub fn shift_new_scope(&mut self) -> NodeIndex {
        let new_node = self.context_graph.add_node(Context::default());
        self.context_graph
            .add_edge(new_node, self.global_context.clone(), ());
        self.context_stack.push(new_node);
        new_node
    }

    pub fn shift_to_scope(&mut self, scope: NodeIndex) {
        self.context_stack.push(scope);
    }

    /// Adds a new context, with a single
    pub fn add_new_context(&mut self, context: Context) -> NodeIndex {
        let ret = self.context_graph.add_node(context);
        self.context_graph.add_edge(ret, self.global_context, ());
        ret
    }

    pub fn set_parent(&mut self, child: &NodeIndex, parent: &NodeIndex) -> bool {
        if !(self.context_graph.contains_node(*child) && self.context_graph.contains_node(*parent))
        {
            return false;
        }
        self.context_graph.add_edge(*child, *parent, ());
        if let Some(edge) = self
            .context_graph
            .find_edge(*child, self.global_context.clone())
        {
            self.context_graph.remove_edge(edge);
        }
        true
    }

    fn current_scope(&self) -> NodeIndex {
        *self.context_stack.last().unwrap()
    }

    pub fn current_context(&mut self) -> CollectedContext {
        /*
        let mut all_nodes = vec![];
        let mut visited = HashSet::new();
        let mut visit_queue = VecDeque::new();
        visit_queue.push_back(self.current_scope());
        while let Some(next) = visit_queue.pop_front() {
            if visited.contains(&next) {
                continue;
            }

            all_nodes.push(next);
            visited.insert(next);

            for neighbor in self
                .context_graph
                .neighbors_directed(next, Direction::Outgoing)
            {
                if !visited.contains(&neighbor) {
                    visit_queue.push_back(neighbor)
                }
            }
        }

        let mut mapping = HashMap::new();
        for node in all_nodes {
            let context = &self.context_graph[node];
            let mut iterator: &mut dyn Iterator<Item = (&String, &Variable)> =
                &mut context.data.iter();
            for (key, value) in iterator {
                if !mapping.contains_key(key) {
                    mapping.insert(key.clone(), value);
                }
            }
        }

        CollectedContext { data: mapping }

         */
        CollectedContext::new(self)
    }
}

#[derive(Debug, Default)]
pub struct Context {
    data: HashMap<String, Variable>,
}

impl From<&Dictionary> for Context {
    fn from(dict: &Dictionary) -> Self {
        let mut mapping = HashMap::new();
        for (key, value) in dict {
            if let Primitive::String(id) = key.as_primitive() {
                mapping.insert(id.clone(), value.clone());
            }
        }
        Context { data: mapping }
    }
}

pub struct CollectedContext<'a> {
    data: &'a mut ContextGraph,
    context_order: Vec<NodeIndex>,
}

impl<'a> CollectedContext<'a> {
    pub fn new(data: &'a mut ContextGraph) -> Self {
        let mut context_order = vec![];
        let mut visited = HashSet::new();
        let mut visit_queue = VecDeque::new();
        visit_queue.push_back(data.current_scope());
        while let Some(next) = visit_queue.pop_front() {
            if visited.contains(&next) {
                continue;
            }

            context_order.push(next);
            visited.insert(next);

            for neighbor in data
                .context_graph
                .neighbors_directed(next, Direction::Outgoing)
            {
                if !visited.contains(&neighbor) {
                    visit_queue.push_back(neighbor)
                }
            }
        }

        CollectedContext {
            data,
            context_order,
        }
    }

    pub fn get(&self, key: &String) -> Option<&Variable> {
        /*
        for node in self
            .context_order
            .iter()
            .filter_map(|id| self.data.context_graph.node_weight(*id))
        {
            if let Some(ret) = node.data.get(key) {
                return Some(ret);
            }
        }
        None

         */
        self.get_with_index(key).map(|(ret, _)| ret)
    }

    fn get_with_index(&self, key: &String) -> Option<(&Variable, NodeIndex)> {
        for (node, index) in self.context_order.iter().filter_map(|id| {
            self.data
                .context_graph
                .node_weight(*id)
                .map(|context| (context, *id))
        }) {
            if let Some(ret) = node.data.get(key) {
                return Some((ret, index));
            }
        }
        None
    }

    pub fn get_mut(&mut self, key: &String) -> Option<&mut Variable> {
        let mut found_index: Option<NodeIndex> = None;
        for node in self.context_order.iter().cloned() {
            if let Some(context) = self.data.context_graph.node_weight(node) {
                if context.data.get(key).is_some() {
                    found_index = Some(node.clone());
                    break;
                }
            }
        }

        match found_index {
            Some(index) => {
                let context = self.data.context_graph.node_weight_mut(index).unwrap();
                context.data.get_mut(key)
            }
            None => None,
        }
    }

    pub fn entry(&mut self, key: String) -> Entry {
        if let Some((_, var)) = self.get_with_index(&key) {
            return Entry::Occupied(Occupied::new(self.data, var, key));
        }

        Entry::Vacant(Vacant {
            context_graph: self.data,
            key,
        })
    }

    pub fn insert(&mut self, key: String, value: Variable) {
        let first = self
            .context_order
            .first()
            .expect("Context order can not be empty");
        self.data
            .context_graph
            .node_weight_mut(*first)
            .unwrap()
            .data
            .insert(key, value);
    }

    pub fn contains(&self, key: &String) -> bool {
        for node in self.context_order.iter().filter_map(|id| {
            self.data
                .context_graph
                .node_weight(*id)
        }) {
            if node.data.contains_key(key) {
                return true;
            }
        }
        false
    }
}

impl<S: AsRef<str>> Index<S> for CollectedContext<'_> {
    type Output = Variable;

    fn index(&self, index: S) -> &Self::Output {
        let string = index.as_ref().to_string();
        self.get(&string).unwrap()
    }
}

impl<S: AsRef<str>> IndexMut<S> for CollectedContext<'_> {
    fn index_mut(&mut self, index: S) -> &mut Self::Output {
        let string = index.as_ref().to_string();
        self.get_mut(&string).unwrap()
    }
}

pub enum Entry<'e> {
    Occupied(Occupied<'e>),
    Vacant(Vacant<'e>),
}

impl<'e> Entry<'e> {
    pub fn or_insert(self, default: Variable) -> &'e mut Variable {
        match self {
            Entry::Occupied(o) => o.get(),
            Entry::Vacant(v) => v.insert(default),
        }
    }
}

pub struct Occupied<'e> {
    context_graph: &'e mut ContextGraph,
    node_index: NodeIndex,
    hash: String,
}

impl<'e> Occupied<'e> {
    fn new(context_graph: &'e mut ContextGraph, node_index: NodeIndex, hash: String) -> Self {
        Occupied {
            context_graph,
            node_index,
            hash,
        }
    }

    pub fn insert(self, val: Variable) -> &'e mut Variable {
        let variable = self.get();
        *variable = val;
        variable
    }

    pub fn get(mut self) -> &'e mut Variable {
        let Occupied {
            context_graph,
            node_index,
            hash,
        } = self;
        context_graph
            .context_graph
            .node_weight_mut(node_index)
            .unwrap()
            .data
            .get_mut(&hash)
            .unwrap()
    }
}

pub struct Vacant<'e> {
    context_graph: &'e mut ContextGraph,
    key: String,
}

impl<'e> Vacant<'e> {
    fn new(context_graph: &'e mut ContextGraph, key: String) -> Self {
        Vacant { context_graph, key }
    }

    pub fn insert(mut self, value: Variable) -> &'e mut Variable {
        let Vacant { context_graph, key } = self;
        let context = context_graph.peek_mut();
        context.data.insert(key.clone(), value);
        context.data.get_mut(&key).unwrap()
    }
}
