extern crate std;

use std::vec::Vec;
use std::collections::{
    HashMap,
    HashSet,
};

use building;


#[derive(Copy,Clone,PartialEq,Eq)]
pub enum UriTemplateOperator {
    ReservedCharacter,
    Fragment,
    PathExtension,
    PathComponent,
    PathParameter,
    QueryParameter,
    QueryContinuation,
}


#[derive(Clone,PartialEq,Eq)]
pub struct UriTemplateVariable {
    name: String,
    explode: bool,
    prefix: Option<u32>,
}

#[derive(Copy,Clone,PartialEq,Eq)]
pub enum UriTemplateModifier {
    Prefix(u32),
    Explode,
    ExplodePrefix(u32),
}


impl UriTemplateVariable {
    pub fn new_simple(name: String) -> UriTemplateVariable {
        UriTemplateVariable{ name: name, prefix: None, explode: false }
    }

    pub fn new_prefix(name: String, prefix: u32) -> UriTemplateVariable {
        UriTemplateVariable{ name: name, prefix: Some(prefix), explode: false }
    }

    pub fn new_explode(name: String) -> UriTemplateVariable {
        UriTemplateVariable{ name: name, prefix: None, explode: true }
    }

    pub fn new_explode_prefix(name: String, prefix: u32) -> UriTemplateVariable {
        UriTemplateVariable{ name: name, prefix: Some(prefix), explode: true }
    }

    pub fn into_template_string(self) -> String {
        match self {
            UriTemplateVariable{ name, prefix: None, explode: false } => name,
            UriTemplateVariable{ name, prefix: Some(prefix), explode: false } => format!("{}:{}", name, prefix),
            UriTemplateVariable{ name, prefix: None, explode: true } => format!("{}*", name),
            UriTemplateVariable{ name, prefix: Some(prefix), explode: true } => format!("{}*:{}", name, prefix),
        }
    }

    pub fn to_template_string(&self) -> String {
        match self {
            &UriTemplateVariable{ ref name, prefix: None, explode: false } => name.clone(),
            &UriTemplateVariable{ ref name, prefix: Some(prefix), explode: false } => format!("{}:{}", name, prefix),
            &UriTemplateVariable{ ref name, prefix: None, explode: true } => format!("{}*", name),
            &UriTemplateVariable{ ref name, prefix: Some(prefix), explode: true } => format!("{}*:{}", name, prefix),
        }
    }
}


#[derive(Clone,PartialEq,Eq)]
pub enum UriTemplateComponent {
    Literal(String),
    Variable(Option<UriTemplateOperator>, Vec<UriTemplateVariable>),
}

#[derive(Copy,Clone,PartialEq,Eq)]
enum UriTemplateEscaping {
    U,
    UR,
}


fn escape_string(method: UriTemplateEscaping, input: &str) -> String {
    let str_u = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-._~";
    let str_ur = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-._~:/?#[]@!$&'()*+,;=";

    let mut set_u: HashSet<u8> = HashSet::new();
    for b in str_u.as_bytes().iter() {
        set_u.insert(*b);
    }
    let set_u = set_u;

    let mut set_ur: HashSet<u8> = HashSet::new();
    for b in str_ur.as_bytes().iter() {
        set_ur.insert(*b);
    }
    let set_ur = set_ur;

    let mut s = String::new();
    for byte in input.as_bytes().iter() {
        match method {
            UriTemplateEscaping::U => {
                if set_u.contains(&(*byte)) {
                    s.push(*byte as char);
                } else {
                    s.push_str(&(format!("%{:02X}", *byte)))
                }
            }
            UriTemplateEscaping::UR => {
                if set_ur.contains(&(*byte)) {
                    s.push(*byte as char);
                } else {
                    s.push_str(&(format!("%{:02X}", *byte)))
                }
            }
        }
    }
    s
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

    fn strings_apply_prefix(strings: Vec<String>, prefix_len: u32) -> Vec<String> {
        if strings.len() == 0 {
            return strings;
        }
        strings.into_iter().map(|s| {
            s.chars().take(prefix_len as usize).collect()
        }).collect()
    }

    fn strings_apply_escaping(strings: Vec<String>, escaping: UriTemplateEscaping) -> Vec<String> {
        if strings.len() == 0 {
            return strings;
        }
        strings.into_iter().map(|s|
            escape_string(escaping, s.as_ref())
        ).collect()
    }

    pub fn to_string_with_values(&self, values: &UriTemplateValues) -> String {
        match self {
            &UriTemplateComponent::Literal(ref value) => escape_string(UriTemplateEscaping::UR, value.as_ref()),
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

                let include_name: bool = operator.map(|o|
                    match o {
                        UriTemplateOperator::ReservedCharacter => false,
                        UriTemplateOperator::Fragment => false,
                        UriTemplateOperator::PathExtension => false,
                        UriTemplateOperator::PathComponent => false,
                        UriTemplateOperator::PathParameter => true,
                        UriTemplateOperator::QueryParameter => true,
                        UriTemplateOperator::QueryContinuation => true,
                    }
                ).unwrap_or(false);

                let include_equals_when_empty: bool = operator.map(|o|
                    match o {
                        UriTemplateOperator::ReservedCharacter => false,
                        UriTemplateOperator::Fragment => false,
                        UriTemplateOperator::PathExtension => false,
                        UriTemplateOperator::PathComponent => false,
                        UriTemplateOperator::PathParameter => false,
                        UriTemplateOperator::QueryParameter => true,
                        UriTemplateOperator::QueryContinuation => true,
                    }
                ).unwrap_or(false);

                let values: Vec<String> = variables.iter().filter_map(|v| {
                    let values: Vec<String> = match v {
                        &UriTemplateVariable{ ref name, prefix, explode: _ } => {
                            let mut strings: Vec<String> = values.strings_for_name(name);
                            if let Some(prefix) = prefix {
                                strings = UriTemplateComponent::strings_apply_prefix(strings, prefix);
                            }
                            UriTemplateComponent::strings_apply_escaping(strings, escaping)
                        }
                    };
                    if values.len() == 0 {
                        return None;
                    }
                    Some(match v {
                        &UriTemplateVariable{ ref name, prefix: _, explode: false } => {
                            let mut value = values.connect(",");
                            if include_name {
                                if value.len() != 0 || include_equals_when_empty {
                                    value = format!("{}={}", name, value);
                                } else {
                                    value = name.clone();
                                }
                            }
                            value
                        },
                        &UriTemplateVariable{ ref name, prefix: _, explode: true } => {
                            if include_name {
                                let strings: Vec<String> = values.into_iter().map(|s| { format!("{}={}", name, s) }).collect();
                                strings.connect(separator)
                            } else {
                                values.connect(separator)
                            }
                        },
                    })
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


#[derive(Clone,PartialEq,Eq)]
pub struct UriTemplate {
    components: Vec<UriTemplateComponent>,
}

impl UriTemplate {
    pub fn builder() -> building::UriTemplateBuilder {
        building::UriTemplateBuilder::new()
    }

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
}

impl std::fmt::Debug for UriTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        f.pad(self.to_template_string().as_ref())
    }
}


pub enum UriTemplateValue {
    String(String),
    List(Vec<String>),
}

impl std::str::FromStr for UriTemplateValue {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        Ok(UriTemplateValue::String(s.to_string()))
    }
}

impl<'a> From<&'a str> for UriTemplateValue {
    fn from(s: &'a str) -> UriTemplateValue {
        UriTemplateValue::String(s.to_owned())
    }
}

impl From<String> for UriTemplateValue {
    fn from(s: String) -> UriTemplateValue {
        UriTemplateValue::String(s)
    }
}

impl<'a, 'b> From<&'a [&'b str]> for UriTemplateValue {
    fn from(l: &'a [&'b str]) -> UriTemplateValue {
        let v: Vec<String> = l.iter().map(|s: &&str| (*s).to_owned()).collect();
        UriTemplateValue::List(v)
    }
}

impl<'a> From<Vec<&'a str>> for UriTemplateValue {
    fn from(v: Vec<&'a str>) -> UriTemplateValue {
        let v: Vec<String> = v.iter().map(|s: &&str| (*s).to_owned()).collect();
        UriTemplateValue::List(v)
    }
}

impl From<Vec<String>> for UriTemplateValue {
    fn from(v: Vec<String>) -> UriTemplateValue {
        UriTemplateValue::List(v)
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

    pub fn set<T: Into<UriTemplateValue>>(&mut self, name: &str, value: T) -> &mut UriTemplateValues {
        self.values.insert(name.to_string(), value.into());
        self
    }

    fn strings_for_name(&self, name: &String) -> Vec<String> {
        self.values.get(name).map(|value| {
            match value {
                &UriTemplateValue::String(ref string) => vec!(string.clone()),
                &UriTemplateValue::List(ref strings) => strings.clone(),
            }
        }).unwrap_or(Vec::new())
    }
}


#[cfg(test)]
mod test_values {
    use super::{UriTemplateValues};

    #[test]
    fn test_values_1() {
        let mut v = UriTemplateValues::new();
        v.set("foo", "bar");
        v.set("foo", ["bar", "baz"].as_ref());
        v.set("foo", "baz");
    }

    #[test]
    fn test_values_2() {
        let mut v = UriTemplateValues::new();
        v.set("foo", "baz");
    }

    #[test]
    fn test_values_3() {
        let mut v = UriTemplateValues::new();
        v.set("foo", "baz");
    }
}

#[cfg(test)]
mod test_expanding {
    use super::super::{UriTemplateBuilder, UriTemplateValues};

    #[test]
    fn test_level_1_1() {
        let t = UriTemplateBuilder::new()
        .literal("http://example.com/")
        .component(None, |c| c.variable("foo", None))
        .into_uri_template();

        let v = UriTemplateValues::new();

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
        v.set("foo", "bar");

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
        v.set("foo", "bar");
        v.set("bar", "baz");

        let s = t.to_template_string();
        assert_eq!(s, "http://example.com/{foo,bar}");

        let s = t.to_string_with_values(&v);
        assert_eq!(s, "http://example.com/bar,baz");
    }
}
