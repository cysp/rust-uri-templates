extern crate std;
// extern crate url;

use std::vec::Vec;
use std::collections::HashMap;


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


#[deriving(Copy)]
pub enum UriTemplateModifier {
    Prefix(u32),
    Explode,
}


pub struct UriTemplateVariable {
    name: String,
    modifier: Option<UriTemplateModifier>,
}

impl UriTemplateVariable {
    pub fn new(name: &str, modifier: Option<UriTemplateModifier>) -> UriTemplateVariable {
        UriTemplateVariable {
            name: name.to_string(),
            modifier: modifier,
        }
    }

    pub fn into_template_string(self) -> String {
        match self {
            UriTemplateVariable{ name, modifier: None } => name,
            UriTemplateVariable{ name, modifier: Some(UriTemplateModifier::Prefix(prefix)) } => {
                format!("{}:{}", name, prefix)
            }
            UriTemplateVariable{ name, modifier: Some(UriTemplateModifier::Explode) } => {
                format!("{}*", name)
            }
        }
    }

    pub fn to_template_string(&self) -> String {
        match self {
            &UriTemplateVariable{ ref name, modifier: None } => name.clone(),
            &UriTemplateVariable{ ref name, modifier: Some(UriTemplateModifier::Prefix(prefix)) } => {
                format!("{}:{}", name, prefix)
            }
            &UriTemplateVariable{ ref name, modifier: Some(UriTemplateModifier::Explode) } => {
                format!("{}*", name)
            }
        }
    }
}


pub enum UriTemplateComponent {
    Literal(String),
    Variable(Option<UriTemplateOperator>, Vec<UriTemplateVariable>),
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
            &UriTemplateComponent::Literal(ref value) => value.clone(),
            &UriTemplateComponent::Variable(operator, ref variables) => {
                let values: Vec<String> = variables.iter().flat_map(|v| {
                    values.strings_for_name(&v.name).into_iter()
                }).collect();

                let prefix: &'static str = operator.map(|o|
                    match o {
                        UriTemplateOperator::ReservedCharacter => "",
                        UriTemplateOperator::Fragment => "#",
                        UriTemplateOperator::PathExtension => ".",
                        UriTemplateOperator::PathComponent => "/",
                        UriTemplateOperator::PathParameter => panic!("unimplemented"),
                        UriTemplateOperator::QueryParameter => panic!("unimplemented"),
                        UriTemplateOperator::QueryContinuation => panic!("unimplemented"),
                    }
                ).unwrap_or("");
                let separator: &'static str = operator.map(|o|
                    match o {
                        UriTemplateOperator::ReservedCharacter => "",
                        UriTemplateOperator::Fragment => "",
                        UriTemplateOperator::PathExtension => ".",
                        UriTemplateOperator::PathComponent => "/",
                        UriTemplateOperator::PathParameter => panic!("unimplemented"),
                        UriTemplateOperator::QueryParameter => panic!("unimplemented"),
                        UriTemplateOperator::QueryContinuation => panic!("unimplemented"),
                    }
                ).unwrap_or(",");
                format!("{}{}", prefix, values.connect(separator))
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


pub enum UriTemplateValue {
    String(String),
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

    fn strings_for_name(&self, name: &String) -> Vec<String> {
        self.values.get(name).map(|value| {
            match value {
                &UriTemplateValue::String(ref string) => vec!(string.clone()),
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
        v.set_string("foo", "bar");

        let s = t.to_template_string();
        assert_eq!(s, "http://example.com/{foo}");

        let s = t.to_string_with_values(&v);
        assert_eq!(s, "http://example.com/bar");
    }

    #[test]
    fn test_level_1_2() {
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
