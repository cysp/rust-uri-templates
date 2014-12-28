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
    fn into_string(self) -> String {
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

enum UriTemplateComponent {
    Literal(String),
    Variable(Option<UriTemplateOperator>, Vec<UriTemplateVariable>),
}

impl UriTemplateComponent {
    fn into_string(self) -> String {
        match self {
            UriTemplateComponent::Literal(value) => value,
            UriTemplateComponent::Variable(operator, variables) => {
                let varspecs: Vec<String> = variables.into_iter().map(|v|
                    v.into_string()
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

pub struct UriTemplateBuilder {
    components: Vec<UriTemplateComponent>,
}

impl UriTemplateBuilder {
    pub fn new() -> UriTemplateBuilder {
        UriTemplateBuilder {
            components: vec!(),
        }
    }

    pub fn literal(mut self, value: &str) -> UriTemplateBuilder {
        self.components.push(UriTemplateComponent::Literal(value.to_string()));
        self
    }

    pub fn component<F: Fn(UriTemplateComponentBuilder) -> UriTemplateComponentBuilder>(mut self, operator: Option<UriTemplateOperator>, f: F) -> UriTemplateBuilder {
        let c = UriTemplateComponentBuilder {
            variables: vec!(),
        };
        let c = f(c);
        self.components.push(UriTemplateComponent::Variable(operator, c.variables));
        self
    }

    pub fn into_string(self) -> String {
        let components: Vec<String> = self.components.into_iter().map(|c|
            c.into_string()
        ).collect();
        components.concat()
    }
}


pub struct UriTemplateComponentBuilder {
    variables: Vec<UriTemplateVariable>,
}

impl UriTemplateComponentBuilder {
    pub fn variable(mut self, name: &str, modifier: Option<UriTemplateModifier>) -> UriTemplateComponentBuilder {
        self.variables.push(UriTemplateVariable {
            name: name.to_string(),
            modifier: modifier,
        });
        self
    }
}


#[cfg(test)]
mod test {
    use super::UriTemplateBuilder;
    use super::UriTemplateOperator;
    use super::UriTemplateModifier;

    #[test]
    fn test_1() {
        let t = UriTemplateBuilder::new()
            .into_string();

        assert_eq!(t, "");
    }

    #[test]
    fn test_2() {
        let t = UriTemplateBuilder::new()
            .literal("http://example.com/")
            .into_string();

        assert_eq!(t, "http://example.com/");
    }

    #[test]
    fn test_3() {
        let t = UriTemplateBuilder::new()
            .literal("http://example.com/")
            .literal("foo")
            .into_string();

        assert_eq!(t, "http://example.com/foo");
    }

    #[test]
    fn test_4() {
        let t = UriTemplateBuilder::new()
            .literal("http://example.com/")
            .component(Some(UriTemplateOperator::PathComponent), |c| {
                c.variable("splat", Some(UriTemplateModifier::Explode))
            })
            .component(Some(UriTemplateOperator::QueryParameter), |c| {
                c.variable("foo", None)
                 .variable("bar", None)
                 .variable("hash", Some(UriTemplateModifier::Prefix(7)))
            })
            .into_string();

        assert_eq!(t, "http://example.com/{/splat*}{?foo,bar,hash:7}");
    }
}
