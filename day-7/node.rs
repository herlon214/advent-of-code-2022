use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug)]
pub enum NodeType {
    Directory,
    File(usize),
}

impl Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeType::File(size) => {
                write!(f, "{}", size)
            }
            NodeType::Directory => {
                write!(f, "dir")
            }
        }
    }
}

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub parent: Option<Rc<RefCell<Node>>>,
    pub children: Vec<Rc<RefCell<Node>>>,
    pub node_type: NodeType,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.node_type, self.name)
    }
}

impl Node {
    pub fn new(name: String, node_type: NodeType, parent: Option<Rc<RefCell<Node>>>) -> Self {
        Node {
            name,
            parent,
            node_type,
            children: vec![],
        }
    }

    pub fn add_child(&mut self, child: Rc<RefCell<Node>>) {
        self.children.push(child);
    }

    pub fn set_parent(&mut self, node: Rc<RefCell<Node>>) {
        self.parent = Some(node)
    }
}

impl From<&str> for Node {
    fn from(input: &str) -> Self {
        let parts: Vec<&str> = input.split(' ').collect();
        let info = parts.get(0).unwrap();
        let name = parts.get(1).unwrap();

        let node_type = match *info {
            "dir" => NodeType::Directory,
            _ => {
                let size = (*info).parse::<usize>().unwrap();

                NodeType::File(size)
            }
        };

        Self {
            name: name.to_string(),
            children: vec![],
            parent: None,
            node_type,
        }
    }
}
