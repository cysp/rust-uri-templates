use std::vec::Vec;
use super::types::{UriTemplate, UriTemplateComponent, UriTemplateOperator, UriTemplateVariable};


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

    pub fn into_uri_template(self) -> UriTemplate {
        UriTemplate::from_components(self.components)
    }

    pub fn into_template_string(self) -> String {
        self.into_uri_template().into_template_string()
    }
}


#[derive(Copy,Clone,PartialEq,Eq)]
pub enum UriTemplateModifier {
    Prefix(u32),
    Explode,
    ExplodePrefix(u32),
}

pub struct UriTemplateComponentBuilder {
    variables: Vec<UriTemplateVariable>,
}

impl UriTemplateComponentBuilder {
    pub fn variable(mut self, name: &str, modifier: Option<UriTemplateModifier>) -> UriTemplateComponentBuilder {
        self.variables.push(match modifier {
            None => UriTemplateVariable::new_simple(name.to_string()),
            Some(UriTemplateModifier::Prefix(prefix)) => UriTemplateVariable::new_prefix(name.to_string(), prefix),
            Some(UriTemplateModifier::Explode) => UriTemplateVariable::new_explode(name.to_string()),
            Some(UriTemplateModifier::ExplodePrefix(prefix)) => UriTemplateVariable::new_explode_prefix(name.to_string(), prefix)
        });
        self
    }
}


#[cfg(test)]
mod test {
    use super::super::UriTemplateBuilder;
    use super::super::{UriTemplateOperator, UriTemplateModifier};

    #[test]
    fn test_1() {
        let t = UriTemplateBuilder::new()
            .into_uri_template()
            .into_template_string();

        assert_eq!(t, "");
    }

    #[test]
    fn test_2() {
        let t = UriTemplateBuilder::new()
            .literal("http://example.com/")
            .into_uri_template()
            .into_template_string();

        assert_eq!(t, "http://example.com/");
    }

    #[test]
    fn test_3() {
        let t = UriTemplateBuilder::new()
            .literal("http://example.com/")
            .literal("foo")
            .into_uri_template()
            .into_template_string();

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
            .into_uri_template();
        let s1 = t.to_template_string();
        let s2 = t.into_template_string();

        assert_eq!(s1, "http://example.com/{/splat*}{?foo,bar,hash:7}");
        assert_eq!(s1, s2);
    }
}
