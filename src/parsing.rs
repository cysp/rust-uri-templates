use std;


mod component_scanner {
    use std;

    #[derive(Debug)]
    pub enum Error {
        UnexpectedEndOfData,
        InvalidCharacter,
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match *self {
                Error::UnexpectedEndOfData => write!(f, "ComponentScanner::Error::UnexpectedEndOfData"),
                Error::InvalidCharacter => write!(f, "ComponentScanner::Error::InvalidCharacter"),
            }
        }
    }

    impl std::error::Error for Error {
        fn description(&self) -> &'static str {
            match *self {
                Error::UnexpectedEndOfData => "unexpected end of data",
                Error::InvalidCharacter => "invalid character",
            }
        }
    }

    pub type Result<T> = std::result::Result<T, Error>;

    #[derive(Copy,Clone,Debug,PartialEq,Eq)]
    pub enum ComponentType {
        Literal,
        Variable,
    }

    pub struct ComponentScanner<'a> {
        buf: &'a [u8],
        cur: usize,
    }

    impl<'a> ComponentScanner<'a> {
        pub fn new(buf: &'a [u8]) -> Self {
            ComponentScanner {
                buf: buf,
                cur: 0,
            }
        }

        pub fn next_component(&mut self) -> Result<Option<(ComponentType, std::ops::Range<usize>)>> {
            enum State {
                AfterAtLeastOneLiteralCharacter,
                AfterOpenBrace,
            }

            let start = self.cur;
            let mut state = None;

            let mut cur = start;
            for c in &self.buf[start..] {
                println!("cur, c: {}, {}", cur, c);
                cur += 1;
                state = Some(match state {
                    None => {
                        match *c as char {
                            '{' => State::AfterOpenBrace,
                            _ => State::AfterAtLeastOneLiteralCharacter,
                        }
                    }
                    Some(State::AfterAtLeastOneLiteralCharacter) => {
                        match *c as char {
                            '{' => {
                                self.cur = cur - 1;
                                println!("literal");
                                return Ok(Some((ComponentType::Literal, start..(cur - 1))));
                            }
                            _ => state.unwrap(),
                        }
                    }
                    Some(State::AfterOpenBrace) => {
                        match *c as char {
                            '}' => {
                                self.cur = cur;
                                println!("variable");
                                return Ok(Some((ComponentType::Variable, (start + 1)..(cur - 1))));
                            }
                            _ => state.unwrap(),
                        }
                    }
                });
            }
            match state {
                None => Ok(None),
                Some(State::AfterAtLeastOneLiteralCharacter) => {
                    self.cur = cur;
                    println!("literal to eod");
                    Ok(Some((ComponentType::Literal, start..cur)))
                }
                Some(State::AfterOpenBrace) => {
                    return Err(Error::UnexpectedEndOfData);
                }
            }
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_simple() {
            let s = "hi{hello}hey{?yeah,okay}";
            let mut p = ComponentScanner::new(s.as_bytes());

            let c = p.next_component().unwrap().unwrap();
            assert_eq!(c, (ComponentType::Literal, 0..2));
            assert_eq!(&s[c.1], "hi");
            let c = p.next_component().unwrap().unwrap();
            assert_eq!(c, (ComponentType::Variable, 3..8));
            assert_eq!(&s[c.1], "hello");
            let c = p.next_component().unwrap().unwrap();
            assert_eq!(c, (ComponentType::Literal, 9..12));
            assert_eq!(&s[c.1], "hey");
            let c = p.next_component().unwrap().unwrap();
            assert_eq!(c, (ComponentType::Variable, 13..23));
            assert_eq!(&s[c.1], "?yeah,okay");
            let c = p.next_component().unwrap();
            assert_eq!(c, None);
        }
    }
}
