extern crate uritemplates;

use uritemplates::UriTemplateBuilder;
use uritemplates::{UriTemplateOperator, UriTemplateModifier};
use uritemplates::{UriTemplateValues, UriTemplateValue};


#[test]
fn test_3_2_1_variable_expansion() {
    let mut v = UriTemplateValues::new();
    v.set_strings("count", vec!["one", "two", "three"]);
    v.set_strings("dom", vec!["example", "com"]);

    let t = UriTemplateBuilder::new()
        .component(None, |c| c.variable("count", None))
        .into_uri_template();
    assert_eq!(t.to_string_with_values(&v), "one,two,three");

    let t = UriTemplateBuilder::new()
        .component(None, |c| c.variable("count", Some(UriTemplateModifier::Explode)))
        .into_uri_template();
    assert_eq!(t.to_string_with_values(&v), "one,two,three");

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::PathComponent), |c| c.variable("count", None))
        .into_uri_template();
    assert_eq!(t.to_string_with_values(&v), "/one,two,three");

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::PathComponent), |c| c.variable("count", Some(UriTemplateModifier::Explode)))
        .into_uri_template();
    assert_eq!(t.to_string_with_values(&v), "/one/two/three");

    // let t = UriTemplateBuilder::new()
    //     .component(Some(UriTemplateOperator::PathParameter), |c| c.variable("count", None))
    //     .into_uri_template();
    // assert_eq!(t.to_string_with_values(&v), ";count=one,two,three");
    //
    // let t = UriTemplateBuilder::new()
    //     .component(Some(UriTemplateOperator::PathParameter), |c| c.variable("count", Some(UriTemplateModifier::Explode)))
    //     .into_uri_template();
    // assert_eq!(t.to_string_with_values(&v), ";count=one;count=two;count=three");
    //
    // let t = UriTemplateBuilder::new()
    //     .component(Some(UriTemplateOperator::QueryParameter), |c| c.variable("count", None))
    //     .into_uri_template();
    // assert_eq!(t.to_string_with_values(&v), "?count=one,two,three");
    //
    // let t = UriTemplateBuilder::new()
    //     .component(Some(UriTemplateOperator::QueryParameter), |c| c.variable("count", Some(UriTemplateModifier::Explode)))
    //     .into_uri_template();
    // assert_eq!(t.to_string_with_values(&v), "?count=one&count=two&count=three");
    //
    // let t = UriTemplateBuilder::new()
    //     .component(Some(UriTemplateOperator::QueryContinuation), |c| c.variable("count", Some(UriTemplateModifier::Explode)))
    //     .into_uri_template();
    // assert_eq!(t.to_string_with_values(&v), "&count=one&count=two&count=three");
}

// "3.2.2 Simple String Expansion" :
// {
//     "variables": {
//         "count"      : ["one", "two", "three"],
//         "dom"        : ["example", "com"],
//         "dub"        : "me/too",
//         "hello"      : "Hello World!",
//         "half"       : "50%",
//         "var"        : "value",
//         "who"        : "fred",
//         "base"       : "http://example.com/home/",
//         "path"       : "/foo/bar",
//         "list"       : ["red", "green", "blue"],
//         "keys"       : { "semi" : ";", "dot" : ".", "comma" : ","},
//         "v"          : "6",
//         "x"          : "1024",
//         "y"          : "768",
//         "empty"      : "",
//         "empty_keys" : [],
//         "undef"      : null
//         },
//         "testcases" : [
//         ["{var}", "value"],
//         ["{hello}", "Hello%20World%21"],
//         ["{half}", "50%25"],
//         ["O{empty}X", "OX"],
//         ["O{undef}X", "OX"],
//         ["{x,y}", "1024,768"],
//         ["{x,hello,y}", "1024,Hello%20World%21,768"],
//         ["?{x,empty}", "?1024,"],
//         ["?{x,undef}", "?1024"],
//         ["?{undef,y}", "?768"],
//         ["{var:3}", "val"],
//         ["{var:30}", "value"],
//         ["{list}", "red,green,blue"],
//         ["{list*}", "red,green,blue"],
//         ["{keys}", [
//         "comma,%2C,dot,.,semi,%3B",
//         "comma,%2C,semi,%3B,dot,.",
//         "dot,.,comma,%2C,semi,%3B",
//         "dot,.,semi,%3B,comma,%2C",
//         "semi,%3B,comma,%2C,dot,.",
//         "semi,%3B,dot,.,comma,%2C"
//         ]],
//         ["{keys*}", [
//         "comma=%2C,dot=.,semi=%3B",
//         "comma=%2C,semi=%3B,dot=.",
//         "dot=.,comma=%2C,semi=%3B",
//         "dot=.,semi=%3B,comma=%2C",
//         "semi=%3B,comma=%2C,dot=.",
//         "semi=%3B,dot=.,comma=%2C"
//         ]]
//         ]
//         },
