use super::types::{UriTemplate, UriTemplateComponent, UriTemplateOperator, UriTemplateVariable};

use automaton::{Automaton, Expression, Token};
use automaton::encoding::Ascii;


struct ParseState {
	t: u32,
}


fn expression_hex_digit<S>() -> Expression<Ascii, S> {
	let digits = Ascii::of(Ascii::from_range(&(('0' as u32)..(('9' as u32)+1))));
	let lower_alphas = Ascii::of(Ascii::from_range(&(('a' as u32)..(('f' as u32)+1))));
	let upper_alphas = Ascii::of(Ascii::from_range(&(('A' as u32)..(('F' as u32)+1))));
	digits.union(lower_alphas).union(upper_alphas)
}

fn expression_uritemplate_ucschar<S>() -> Expression<Ascii, S> {
// ucschar        =  %xA0-D7FF / %xF900-FDCF / %xFDF0-FFEF
// 			/  %x10000-1FFFD / %x20000-2FFFD / %x30000-3FFFD
// 			/  %x40000-4FFFD / %x50000-5FFFD / %x60000-6FFFD
// 			/  %x70000-7FFFD / %x80000-8FFFD / %x90000-9FFFD
// 			/  %xA0000-AFFFD / %xB0000-BFFFD / %xC0000-CFFFD
// 			/  %xD0000-DFFFD / %xE1000-EFFFD
//
	let e1 = Ascii::from_range(&(0xa0..0xd800));
	Expression::token(e1)
}

fn expression_uritemplate_iprivate<S>() -> Expression<Ascii, S> {
// iprivate       =  %xE000-F8FF / %xF0000-FFFFD / %x100000-10FFFD
	let e1 = Ascii::of(Ascii::from_range(&(0xe000..0xf900)));
	let e2 = Ascii::of(Ascii::from_range(&(0xf0000..(0xfffd+1))));
	let e3 = Ascii::of(Ascii::from_range(&(0x100000..(0x10fffd+1))));
	e1.union(e2).union(e3)
}

fn expression_uritemplate_literals<S>() -> Expression<Ascii, S> {
// literals      =  %x21 / %x23-24 / %x26 / %x28-3B / %x3D / %x3F-5B
// /  %x5D / %x5F / %x61-7A / %x7E / ucschar / iprivate
// /  pct-encoded
// ; any Unicode character except: CTL, SP,
// ;  DQUOTE, "'", "%" (aside from pct-encoded),
// ;  "<", ">", "\", "^", "`", "{", "|", "}"
	let e1 = Ascii::of('\x21');
	let e2 = Ascii::of(Ascii::from_range(&(0x23..(0x24+1))));
	let e3 = Ascii::of('\x26');
	let e4 = Ascii::of(Ascii::from_range(&(0x28..(0x3b+1))));
	let e5 = Ascii::of('\x3d');
	let e6 = Ascii::of(Ascii::from_range(&(0x3f..(0x5b+1))));
	let e7 = Ascii::of('\x5d');
	let e8 = Ascii::of('\x5f');
	let e9 = Ascii::of(Ascii::from_range(&(0x61..(0x7a+1))));
	let ea = Ascii::of('\x7e');
	let eb = expression_uritemplate_ucschar();
	let ec = expression_uritemplate_iprivate();
	e1.union(e2).union(e3).union(e4).union(e5).union(e6).union(e7).union(e8).union(e9).union(ea).union(eb).union(ec)
}

pub fn parse(s: &str) -> Result<UriTemplate, ()> {
	let percent_encoded = Ascii::of('%').concat(expression_hex_digit()).concat(expression_hex_digit());

	let literal_component = expression_uritemplate_literals().union(percent_encoded);
	let variable_component = Ascii::of('{').concat(Ascii::any().intersection(Ascii::of('}'))).concat(Ascii::of('}'));

	let a: Automaton<Ascii, ParseState> = literal_component.optional().union(variable_component.optional()).kleene().compile();

	let mut ps = ParseState{ t: 0 };
	if !a.parse(&mut ps, s) {
		return Err(());
	}

	Err(())
}


// // pub struct Parser {
// //     enum Token {
// //         Component()
// //     }
//
// //     pub fn parse_byte(&self, ctx: &)
// //     pub fn from_str(template: &str) -> Parser {
//
// //     }
// // }
//
// // #[cfg(test)]
// // mod test {
// //     use std::collections::HashMap;
// //     use super::UriTemplate;
// //
// //     #[test]
// //     fn test_1() {
// //         let t = match UriTemplate::from_str("http://example.com/{?q}") {
// //             Some(t) => t,
// //             None => return,
// //         };
// //
// //         let u = t.to_url().expect("");
// //         assert_eq!(u.serialize(), "http://example.com/");
// //     }
// //
// //     #[test]
// //     fn test_2() {
// //         let t = match UriTemplate::from_str("http://example.com/{?q}") {
// //             Some(t) => t,
// //             None => return,
// //         };
// //
// //         let mut v = HashMap::<&str, &str>::new();
// //         v.insert("q", "something");
// //
// //         let u = t.to_url_with_values(v).expect("");
// //         assert_eq!(u.serialize(), "http://example.com/?q=something");
// //     }
// //
// // }
//
// use std;
//
//
// #[derive(Clone,Debug)]
// struct Literal {
// 	range: std::ops::Range<usize>,
// }
//
// #[derive(Clone,Debug)]
// struct Expression {
// 	operator: Option<char>,
// 	variables: Vec<Variable>,
// }
//
// #[derive(Clone,Debug)]
// struct Variable {
// 	name_range: std::ops::Range<usize>,
// 	explode: bool,
// 	prefix: Option<usize>,
// }
//
// #[derive(Clone,Debug)]
// enum ParseComponent {
// 	Literal(Literal),
// 	Expression(Expression),
// }
//
// pub fn parse(s: &str) -> Result<(), ()> {
// 	#[derive(Clone,Debug)]
// 	enum State {
// 		Literal(usize),
// 		ExpectingCloseBrace,
// 		AfterCloseBrace,
// 	}
//
// 	let mut p = 0;
// 	let mut state = State::Literal(p);
//
// 	let mut components: Vec<ParseComponent> = Vec::new();
//
// 	let mut current_expression_operator: Option<char> = None;
// 	let mut current_expression_variables: Vec<Variable> = Vec::new();
//
// 	let mut it = s.char_indices().peekable();
// 	while let Some((idx, c)) = it.next() {
// 		println!("{:?}: (idx, c): ({:?}, {:?})", state, idx, c);
//
// 		match state {
// 			State::AfterCloseBrace => {
// 				// test c for validity
// 				state = State::Literal(idx);
// 			}
// 			State::Literal(start) => {
// 				match c {
// 					'{' => {
// 						components.push(ParseComponent::Literal(Literal { range: start..idx+1 }));
// 						match parse_expression(&mut it) {
// 							Ok((operator, variables)) => {
// 								current_expression_operator = operator;
// 								current_expression_variables = variables;
// 							}
// 							Err(e) => return Err(e),
// 						}
// 						state = State::ExpectingCloseBrace;
// 					}
// 					c => {
// 						// test c for validity
// 					}
// 				}
// 			}
// 			State::ExpectingCloseBrace => {
// 				match c {
// 					'}' => {
// 						// let variables: Vec<Variable> = current_expression_variables.drain().collect();
// 						components.push(ParseComponent::Expression(Expression {
// 							operator: current_expression_operator,
// 							variables: current_expression_variables,
// 						}));
// 						current_expression_operator = None;
// 						current_expression_variables = Vec::new();
// 						state = State::AfterCloseBrace;
// 					}
// 					_ => panic!(),
// 				}
// 			}
// 		}
// 	}
//
// 	match state {
// 		State::Literal(start) => {
// 			components.push(ParseComponent::Literal(Literal { range: start..s.len() }));
// 		}
// 		State::ExpectingCloseBrace => panic!(),
// 		State::AfterCloseBrace => {}
// 	}
//
// 	println!("components: {:?}", components);
//
// 	Err(())
// }
//
//
// enum ParseExpressionState {
// 	ExpectVariableName(bool),
// 	VariableName(usize),
// 	After
// }
//
// fn parse_expression(it: &mut std::iter::Peekable<std::str::CharIndices>) -> Result<(Option<char>, Vec<Variable>), ()> {
// 	let operator = match it.peek() {
// 		Some(&(_, '+')) => {
// 			it.next();
// 			Some('+')
// 		}
// 		None => return Err(()),
// 	};
//
// 	enum State {
// 		ExpectVariableName(bool),
// 		VariableName(usize),
// 		AfterColon,
// 		PrefixModifier(usize),
// 		AfterCloseBrace,
// 	}
//
// 	let state = State::ExpectVariableName(true);
// 	let mut variables: Vec<Variable> = Vec::new();
// 	let mut current_variable: Option<Variable> = None;
//
// 	while let Some((idx, c)) = it.next() {
// 		if c == '}' {
// 			match state {
// 				State::ExpectVariableName(required) => {
// 					if required {
// 						panic!();
// 					}
// 				}
// 				State::VariableName(start) => {
// 				}
// 				State::AfterColon => panic!(),
// 				State::PrefixModifier(start) => {
//
// 				}
// 				// State::AfterComma => panic!(),
// 				State::AfterCloseBrace => panic!(),
// 			}
// 			state = State::AfterCloseBrace;
// 			break;
// 		}
//
// 		match state {
// 			State::ExpectVariableName(required) => {
// 				match c {
// 					'a'|'b'|'c'|'d'|'e'|'f'|'g'|'h'|'i'|'j'|'k'|'l'|'m'|'n'|'o'|'p'|'q'|'r'|'s'|'t'|'u'|'v'|'w'|'x'|'y'|'z' => {
// 						state = State::VariableName(idx);
// 					}
// 					_ => panic!(),
// 				}
// 			}
// 			// State::AfterOperator => {
// 			// 	// test c for validity
// 			// 	state = State::VariableName(idx);
// 			// }
// 			State::VariableName(start) => {
// 				match c {
// 					':' => {
// 						state = State::AfterColon;
// 					}
// 					'*' => {
// 						match current_variable.as_mut() {
// 							Some(v) => {
// 								(*v).explode = true
// 							}
// 							None => panic!(),
// 						}
// 						state = State::AfterAsterisk;
// 					}
// 					',' => {
// 						state = State::AfterComma;
// 					}
//
// 				}
// 			}
// 			State::AfterColon => {
// 				// test c for validity
// 				state = State::VariableName(idx);
// 			}
// 			State::PrefixModifier(start) => {}
// 			// State::AfterComma => {
// 			// 	// test c for validity
// 			// 	state = State::VariableName(idx);
// 			// }
// 			State::AfterCloseBrace => panic!(),
// 		}
// 		match c {
// 			'}' => {
// 				state = State::AfterCloseBrace;
// 			}
// 		}
// 	}
//
// 	match state {
// 		State::OperatorOrVariableName(start) => {
//
// 		}
// 		// State::AfterOperator => panic!(),
// 		State::VariableName(start) => {
//
// 		}
// 		State::AfterColon => panic!(),
// 		State::PrefixModifier(start) => panic!(),
// 		State::AfterCloseBrace => panic!(),
// 	}
//
// 	Ok((operator, variables))
// 	// Err(())
// }
//
// fn parse_variable(it: &mut std::iter::Peekable<std::str::CharIndices>) -> Result<Variable, ()> {
// 	Err(())
// }
//
// #[cfg(test)]
// mod test {
// 	use super::*;
//
// 	#[test]
// 	pub fn test1() {
//         let _ = parse("foo{bar}baz/{?quux}");
// 	}
// }
//
// // mod lexing {
// // 	use std;
// //
// // 	#[derive(Clone,PartialEq,Eq,Debug)]
// // 	pub enum Token<'a> {
// // 		OpenBrace,
// // 		CloseBrace,
// // 		Semicolon,
// // 		Asterisk,
// // 		Comma,
// // 		String(&'a str),
// // 	}
// //
// // 	#[derive(Copy,Clone,PartialEq,Eq,Debug)]
// // 	pub enum Character {
// // 		OpenBrace,
// // 		Semicolon,
// // 		Asterisk,
// // 		Comma,
// // 		CloseBrace,
// // 		Character(char),
// // 	}
// //
// // 	pub struct Lexer<'a> {
// // 		s: &'a str,
// // 		finished: bool,
// // 		char_indices: std::str::CharIndices<'a>,
// // 	}
// //
// // 	impl<'a> Lexer<'a> {
// // 		pub fn new(s: &'a str) -> Lexer<'a> {
// // 			Lexer {
// // 				s: s,
// // 				finished: false,
// // 				char_indices: s.char_indices(),
// // 			}
// // 		}
// //
// // 		fn classify_char(c: char) -> Character {
// // 			match c {
// // 				'{' => Character::OpenBrace,
// // 				'}' => Character::CloseBrace,
// // 				':' => Character::Semicolon,
// // 				'*' => Character::Asterisk,
// // 				',' => Character::Comma,
// // 				c => Character::Character(c),
// // 			}
// // 		}
// // 	}
// //
// // 	impl<'a> Iterator for Lexer<'a> {
// // 		type Item = Token<'a>;
// //
// // 		fn next(&mut self) -> Option<Token<'a>> {
// // 			if self.finished {
// // 				return None;
// // 			}
// //
// // 			// let string_range = self.char_indices.take_while(|&(i, c)|
// // 			// 	match Lexer::classify_char(c) {
// // 			// 		Character::OpenBrace => false,
// // 			// 		Character::CloseBrace => false,
// // 			// 		Character::Semicolon => false,
// // 			// 		Character::Asterisk => false,
// // 			// 		Character::Comma => false,
// // 			// 		Character::Character(_) => true,
// // 			// 	}
// // 			// ).map(|(i, c)| i).fold((0usize,0usize), |acc, i| {
// // 			// 	if acc.1 == 0 || i < acc.0 {
// // 			// 		acc.0 = i
// // 			// 	}
// // 			// 	if i > acc.1 {
// // 			// 		acc.1 = i
// // 			// 	}
// // 			// 	acc
// // 			// });
// //
// // 			let (first_pos, first_c) = match self.char_indices.next() {
// // 				None => {
// // 					self.finished = true;
// // 					return None;
// // 				},
// // 				Some((i, c)) => (i, c),
// // 			};
// //
// // 			match Lexer::classify_char(first_c) {
// // 				Character::OpenBrace => return Some(Token::OpenBrace),
// // 				Character::CloseBrace => return Some(Token::CloseBrace),
// // 				Character::Semicolon => return Some(Token::Semicolon),
// // 				Character::Asterisk => return Some(Token::Asterisk),
// // 				Character::Comma => return Some(Token::Comma),
// // 				Character::Character(_) => (),
// // 			}
// //
// // 			let mut string_range = (first_pos, 0usize);
// // 			loop {
// // 			let (pos, c) = match self.char_indices.next() {
// // 				None => {
// // 					self.finished = true;
// // 					break;
// // 				},
// // 				Some((i, c)) => (i, c),
// // 			};
// // break;
// // 			}
// //
// // 			match string_range {
// // 				(_, 0) => (),
// // 				(pos, len) => return Some(Token::String(&self.s[pos..(pos+len)])),
// // 			}
// //
// // 			let (pos, c) = match self.char_indices.next() {
// // 				None => {
// // 					self.finished = true;
// // 					return None;
// // 				},
// // 				Some((i, c)) => (i, c),
// // 			};
// //
// // 			let t = match c {
// // 				'{' => Some(Token::OpenBrace),
// // 				'}' => Some(Token::CloseBrace),
// // 				':' => Some(Token::Semicolon),
// // 				'*' => Some(Token::Asterisk),
// // 				',' => Some(Token::Comma),
// // 				_ => None
// // 			};
// //
// // 			if t.is_some() {
// // 				return t
// // 			}
// //
// // 			None
// // 		}
// // 	}
// //
// // 	#[cfg(test)]
// // 	mod test {
// // 		use super::*;
// //
// // 		#[test]
// // 		pub fn test1() {
// // 			let mut l = Lexer::new("foo");
// // 			assert_eq!(l.next(), Some(Token::String("foo")));
// // 		}
// // 	}
// // }
