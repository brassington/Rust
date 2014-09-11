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
    
  }
}


