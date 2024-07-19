use sv_parser::{unwrap_node, RefNode, SyntaxTree};

pub fn identifier(parent: RefNode, syntax_tree: &SyntaxTree) -> Option<String> {
    let id = match unwrap_node!(parent, SimpleIdentifier, EscapedIdentifier) {
        Some(RefNode::SimpleIdentifier(x)) => Some(x.nodes.0),
        Some(RefNode::EscapedIdentifier(x)) => Some(x.nodes.0),
        _ => None,
    };

    id.map(|x| syntax_tree.get_str(&x).unwrap().to_string())
}
