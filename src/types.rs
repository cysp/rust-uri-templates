extern crate std;

use std::vec::Vec;


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
}


pub enum UriTemplateComponent {
    Literal(String),
    Variable(Option<UriTemplateOperator>, Vec<UriTemplateVariable>),
}

impl UriTemplateComponent {
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
}


pub struct UriTemplate {
    components: Vec<UriTemplateComponent>,
}

impl UriTemplate {
    pub fn from_components(components: Vec<UriTemplateComponent>) -> UriTemplate {
        UriTemplate {
            components: components,
        }
    }

    pub fn into_template_string(self) -> String {
        let components: Vec<String> = self.components.into_iter().map(|c|
            c.into_template_string()
        ).collect();
        components.concat()
    }
}
