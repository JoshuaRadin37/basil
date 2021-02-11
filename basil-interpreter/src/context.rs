use std::rc::Rc;
use std::sync::RwLock;
use std::marker::PhantomData;
use std::cell::RefCell;
use petgraph::prelude::*;
use std::collections::{HashMap, VecDeque, HashSet};
use basil_core::object::Object;
use basil_core::variable::Variable;
use std::ops::Index;
use basil_core::dictionary::Dictionary;

/// Context structs
#[derive(Debug)]
pub struct ContextGraph {
    context_graph: StableGraph<Context, ()>,
    global_context: NodeIndex,
    context_stack: Vec<NodeIndex>
}

impl ContextGraph {
    pub fn new() -> Self {
        let mut graph = StableGraph::new();
        let global_context = graph.add_node(Context::default());
        ContextGraph { context_graph: graph, global_context, context_stack: vec![global_context] }
    }

    pub fn global(&self) -> &Context {
        &self.context_graph[self.global_context]
    }

    pub fn global_mut(&mut self) -> &mut Context {
        &mut self.context_graph[self.global_context]
    }

    pub fn push(&mut self, context: Context) -> NodeIndex {
        let parent = *self.context_stack.last().expect("The Global context should not have been popped");
        let new_node = self.context_graph.add_node(context);
        self.context_graph.add_edge(new_node, parent, ());
        self.context_stack.push(new_node);
        new_node
    }

    pub fn pop(&mut self) {
        let node = self.context_stack.pop().expect("The Global context should not have been popped");
        self.context_graph.remove_node(node);
    }

    pub fn higher_scope(&mut self) -> NodeIndex {
        self.push(Context::default())
    }

    fn current_scope(&self) -> NodeIndex {
        *self.context_stack.last().unwrap()
    }

    pub fn current_context(&mut self) -> CollectedContext {
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

            for neighbor in self.context_graph.neighbors_directed(next, Direction::Outgoing) {
                if !visited.contains(&neighbor) {
                    visit_queue.push_back(neighbor)
                }
            }
        }

        let mut mapping = HashMap::new();
        for node in all_nodes {
            let context = &self.context_graph[node];
            let mut iterator: &mut dyn Iterator<Item=(&String, &Variable)> = &mut context.data.into_iter();
            for (key, value) in iterator {
                if !mapping.contains_key(key) {
                    mapping.insert(key.clone(), value);
                }
            }
        }

        CollectedContext {
            data: mapping
        }
    }

}


#[derive(Debug, Default)]
pub struct Context {
    data: HashMap<String, Variable>
}

pub struct CollectedContext<'a> {
    data: HashMap<String, &'a Variable>
}
