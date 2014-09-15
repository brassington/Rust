struct Stylesheet {
  rules: Vec<Rule>,
}

struct Rule {
  selectors: Vec<Selector>,
  declarations: Vec<Declaration>,
}

enum Selector {
  Simple(SimpleSelector),
}

struct SimpleSelector {
  tag_name: Option<String>,
  id: Option<String>,
  class: Vec<String>,
}

struct Declaration {
  name: String, 
  value: String,
}

enum Value {
  Keyword(String),
  Color(u8, u8, u8, u8),
  Length(f32, uint),
  // insert more values
}

impl Value {
  // Return the size of a length in px, or zero for non-lengths
  pub fn to_px(&self) -> f32 {
    match *self {
      Length(f, Px) => f,
      _ => 0.0
    }
  }
}

enum Unit {
  Px,
  // inser more units 
}

struct Parser {
  pos: uint,
  input: String,
}

// Parse a simple selector
impl Parser {
  // Parse a list of rule sets, separated by optional whitespace
  fn parse_rules(&mut self) -> Vec<Rule> {
    let mut rules = Vec::new();
    loop {
      self.consume_whitespace();
      if self.eof() { break }
      rules.push(self.parse_rule());
    }
    return rules;
  }


  fn parse_simple_selector(&mut self) -> SimpleSelector {
    let mut selector = SimpleSelector { tag_name: None,
                                        id: None,
                                        class: Vec::new() };
    while !self.eof() {
      match self.next_char() {
        '#' => {
          self.consume_char();
          selector.id = Some(self.parse_identifier());
        }
        '.' => {
          self.consume_char();
          selector.class.push(self.parse_identifier());
        }
        '*' => {
          // universal selector
          self.consume_char();
        }
        c if valid_identifier_char(c) => {
          selector.tag_name = Some(self.parse_identifier());
        }
        _ => break
      }
    }
    return selector;
  }

  // Parse a whole CSS stylesheet
  pub fn parse(source: String) -> Stylesheet {
    let mut parser = Parser { pos: 0u, input: source };
    Stylesheet { rules: parser.parse_rules() }
  }

  // Parse a css rule
  pub fn parse_rule(&mut self) -> Rule {
    Rule {
      selectors: self.parse_selectors();
      declarations: self.parse_declarations();
    }
  }

  // Parse a comma-separate list of selectors
  fn parse_selectors(&mut self) -> Vec<Selector> {
    let mut selectors = Vec::new();
    loop {
      selectors.push(Simple(self.parse_simple_selector()));
      self.consume_whitespace();
      match self.next_char() {
        ',' => { self.consume_char();
                 self.consume_whitespace(); }
        '{' => break,   // start of declarations
         c  => fail!("Unexpected character {} in selector list", c)
      }
    }
    // Return selectors with highest specificity first, for use in matching
    selectors.sort_by(|a, b| b.specificity().cmp(&a.specificity()));
    return selectors;
  }

  // Parse one `<property>: <value>;` declaration
  fn parse_declaration(&mut self) -> Declaration {
    let property_name = self.parse_identifier();
    self.consume_whitespace();
    assert!(self.consume_char() == ':')
    self.consume_whitespace();
    assert!(self.consume_char() == ';')

    Declaration {
      name: property_name,
      value: value,
    }

    // Methods for parsing values:

    fn parse_value(&mut self) -> Value {
      match self.next_char() {
        '0'..'9' => self.parse_length(),
        '#' => self.parse_color(),
        _ => Keyword(self.parse_identifier())
      }
    }

    fn parse_length(&mut self) -> Value {
      Length(self.parse_float(), self.parse_uint())
    }

    fn parse_float(&mut self) -> f32 {
      let s = self.consume_while(|c| match c {
        '0'..'9' | '.' => true,
        _ => false
      });
      let f: Option<f32> = FromStr::from_str(s.as_slice());
      f.unwrap()
    }

    fn parse_unit(&mut self) -> Unit {
      match self.parse_identifier().into_ascii_lower().as_slice() {
        "px" => Px,
        _ => fail!("unrecognized unit")
      }
    }

    fn parse_color(&mut self) -> Value {
      assert!(self.consume_char() == '#');
      Color(self.parse_hex_pair(), self.parse_hex_pair(), self.parse_hex_pair(), 255)
    }

    // Parse two hexadecimal digits
    fn parse_hex_pair(&mut self) -> u8 {
      let s = self.input.as_slice().slice(self.pos, self.pos + 2);
      self.pos = self.pos + 2;
      FromStrRadix::from_str_radix(s, 0x10).unwrap()
    }

    // Parse a property name or keyword
    fn parse_identifier(&mut self) -> String {
      self.consume_while(valid_identifier_char)
    }
  }

  // Parse a list of declarations enclose in `{...}`
  fn parse_declarations(&mut self) -> Vec<Declaration> {
    assert!(self.consume_char() == '{');
    let mut declarations = Vec::new();
    loop {
      self.consume_whitespace();
      if self.next_char() == '}' {
        self.consume_char();
        break;
      }
      declarations.push(self.parse_declarations());
    }
    return declarations;
  }
}

// Specificity rules 
pub type Specificity = (uint, uint, uint);

impl Selector {
  pub fn specificity(&self) -> Specificity {
    let Simple(ref simple) = *self;
    let a = simple.id.iter().len();
    let b = simple.class.len();
    let c = simple.tag_name.iter().len();
    (a, b, c)
  }
}

fn valid_identifier_char(c: char) -> bool {
  match c {
    'a'..'z' | 'A'..'Z' | '0'..'9' | '-' | '_' => true,
    _ => false,
  }
}





