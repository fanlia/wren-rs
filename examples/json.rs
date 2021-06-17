
use std::str::FromStr;
use std::iter::Peekable;
use std::collections::BTreeMap;

type JSONMap = BTreeMap<String, JSON>;

#[derive(Debug)]
pub enum JSON {
    Object(Box<JSONMap>),
    Array(Vec<JSON>),
    String(String),
    Number(f64),
    True,
    False,
    Null,
}

struct Parser<I>
where I: Iterator<Item=char>
{
    chars: Peekable<I>,
}

impl<I> Parser<I>
where I: Iterator<Item=char>
{
    pub fn parse(&mut self) -> JSON {
	self.parse_element()
    }

    fn parse_value(&mut self) -> JSON {
	match self.chars.peek() {
	    Some(&ch) if ch == '{'  => {
		JSON::Object(self.parse_object())
	    }
	    Some(&ch) if ch == '[' => {
		JSON::Array(self.parse_array())
	    }
	    Some(&ch) if ch == '"' => {
		JSON::String(self.parse_string())
	    }
	    Some(&ch) if ch == '-' => {
		JSON::Number(self.parse_number())
	    }
	    Some(ch) if ch.is_digit(10) => {
		JSON::Number(self.parse_number())
	    }
	    Some(_) => {
		let keyword = self.parse_keyword();
		match &keyword[..] {
		    "true" => JSON::True,
		    "false" => JSON::False,
		    "null" => JSON::Null,
		    _ => JSON::String(keyword),
		}
	    }
	    None => JSON::Null
	}
    }

    fn parse_char(&mut self, ch: char) -> bool {
	if let Some(&r) = self.chars.peek() {
	    if r == ch {
		self.chars.next();
		return true
	    }
	}
	false
    }

    fn parse_object(&mut self) -> Box<JSONMap> {
	let mut object = Box::new(JSONMap::new());
	self.parse_char('{');
	
	while let Some((key, value)) = self.parse_members() {
	    object.insert(key, value);
	}
	self.parse_char('}');

	object
    }

    fn parse_members(&mut self) -> Option<(String, JSON)> {
	self.parse_ws();
	match self.chars.peek() {
	    None => None,
	    Some(&ch) if ch == '}' => None,
	    _ => Some(self.parse_member()),
	}
    }

    fn parse_member(&mut self) -> (String, JSON) {
	self.parse_ws();
	let key = self.parse_string();
	self.parse_ws();
	self.parse_char(':');
	let value = self.parse_element();
	self.parse_char(',');
	(key, value)
    }

    fn parse_array(&mut self) -> Vec<JSON> {
	let mut array = Vec::new();
	self.parse_char('[');

	while let Some(value) = self.parse_elements() {
	    array.push(value);
	}
	self.parse_char(']');

	array
    }

    fn parse_elements(&mut self) -> Option<JSON> {
	self.parse_ws();
	match self.chars.peek() {
	    None => None,
	    Some(&ch) if ch == ']' => None,
	    _ => Some(self.parse_element()),
	}
    }

    fn parse_element(&mut self) -> JSON {
	self.parse_ws();
	let json = self.parse_value();
	self.parse_ws();
	self.parse_char(',');
	json
    }

    fn parse_string(&mut self) -> String {
	let mut string = String::new();
	let with_quote = self.parse_char('"');
	loop {
	    match self.chars.peek() {
		None => break,
		Some(&ch) => {
		    if with_quote {
			if ch == '"' {
			    break;
			}
		    } else {
			if !ch.is_alphanumeric() {
			    break;
			}
		    }
		    string.push(ch);
		}
	    }
	    self.chars.next();
	}
	self.parse_char('"');
	string
    }

    fn parse_number(&mut self) -> f64 {
	let mut string = String::new();

	// parse integer
	if let Some(&ch) = self.chars.peek() {
	    if ch == '-' {
		string.push('-');
		self.chars.next();
	    }
	}

	while let Some(&ch) = self.chars.peek() {
	    if ch.is_digit(10) {
		string.push(ch);
	    } else {
		break;
	    }
	    self.chars.next();
	}
	// parse fraction
	if let Some(&ch) = self.chars.peek() {
	    if ch == '.' {
		string.push('.');
		self.chars.next();
	    }
	}

	while let Some(&ch) = self.chars.peek() {
	    if ch.is_digit(10) {
		string.push(ch);
	    } else {
		break;
	    }
	    self.chars.next();
	}
	
	// parse exponent
	if let Some(&ch) = self.chars.peek() {
	    if ch == 'e' || ch == 'E' {
		string.push('e');
		self.chars.next();
	    }
	}
	if let Some(&ch) = self.chars.peek() {
	    if ch == '-' || ch == '+' {
		string.push(ch);
		self.chars.next();
	    }
	}
	while let Some(&ch) = self.chars.peek() {
	    if ch.is_digit(10) {
		string.push(ch);
	    } else {
		break;
	    }
	    self.chars.next();
	}
	
	f64::from_str(&string).unwrap()
    }

    fn parse_keyword(&mut self) -> String {
	let mut string = String::new();
	self.parse_ws();

	while let Some(&ch) = self.chars.peek() {
	    if !ch.is_alphanumeric() {
		break;
	    } else {
		string.push(ch);
	    }
	    self.chars.next();
	}

	string
    }

    fn parse_ws(&mut self) {
	while let Some(ch) = self.chars.peek() {
	    if !ch.is_whitespace() && !ch.is_control() {
		break;
	    }
	    self.chars.next();
	}
    }
}

fn main() {
    let json_string = r#"
{
"o\nbject": {"key": "value"},
"array": [1, 2],
"string": "this is a \nstring",
"number": 3.14,
"true": true,
"false": false,
"null": null

}
"#;
    println!("{}", json_string);
    let mut parser = Parser {
	chars: json_string.chars().peekable(),
    };
    let json = parser.parse();
    println!("{:#?}", json);
    println!("json is ok");
}
