use pyo3::prelude::*;
use sv_parser::{unwrap_node, NodeEvent, RefNode, SyntaxTree};

use crate::{
    sv_misc::identifier,
    sv_port::{port_declaration_ansi, SvPort},
};

#[derive(Debug, Clone, PartialEq)]
#[pyclass]
pub struct SvModule {
    #[pyo3(get, set)]
    pub identifier: String,
    #[pyo3(get, set)]
    pub filepath: String,
    #[pyo3(get, set)]
    pub ports: Vec<SvPort>,
}

#[pymethods]
impl SvModule {
    #[new]
    fn new() -> Self {
        SvModule {
            identifier: String::new(),
            filepath: String::new(),
            ports: Vec::new(),
        }
    }
}

pub fn module_declaration_ansi(m: RefNode, syntax_tree: &SyntaxTree, filepath: &str) -> SvModule {
    let mut ret = SvModule {
        identifier: module_identifier(m.clone(), syntax_tree).unwrap(),
        filepath: filepath.to_string(),
        ports: Vec::new(),
    };

    let mut entering: bool;

    for event in m.into_iter().event() {
        let node = match event {
            NodeEvent::Enter(x) => {
                entering = true;
                x
            }
            NodeEvent::Leave(x) => {
                entering = false;
                x
            }
        };

        if let RefNode::AnsiPortDeclaration(decl) = node {
            if entering {
                ret.ports.push(port_declaration_ansi(decl, syntax_tree));
            }
        }
    }

    ret
}

fn module_identifier(node: RefNode, syntax_tree: &SyntaxTree) -> Option<String> {
    if let Some(id) = unwrap_node!(node, ModuleIdentifier) {
        identifier(id, syntax_tree)
    } else {
        unreachable!()
    }
}
