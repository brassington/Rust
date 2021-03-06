struct Parser {
  pos: uint,
  input: String,
}

impl Parser {

  // Read the next character without consuming it
  fn next_char(&self) -> char {
    self.input.as_slice().char_at(self.pos)
  }

  // Do the next characters start with a given string?
  fn starts_with(&self, string: &str) -> bool {
    self.input.as_slice().slice_from(self.pos).starts_with(string)
  }

  // Return true if all input is consumed
  fn eof(&self) -> bool {
    self.pos >= self.input.len()
  }

  // Return the current character, and advance to the next character
  fn consume_char(&mut self) -> char {
    let range = self.input.as_slice().char_range_at(self.pos);
    self.pos = range.next;
    return range.ch;
  }

  // Consume characters until `test` return false
  fn consume_while(&mut self, test: |char| -> bool) -> String {
    let mut result = String::new();
    while !self.eof() && test(self.next_char()) {
      result.push_char(self.consume_char());
    }
    return result;
  }

  // Consume and discard zero or more whitespace characters
  fn consume_whitespace(&mut self) {
    self.consume_while(|c| c.is_whitespace());
  }

  // Parse a tag or attribute name
  fn parse_tag_name(&mut self) -> String {
    self.consume_while(|c| match c {
      'a'..'z' | 'A'..'Z' | '0'..'9' => true,
      _ => false
    })
  }

  // Parse a single node
  fn parse_node(&mut self) -> dom::Node {
    match self.next_char() {
      '<' => self.parse_element(),
      _   => self.parse_text()
    }
  }

  // Parse a text node
  fn parse_text_node(&mut self) -> dom::Node {
    dom::text(self.consume_while(|c| c != '<'))
  }

  // Parse a single element, including its 
  // open tag, contents, and closing tag
  fn parse_element(&mut self) -> dom::Node {
    // Opening tag
    assert!(self.consume_char() == '<');
    let tag_name = self.parse_tag_name();
    let attrs = self.parse_attributes();
    assert!(self.consume_char() == '>');

    // Contents
    let children = self.parse_nodes();

    // Closing tag
    assert!(self.consume_char() == '<');
    assert!(self.consume_char() == '/');
    assert!(self.parse_tag_name() == tag_name);
    assert!(self.consume_char() == '>');

    return dom::elem(tag_name, attrs, children);
  }

  // Parse a single name="value" pair
  fn parse_attr(&mut self) -> (String, String) {
    let name = self.parse_tag_name();
    assert!(self.consume_char() == '=');
    let value = self.parse_attr_value();
    return (name, value);
  }

  // Parse a quoted value
  fn parse_attr_value(&mut self) -> String {
    let open_quote = self.consume_char();
  }

  // Parse a list of name="value" pairs, separated by whitespace
  fn parse_attributes(&mut self) -> dom::AttrMap {
    let mut attributes = HashMap::new();
    loop {
      self.consume_whitespace();
      if self.next_char() == '>' {
        break;
      }
      let (name, value) = self.parse_attr();
      attributes.insert(name, value);
    }
    return attributes;
  }

  // Parse a sequence of sibling nodes
  fn parse_nodes(&mut self) -> Vec<dom::Node> {
    let mut nodes = vec!();
    loop {
      self.consume_whitespace();
      if self.eof() || self.starts_with("</") {
        break;
      }
      nodes.push(self.parse_node());
    }
    return nodes;
  }

  // Parse an HTML document and return the root element
  pub fn parse(source: String) -> dom::Node {
    let mut nodes = Parser { pos: 0u, input: source }.parse_nodes();

    // If the document contains a root element, return it
    // Otherwise, create one
    if nodes.len() == 1 {
      nodes.swap_remove(0).unwrap()
    } else {
      dom::elem("html".to_string(), HashMap::new(), nodes)
    }
  }
}


