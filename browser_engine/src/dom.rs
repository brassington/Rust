struct Node {
  // data common to all nodes:
  children: Vec<Node>,

  // data specific to each node type:
  node_type: NodeType,
}

enum NodeType {
  Text(String),
  Comment(String),
  Element(ElementData),
}

struct ElementData {
  tag_name: String,
  attributes: AttrMap,
}

type AttrMap = HashMap<String, String>;

// constructor functions 

fn text(data: String) -> Node {
  Node { children: vec![], node_type: Text(data) }
}

fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
  Node {
    children: children,
    node_type: Element(ElementData {
      tag_name: name,
      attributes: attrs,
    })
  }
}

// element data

impl ElementData {

  fn get_attribute(&self, key: &str) -> Option<&String> {
    self.attributes.find_equiv(&key)
  }

  fn id(&self) -> Option<&String> {
    self.get_attribute("id")
  }

  fn classes(&self) -> HashSet<&str> {
    match self.get_attribute("class") {
      Some(classList) => classList.as_slice().split(' ').collect(),
      None => HashSet::new()
    }
  }
}
