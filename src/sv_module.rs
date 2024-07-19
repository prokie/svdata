use pyo3::prelude::*;
use sv_parser::{unwrap_node, NodeEvent, RefNode, SyntaxTree};

use crate::{
    sv_misc::identifier,
    sv_port::{port_declaration_ansi, SvPort},
    sv_variable::{variable_declaration, SvVariable},
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
    #[pyo3(get, set)]
    pub variables: Vec<SvVariable>,
}

#[pymethods]
impl SvModule {
    #[new]
    fn new() -> Self {
        SvModule {
            identifier: String::new(),
            filepath: String::new(),
            ports: Vec::new(),
            variables: Vec::new(),
        }
    }
}

pub fn module_declaration_ansi(m: RefNode, syntax_tree: &SyntaxTree, filepath: &str) -> SvModule {
    let mut ret = SvModule {
        identifier: module_identifier(m.clone(), syntax_tree).unwrap(),
        filepath: filepath.to_string(),
        ports: Vec::new(),
        variables: Vec::new(),
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
        if entering {
            match node {
                RefNode::AnsiPortDeclaration(p) => {
                    let port = port_declaration_ansi(p, syntax_tree);
                    ret.ports.push(port);
                }
                RefNode::NonPortModuleItem(p) => {
                    let variable = variable_declaration(p, syntax_tree);
                    ret.variables.push(variable);
                }

                _ => (),
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
