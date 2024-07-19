use pyo3::prelude::*;
use sv_parser::{unwrap_node, PortDirection, RefNode, SyntaxTree};

use crate::sv_misc::identifier;

#[derive(Debug, Clone, PartialEq)]
#[pyclass]
pub struct SvPort {
    #[pyo3(get, set)]
    pub identifier: String,
    #[pyo3(get, set)]
    pub direction: SvPortDirection,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum SvPortDirection {
    Inout,
    Input,
    Output,
    Ref,
    IMPLICIT,
}

#[pymethods]
impl SvPortDirection {
    fn __repr__(&self) -> String {
        match self {
            SvPortDirection::Inout => "Inout".to_string(),
            SvPortDirection::Input => "Input".to_string(),
            SvPortDirection::Output => "Output".to_string(),
            SvPortDirection::Ref => "Ref".to_string(),
            SvPortDirection::IMPLICIT => "IMPLICIT".to_string(),
        }
    }
}

pub fn port_declaration_ansi(
    p: &sv_parser::AnsiPortDeclaration,
    syntax_tree: &SyntaxTree,
) -> SvPort {
    SvPort {
        identifier: port_identifier(p, syntax_tree),
        direction: port_direction_ansi(p),
    }
}

fn port_identifier(node: &sv_parser::AnsiPortDeclaration, syntax_tree: &SyntaxTree) -> String {
    if let Some(id) = unwrap_node!(node, PortIdentifier) {
        identifier(id, syntax_tree).unwrap()
    } else {
        unreachable!()
    }
}

fn port_direction_ansi(node: &sv_parser::AnsiPortDeclaration) -> SvPortDirection {
    let dir = unwrap_node!(node, PortDirection);
    match dir {
        Some(RefNode::PortDirection(PortDirection::Input(_))) => SvPortDirection::Input,
        Some(RefNode::PortDirection(PortDirection::Output(_))) => SvPortDirection::Output,
        Some(RefNode::PortDirection(PortDirection::Ref(_))) => SvPortDirection::Ref,
        _ => SvPortDirection::Inout,
    }
}
