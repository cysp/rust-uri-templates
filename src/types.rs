extern crate std;
extern crate url;

use std::vec::Vec;
use std::collections::BitvSet;
use std::collections::HashMap;

// use url::percent_encoding;


#[deriving(Copy)]
pub enum UriTemplateOperator {
    ReservedCharacter,
    Fragment,
    PathExtension,
    PathComponent,
    PathParameter,
    QueryParameter,
    QueryContinuation,
}


pub enum UriTemplateVariable {
    Simple(String),
    Prefix(String, u32),
    Explode(String),
    ExplodePrefix(String, u32),
}

#[deriving(Copy)]
pub enum UriTemplateModifier {
    Prefix(u32),
    Explode,
    ExplodePrefix(u32),
}


impl UriTemplateVariable {
    pub fn into_template_string(self) -> String {
        match self {
            UriTemplateVariable::Simple(name) => name,
            UriTemplateVariable::Prefix(name, prefix) => format!("{}:{}", name, prefix),
            UriTemplateVariable::Explode(name) => format!("{}*", name),
            UriTemplateVariable::ExplodePrefix(name, prefix) => format!("{}*:{}", name, prefix),
        }
    }

    pub fn to_template_string(&self) -> String {
        match self {
            &UriTemplateVariable::Simple(ref name) => name.clone(),
            &UriTemplateVariable::Prefix(ref name, prefix) => format!("{}:{}", name, prefix),
            &UriTemplateVariable::Explode(ref name) => format!("{}*", name),
            &UriTemplateVariable::ExplodePrefix(ref name, prefix) => format!("{}*:{}", name, prefix),
        }
    }
}


pub enum UriTemplateComponent {
    Literal(String),
    Variable(Option<UriTemplateOperator>, Vec<UriTemplateVariable>),
}

#[deriving(Copy)]
enum UriTemplateEscaping {
    U,
    UR,
}


fn escape_string(method: UriTemplateEscaping, input: &str) -> String {
    // unreserved     =  ALPHA / DIGIT / "-" / "." / "_" / "~"
    // reserved       =  gen-delims / sub-delims
    // gen-delims     =  ":" / "/" / "?" / "#" / "[" / "]" / "@"
    // sub-delims     =  "!" / "$" / "&" / "'" / "(" / ")"
    // /  "*" / "+" / "," / ";" / "=" / "%"

    let str_u = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-._~";
    let str_ur = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-._~:/?#[]@!$&'()*+,;=%";

    let mut set_u = BitvSet::new();
    for b in str_u.as_bytes().iter() {
        set_u.insert(*b as uint);
    }
    let set_u = set_u;

    let mut set_ur = BitvSet::new();
    for b in str_ur.as_bytes().iter() {
        set_ur.insert(*b as uint);
    }
    let set_ur = set_ur;

    let mut s = String::new();
    for byte in input.as_bytes().iter() {
        match method {
            UriTemplateEscaping::U => {
                if set_u.contains(&(*byte as uint)) {
                    s.push(*byte as char);
                } else {
                    s.push_str(format!("%{:02X}", *byte).as_slice())
                }
            }
            UriTemplateEscaping::UR => {
                if set_ur.contains(&(*byte as uint)) {
                    s.push(*byte as char);
                } else {
                    s.push_str(format!("%{:02X}", *byte).as_slice())
                }
            }
        }
    }
    s
    // let encode_set = match op {
    //     None => url::percent_encoding::DEFAULT_ENCODE_SET,
    //     Some(UriTemplateOperator::ReservedCharacter) => url::percent_encoding::SIMPLE_ENCODE_SET,
    //     Some(UriTemplateOperator::Fragment) => url::percent_encoding::DEFAULT_ENCODE_SET,
    //     Some(UriTemplateOperator::PathExtension) => url::percent_encoding::DEFAULT_ENCODE_SET,
    //     Some(UriTemplateOperator::PathComponent) => url::percent_encoding::DEFAULT_ENCODE_SET,
    //     Some(UriTemplateOperator::PathParameter) => url::percent_encoding::DEFAULT_ENCODE_SET,
    //     Some(UriTemplateOperator::QueryParameter) => url::percent_encoding::QUERY_ENCODE_SET,
    //     Some(UriTemplateOperator::QueryContinuation) => url::percent_encoding::QUERY_ENCODE_SET,
    // };
    // url::percent_encoding::percent_encode(s.as_bytes(), encode_set)
}

impl UriTemplateComponent {
    pub fn to_template_string(&self) -> String {
        match self {
            &UriTemplateComponent::Literal(ref value) => value.clone(),
            &UriTemplateComponent::Variable(operator, ref variables) => {
                let varspecs: Vec<String> = variables.iter().map(|v|
                    v.to_template_string()
                ).collect();

                let operator: &'static str = operator.map(|o|
                    match o {
                        UriTemplateOperator::ReservedCharacter => "+",
                        UriTemplateOperator::Fragment => "#",
                        UriTemplateOperator::PathExtension => ".",
                        UriTemplateOperator::PathComponent => "/",
                        UriTemplateOperator::PathParameter => ";",
                        UriTemplateOperator::QueryParameter => "?",
                        UriTemplateOperator::QueryContinuation => "&",
                    }
                ).unwrap_or("");
                format!("{{{}{}}}", operator, varspecs.connect(","))
            },
        }
    }

    pub fn into_template_string(self) -> String {
        match self {
            UriTemplateComponent::Literal(value) => value,
            UriTemplateComponent::Variable(operator, variables) => {
                let varspecs: Vec<String> = variables.into_iter().map(|v|
                    v.into_template_string()
                ).collect();

                let operator: &'static str = operator.map(|o|
                    match o {
                        UriTemplateOperator::ReservedCharacter => "+",
                        UriTemplateOperator::Fragment => "#",
                        UriTemplateOperator::PathExtension => ".",
                        UriTemplateOperator::PathComponent => "/",
                        UriTemplateOperator::PathParameter => ";",
                        UriTemplateOperator::QueryParameter => "?",
                        UriTemplateOperator::QueryContinuation => "&",
                    }
                ).unwrap_or("");
                format!("{{{}{}}}", operator, varspecs.connect(","))
            },
        }
    }

    pub fn to_string_with_values(&self, values: &UriTemplateValues) -> String {
        match self {
            &UriTemplateComponent::Literal(ref value) => escape_string(UriTemplateEscaping::UR, value.as_slice()),
            &UriTemplateComponent::Variable(operator, ref variables) => {
                let prefix: &'static str = operator.map(|o|
                    match o {
                        UriTemplateOperator::ReservedCharacter => "",
                        UriTemplateOperator::Fragment => "#",
                        UriTemplateOperator::PathExtension => ".",
                        UriTemplateOperator::PathComponent => "/",
                        UriTemplateOperator::PathParameter => ";",
                        UriTemplateOperator::QueryParameter => "?",
                        UriTemplateOperator::QueryContinuation => "&",
                    }
                ).unwrap_or("");

                let separator: &'static str = operator.map(|o|
                    match o {
                        UriTemplateOperator::ReservedCharacter => ",",
                        UriTemplateOperator::Fragment => ",",
                        UriTemplateOperator::PathExtension => ".",
                        UriTemplateOperator::PathComponent => "/",
                        UriTemplateOperator::PathParameter => ";",
                        UriTemplateOperator::QueryParameter => "&",
                        UriTemplateOperator::QueryContinuation => "&",
                    }
                ).unwrap_or(",");

                let escaping = operator.map(|o|
                    match o {
                        UriTemplateOperator::ReservedCharacter => UriTemplateEscaping::UR,
                        UriTemplateOperator::Fragment => UriTemplateEscaping::UR,
                        UriTemplateOperator::PathExtension => UriTemplateEscaping::U,
                        UriTemplateOperator::PathComponent => UriTemplateEscaping::U,
                        UriTemplateOperator::PathParameter => UriTemplateEscaping::U,
                        UriTemplateOperator::QueryParameter => UriTemplateEscaping::U,
                        UriTemplateOperator::QueryContinuation => UriTemplateEscaping::U,
                    }
                ).unwrap_or(UriTemplateEscaping::U);

                let values: Vec<String> = variables.iter().map(|v| {
                    match v {
                        &UriTemplateVariable::Simple(ref name) => {
                            let strings: Vec<String> = values.strings_for_name(name);
                            let strings: Vec<String> = strings.into_iter().map(|s| escape_string(escaping, s.as_slice())).collect();
                            strings.connect(",")
                        }
                        &UriTemplateVariable::Prefix(ref name, prefix_len) => {
                            let strings: Vec<String> = values.strings_for_name(name).into_iter().map(|s| {
                                let strings: Vec<String> = s.as_slice().graphemes(true).take(prefix_len as uint).map(|s| s.to_string()).collect();
                                strings.concat()
                            }).collect();
                            let strings: Vec<String> = strings.into_iter().map(|s| escape_string(escaping, s.as_slice())).collect();
                            strings.connect(",")
                        }
                        &UriTemplateVariable::Explode(ref name) => {
                            let strings: Vec<String> = values.strings_for_name(name);
                            let strings: Vec<String> = strings.into_iter().map(|s| escape_string(escaping, s.as_slice())).collect();
                            strings.connect(separator)
                        }
                        &UriTemplateVariable::ExplodePrefix(ref name, prefix_len) => {
                            panic!("unimplemented");
                            "".to_string()
                        }
                    }
                }).collect();

                if values.len() == 0 {
                    "".to_string()
                } else {
                    format!("{}{}", prefix, values.connect(separator))
                }
            },
        }
    }
}


pub struct UriTemplate {
    components: Vec<UriTemplateComponent>,
}

impl UriTemplate {
    // pub fn from_str(template_: &'a str) -> Result<UriTemplate, Error> {
    //     let template = template_.to_owned();
    //     let slice = template.as_slice();
    //     let mut components = Vec::<UriTemplateComponent<'a>>::new();
    //
    //     // let chars = template.as_slice().chars();
    //     // loop {
    //     //     let component =
    //     // }
    //
    //     Ok(UriTemplate{
    //         template: template,
    //         components: components,
    //     })
    // }

    pub fn from_components(components: Vec<UriTemplateComponent>) -> UriTemplate {
        UriTemplate {
            components: components,
        }
    }

    pub fn to_template_string(&self) -> String {
        let components: Vec<String> = self.components.iter().map(|c|
            c.to_template_string()
        ).collect();
        components.concat()
    }

    pub fn into_template_string(self) -> String {
        let components: Vec<String> = self.components.into_iter().map(|c|
            c.into_template_string()
        ).collect();
        components.concat()
    }

    pub fn to_string_with_values(&self, values: &UriTemplateValues) -> String {
        let components: Vec<String> = self.components.iter().map(|c|
            c.to_string_with_values(values)
        ).collect();
        components.concat()
    }

    // pub fn to_url(&self) -> Option<url::Url> {
    //     let mut s = String::new();
    //     for component in self.components.iter() {
    //         match component {
    //             &UriTemplateComponent::Literal(literal) => s.push_str(literal),
    //             _ => (),
    //         }
    //     }
    // }
}

impl std::fmt::Show for UriTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        f.pad(self.to_template_string().as_slice())
    }
}


pub enum UriTemplateValue {
    String(String),
    List(Vec<String>),
}

impl std::str::FromStr for UriTemplateValue {
    fn from_str(s: &str) -> Option<Self> {
        Some(UriTemplateValue::String(s.to_string()))
    }
}

pub struct UriTemplateValues {
    values: HashMap<String, UriTemplateValue>,
}

impl UriTemplateValues {
    pub fn new() -> UriTemplateValues {
        UriTemplateValues {
            values: HashMap::new(),
        }
    }

    pub fn set(&mut self, name: &str, value: UriTemplateValue) -> &mut UriTemplateValues {
        self.values.insert(name.to_string(), value);
        self
    }

    pub fn set_string(&mut self, name: &str, value: &str) -> &mut UriTemplateValues {
        self.values.insert(name.to_string(), UriTemplateValue::String(value.to_string()));
        self
    }

    pub fn set_strings(&mut self, name: &str, value: Vec<&str>) -> &mut UriTemplateValues {
        self.values.insert(name.to_string(), UriTemplateValue::List(value.iter().map(|s| s.to_string()).collect()));
        self
    }

    fn strings_for_name(&self, name: &String) -> Vec<String> {
        self.values.get(name).map(|value| {
            match value {
                &UriTemplateValue::String(ref string) => vec!(string.clone()),
                &UriTemplateValue::List(ref strings) => strings.clone(),
            }
        }).unwrap_or(vec!())
    }
}


#[cfg(test)]
mod test_values {
    use super::{UriTemplateValues, UriTemplateValue};

    #[test]
    fn test_values_1() {
        let mut v = UriTemplateValues::new();
        v.set("foo", from_str("bar").unwrap());
        v.set_string("foo", "baz");
    }
}

#[cfg(test)]
mod test_expanding {
    use super::super::UriTemplateBuilder;
    use super::super::{UriTemplateOperator, UriTemplateModifier};
    use super::super::{UriTemplateValues, UriTemplateValue};

    #[test]
    fn test_level_1_1() {
        let t = UriTemplateBuilder::new()
        .literal("http://example.com/")
        .component(None, |c| c.variable("foo", None))
        .into_uri_template();

        let mut v = UriTemplateValues::new();

        let s = t.to_template_string();
        assert_eq!(s, "http://example.com/{foo}");

        let s = t.to_string_with_values(&v);
        assert_eq!(s, "http://example.com/");
    }

    #[test]
    fn test_level_1_2() {
        let t = UriTemplateBuilder::new()
        .literal("http://example.com/")
        .component(None, |c| c.variable("foo", None))
        .into_uri_template();

        let mut v = UriTemplateValues::new();
        v.set_string("foo", "bar");

        let s = t.to_template_string();
        assert_eq!(s, "http://example.com/{foo}");

        let s = t.to_string_with_values(&v);
        assert_eq!(s, "http://example.com/bar");
    }

    #[test]
    fn test_level_1_3() {
        let t = UriTemplateBuilder::new()
        .literal("http://example.com/")
        .component(None, |c| {
            c.variable("foo", None)
            .variable("bar", None)
        })
        .into_uri_template();

        let mut v = UriTemplateValues::new();
        v.set_string("foo", "bar");
        v.set_string("bar", "baz");

        let s = t.to_template_string();
        assert_eq!(s, "http://example.com/{foo,bar}");

        let s = t.to_string_with_values(&v);
        assert_eq!(s, "http://example.com/bar,baz");
    }
}
