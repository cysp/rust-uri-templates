extern crate uritemplates;

use uritemplates::UriTemplateBuilder;
use uritemplates::{UriTemplateOperator, UriTemplateModifier};
use uritemplates::{UriTemplateValues, UriTemplateValue};


#[test]
fn test_level_1() {
    let mut v = UriTemplateValues::new();
    v.set_string("var", "value");
    v.set_string("hello", "Hello World!");

    let t = UriTemplateBuilder::new()
        .component(None, |c| c.variable("var", None))
        .into_uri_template();
    assert_eq!(t.to_string_with_values(&v), "value");

    let t = UriTemplateBuilder::new()
        .component(None, |c| c.variable("hello", None))
        .into_uri_template();
    assert_eq!(t.to_string_with_values(&v), "Hello%20World%21");
}

#[test]
fn test_level_2() {
    let mut v = UriTemplateValues::new();
    v.set_string("var", "value");
    v.set_string("hello", "Hello World!");
    v.set_string("path", "/foo/bar");

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::ReservedCharacter), |c| c.variable("var", None))
        .into_uri_template();
    assert_eq!(t.to_string_with_values(&v), "value");

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::ReservedCharacter), |c| c.variable("hello", None))
        .into_uri_template();
    assert_eq!(t.to_string_with_values(&v), "Hello%20World!");

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::ReservedCharacter), |c| c.variable("path", None))
        .literal("/here")
        .into_uri_template();
    assert_eq!(t.to_string_with_values(&v), "/foo/bar/here");

    let t = UriTemplateBuilder::new()
        .literal("here?ref=")
        .component(Some(UriTemplateOperator::ReservedCharacter), |c| c.variable("path", None))
        .into_uri_template();
    assert_eq!(t.to_string_with_values(&v), "here?ref=/foo/bar");
}

#[test]
fn test_level_3() {
    let mut v = UriTemplateValues::new();
    v.set_string("var", "value");
    v.set_string("hello", "Hello World!");
    v.set_string("empty", "");
    v.set_string("path", "/foo/bar");
    v.set_string("x", "1024");
    v.set_string("y", "768");

    let t = UriTemplateBuilder::new()
        .literal("map?")
        .component(None, |c|
            c.variable("x", None)
             .variable("y", None)
        )
        .into_uri_template();
    assert_eq!(t.to_string_with_values(&v), "map?1024,768");

    let t = UriTemplateBuilder::new()
        .component(None, |c|
            c.variable("x", None)
             .variable("hello", None)
             .variable("y", None)
        )
        .into_uri_template();
    assert_eq!(t.to_string_with_values(&v), "1024,Hello%20World%21,768");

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::ReservedCharacter), |c|
            c.variable("x", None)
             .variable("hello", None)
             .variable("y", None)
        )
        .into_uri_template();
    assert_eq!(t.to_string_with_values(&v), "1024,Hello%20World!,768");

    let t = UriTemplateBuilder::new()
    .component(Some(UriTemplateOperator::ReservedCharacter), |c|
        c.variable("path", None)
         .variable("x", None)
    )
    .literal("/here")
    .into_uri_template();
    assert_eq!(t.to_string_with_values(&v), "/foo/bar,1024/here");

    let t = UriTemplateBuilder::new()
    .component(Some(UriTemplateOperator::Fragment), |c|
    c.variable("x", None)
    .variable("hello", None)
    .variable("y", None)
    )
    .into_uri_template();
    assert_eq!(t.to_string_with_values(&v), "#1024,Hello%20World!,768");

    let t = UriTemplateBuilder::new()
    .component(Some(UriTemplateOperator::Fragment), |c|
    c.variable("path", None)
    .variable("x", None)
    )
    .literal("/here")
    .into_uri_template();
    assert_eq!(t.to_string_with_values(&v), "#/foo/bar,1024/here");

    let t = UriTemplateBuilder::new()
    .literal("X")
    .component(Some(UriTemplateOperator::PathExtension), |c|
    c.variable("var", None)
    )
    .into_uri_template();
    assert_eq!(t.to_string_with_values(&v), "X.value");

    let t = UriTemplateBuilder::new()
    .literal("X")
    .component(Some(UriTemplateOperator::PathExtension), |c|
    c.variable("x", None)
    .variable("y", None)
    )
    .into_uri_template();
    assert_eq!(t.to_string_with_values(&v), "X.1024.768");

    let t = UriTemplateBuilder::new()
    .component(Some(UriTemplateOperator::PathComponent), |c|
    c.variable("var", None)
    )
    .into_uri_template();
    assert_eq!(t.to_string_with_values(&v), "/value");

    let t = UriTemplateBuilder::new()
    .component(Some(UriTemplateOperator::PathComponent), |c|
    c.variable("var", None)
    .variable("x", None)
    )
    .literal("/here")
    .into_uri_template();
    assert_eq!(t.to_string_with_values(&v), "/value/1024/here");

    let t = UriTemplateBuilder::new()
    .component(Some(UriTemplateOperator::PathParameter), |c|
    c.variable("x", None)
    .variable("y", None)
    )
    .into_uri_template();
    assert_eq!(t.to_string_with_values(&v), ";x=1024;y=768");

    let t = UriTemplateBuilder::new()
    .component(Some(UriTemplateOperator::PathParameter), |c|
    c.variable("x", None)
    .variable("y", None)
    .variable("empty", None)
    )
    .into_uri_template();
    assert_eq!(t.to_string_with_values(&v), ";x=1024;y=768;empty");

    let t = UriTemplateBuilder::new()
    .component(Some(UriTemplateOperator::QueryParameter), |c|
    c.variable("x", None)
    .variable("y", None)
    )
    .into_uri_template();
    assert_eq!(t.to_string_with_values(&v), "?x=1024&y=768");

    let t = UriTemplateBuilder::new()
    .component(Some(UriTemplateOperator::QueryParameter), |c|
    c.variable("x", None)
    .variable("y", None)
    .variable("empty", None)
    )
    .into_uri_template();
    assert_eq!(t.to_string_with_values(&v), "?x=1024&y=768&empty=");

    let t = UriTemplateBuilder::new()
    .literal("?fixed=yes")
    .component(Some(UriTemplateOperator::QueryContinuation), |c|
    c.variable("x", None)
    )
    .into_uri_template();
    assert_eq!(t.to_string_with_values(&v), "?fixed=yes&x=1024");

    let t = UriTemplateBuilder::new()
    .component(Some(UriTemplateOperator::QueryContinuation), |c|
    c.variable("x", None)
    .variable("y", None)
    .variable("empty", None)
    )
    .into_uri_template();
    assert_eq!(t.to_string_with_values(&v), "?x=1024&y=768&empty=");
}

    // ["{&x,y,empty}", "&x=1024&y=768&empty="]
