extern crate uritemplates;

use uritemplates::UriTemplateBuilder;
use uritemplates::{UriTemplateOperator, UriTemplateModifier};
use uritemplates::{UriTemplateValues};


fn test_values() -> UriTemplateValues {
    let mut v = UriTemplateValues::new();
    v.set("count", ["one", "two", "three"].as_ref());
    v.set("dom", ["example", "com"].as_ref());
    v.set("dub", "me/too");
    v.set("hello", "Hello World!");
    v.set("half", "50%");
    v.set("var", "value");
    v.set("who", "fred");
    v.set("base", "http://example.com/home/");
    v.set("path", "/foo/bar");
    v.set("list", ["red", "green", "blue"].as_ref());
    // "keys"       : { "semi" : ";", "dot" : ".", "comma" : ","},
    v.set("v", "6");
    v.set("x", "1024");
    v.set("y", "768");
    v.set("empty", "");
    v.set("empty_keys", Vec::new());
    v
}


#[test]
fn test_3_2_1_variable_expansion_a() {
    let v = test_values();

    let t = UriTemplateBuilder::new()
        .component(None, |c| c.variable("count", None))
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{count}");
    assert_eq!(t.to_string_with_values(&v), "one,two,three");
}

#[test]
fn test_3_2_1_variable_expansion_b() {
    let v = test_values();

    let t = UriTemplateBuilder::new()
        .component(None, |c| c.variable("count", Some(UriTemplateModifier::Explode)))
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{count*}");
    assert_eq!(t.to_string_with_values(&v), "one,two,three");
}

#[test]
fn test_3_2_1_variable_expansion_c() {
    let v = test_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::PathComponent), |c| c.variable("count", None))
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{/count}");
    assert_eq!(t.to_string_with_values(&v), "/one,two,three");
}

#[test]
fn test_3_2_1_variable_expansion_d() {
    let v = test_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::PathComponent), |c| c.variable("count", Some(UriTemplateModifier::Explode)))
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{/count*}");
    assert_eq!(t.to_string_with_values(&v), "/one/two/three");
}

#[test]
fn test_3_2_1_variable_expansion_e() {
    let v = test_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::PathParameter), |c| c.variable("count", None))
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{;count}");
    assert_eq!(t.to_string_with_values(&v), ";count=one,two,three");
}

#[test]
fn test_3_2_1_variable_expansion_f() {
    let v = test_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::PathParameter), |c| c.variable("count", Some(UriTemplateModifier::Explode)))
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{;count*}");
    assert_eq!(t.to_string_with_values(&v), ";count=one;count=two;count=three");
}

#[test]
fn test_3_2_1_variable_expansion_g() {
    let v = test_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::QueryParameter), |c| c.variable("count", None))
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{?count}");
    assert_eq!(t.to_string_with_values(&v), "?count=one,two,three");
}

#[test]
fn test_3_2_1_variable_expansion_h() {
    let v = test_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::QueryParameter), |c| c.variable("count", Some(UriTemplateModifier::Explode)))
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{?count*}");
    assert_eq!(t.to_string_with_values(&v), "?count=one&count=two&count=three");
}

#[test]
fn test_3_2_1_variable_expansion_i() {
    let v = test_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::QueryContinuation), |c| c.variable("count", Some(UriTemplateModifier::Explode)))
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{&count*}");
    assert_eq!(t.to_string_with_values(&v), "&count=one&count=two&count=three");
}


#[test]
fn test_3_2_2_simple_string_expansion_a() {
    let v = test_values();

    let t = UriTemplateBuilder::new()
        .component(None, |c| c.variable("var", None))
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{var}");
    assert_eq!(t.to_string_with_values(&v), "value");
}

#[test]
fn test_3_2_2_simple_string_expansion_b() {
    let v = test_values();

    let t = UriTemplateBuilder::new()
        .component(None, |c| c.variable("hello", None))
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{hello}");
    assert_eq!(t.to_string_with_values(&v), "Hello%20World%21");
}

#[test]
fn test_3_2_2_simple_string_expansion_c() {
    let v = test_values();

    let t = UriTemplateBuilder::new()
        .component(None, |c| c.variable("half", None))
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{half}");
    assert_eq!(t.to_string_with_values(&v), "50%25");
}

#[test]
fn test_3_2_2_simple_string_expansion_d() {
    let v = test_values();

    let t = UriTemplateBuilder::new()
        .literal("O")
        .component(None, |c| c.variable("empty", None))
        .literal("X")
        .into_uri_template();
    assert_eq!(t.to_template_string(), "O{empty}X");
    assert_eq!(t.to_string_with_values(&v), "OX");
}

#[test]
fn test_3_2_2_simple_string_expansion_e() {
    let v = test_values();

    let t = UriTemplateBuilder::new()
        .literal("O")
        .component(None, |c| c.variable("undef", None))
        .literal("X")
        .into_uri_template();
    assert_eq!(t.to_template_string(), "O{undef}X");
    assert_eq!(t.to_string_with_values(&v), "OX");
}

#[test]
fn test_3_2_2_simple_string_expansion_f() {
    let v = test_values();

    let t = UriTemplateBuilder::new()
        .component(None, |c|
            c.variable("x", None)
             .variable("y", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{x,y}");
    assert_eq!(t.to_string_with_values(&v), "1024,768");
}

#[test]
fn test_3_2_2_simple_string_expansion_g() {
    let v = test_values();

    let t = UriTemplateBuilder::new()
        .component(None, |c|
            c.variable("x", None)
             .variable("hello", None)
             .variable("y", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{x,hello,y}");
    assert_eq!(t.to_string_with_values(&v), "1024,Hello%20World%21,768");
}

#[test]
fn test_3_2_2_simple_string_expansion_h() {
    let v = test_values();

    let t = UriTemplateBuilder::new()
        .literal("?")
        .component(None, |c|
            c.variable("x", None)
             .variable("empty", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "?{x,empty}");
    assert_eq!(t.to_string_with_values(&v), "?1024,");
}

#[test]
fn test_3_2_2_simple_string_expansion_i() {
    let v = test_values();

    let t = UriTemplateBuilder::new()
        .literal("?")
        .component(None, |c|
            c.variable("x", None)
             .variable("undef", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "?{x,undef}");
    assert_eq!(t.to_string_with_values(&v), "?1024");
}

#[test]
fn test_3_2_2_simple_string_expansion_j() {
    let v = test_values();

    let t = UriTemplateBuilder::new()
        .literal("?")
        .component(None, |c|
            c.variable("undef", None)
             .variable("y", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "?{undef,y}");
    assert_eq!(t.to_string_with_values(&v), "?768");
}

#[test]
fn test_3_2_2_simple_string_expansion_k() {
    let v = test_values();

    let t = UriTemplateBuilder::new()
        .component(None, |c|
            c.variable("var", Some(UriTemplateModifier::Prefix(3)))
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{var:3}");
    assert_eq!(t.to_string_with_values(&v), "val");
}

#[test]
fn test_3_2_2_simple_string_expansion_l() {
    let v = test_values();

    let t = UriTemplateBuilder::new()
        .component(None, |c|
            c.variable("var", Some(UriTemplateModifier::Prefix(30)))
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{var:30}");
    assert_eq!(t.to_string_with_values(&v), "value");
}

#[test]
fn test_3_2_2_simple_string_expansion_m() {
    let v = test_values();

    let t = UriTemplateBuilder::new()
        .component(None, |c|
            c.variable("list", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{list}");
    assert_eq!(t.to_string_with_values(&v), "red,green,blue");
}

#[test]
fn test_3_2_2_simple_string_expansion_n() {
    let v = test_values();

    let t = UriTemplateBuilder::new()
        .component(None, |c|
            c.variable("list", Some(UriTemplateModifier::Explode))
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{list*}");
    assert_eq!(t.to_string_with_values(&v), "red,green,blue");
}

#[test]
fn test_3_2_2_simple_string_expansion_o() {
    // let v = test_values();

    let t = UriTemplateBuilder::new()
        .component(None, |c|
            c.variable("keys", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{keys}");
    // assert_eq!(t.to_string_with_values(&v), "comma,%2C,dot,.,semi,%3B");
}

#[test]
fn test_3_2_2_simple_string_expansion_p() {
    // let v = test_values();

    let t = UriTemplateBuilder::new()
        .component(None, |c|
            c.variable("keys", Some(UriTemplateModifier::Explode))
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{keys*}");
    // assert_eq!(t.to_string_with_values(&v), "comma=%2C,dot=.,semi=%3B");
}
