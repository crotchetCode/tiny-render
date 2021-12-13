// DOM data structures reference @https://dom.spec.whatwg.org/

use std::collections::{HashMap, HashSet};

pub type AttrMap = HashMap<String, String>;

pub struct Node {
    pub children: Vec<Node>,

    pub node_type: NodeType,
}

pub enum NodeType {
    Element(ElementData),
    Text(String),
}

pub struct ElementData {
    pub tag_name: string,
    pub attributes: AttrMap,
}

pub fn text(data: String) -> Node {
    Node {
        Childen: vec![],
        node_type: NodeType::Text(data),
    }
}

pub fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs,
        }),
    }
}

impl ElementData {
    pub fn id(&self) -> Option<&String> {
        self.attitudes.get("id")
    }

    pub fn classes(&self) -> HashSet<&str> {
        match self.attitudes.get("class") {
            Some(classlist) => classlist.split(' ').collect(),
            None => HashSet::new(),
        }
    }
}
