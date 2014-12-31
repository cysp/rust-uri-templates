extern crate uritemplates;

use uritemplates::UriTemplateBuilder;
use uritemplates::{UriTemplateOperator, UriTemplateModifier};
use uritemplates::{UriTemplateValues, UriTemplateValue};


#[test]
fn test_level_1() {
    let mut v = UriTemplateValues::new();
    v.set_string("var", "value");
    v.set_string("hello", "Hello World!");
    let v = v;

    let t = UriTemplateBuilder::new()
        .component(None, |c| c.variable("var", None))
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{var}");
    assert_eq!(t.to_string_with_values(&v), "value");

    let t = UriTemplateBuilder::new()
        .component(None, |c| c.variable("hello", None))
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{hello}");
    assert_eq!(t.to_string_with_values(&v), "Hello%20World%21");
}

#[test]
fn test_level_2() {
    let mut v = UriTemplateValues::new();
    v.set_string("var", "value");
    v.set_string("hello", "Hello World!");
    v.set_string("path", "/foo/bar");
    let v = v;

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::ReservedCharacter), |c| c.variable("var", None))
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{+var}");
    assert_eq!(t.to_string_with_values(&v), "value");

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::ReservedCharacter), |c| c.variable("hello", None))
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{+hello}");
    assert_eq!(t.to_string_with_values(&v), "Hello%20World!");

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::ReservedCharacter), |c| c.variable("path", None))
        .literal("/here")
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{+path}/here");
    assert_eq!(t.to_string_with_values(&v), "/foo/bar/here");

    let t = UriTemplateBuilder::new()
        .literal("here?ref=")
        .component(Some(UriTemplateOperator::ReservedCharacter), |c| c.variable("path", None))
        .into_uri_template();
    assert_eq!(t.to_template_string(), "here?ref={+path}");
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
    let v = v;

    let t = UriTemplateBuilder::new()
        .literal("map?")
        .component(None, |c|
            c.variable("x", None)
             .variable("y", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "map?{x,y}");
    assert_eq!(t.to_string_with_values(&v), "map?1024,768");

    let t = UriTemplateBuilder::new()
        .component(None, |c|
            c.variable("x", None)
             .variable("hello", None)
             .variable("y", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{x,hello,y}");
    assert_eq!(t.to_string_with_values(&v), "1024,Hello%20World%21,768");

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::ReservedCharacter), |c|
            c.variable("x", None)
             .variable("hello", None)
             .variable("y", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{+x,hello,y}");
    assert_eq!(t.to_string_with_values(&v), "1024,Hello%20World!,768");

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::ReservedCharacter), |c|
            c.variable("path", None)
             .variable("x", None)
        )
        .literal("/here")
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{+path,x}/here");
    assert_eq!(t.to_string_with_values(&v), "/foo/bar,1024/here");

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::Fragment), |c|
            c.variable("x", None)
             .variable("hello", None)
             .variable("y", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{#x,hello,y}");
    assert_eq!(t.to_string_with_values(&v), "#1024,Hello%20World!,768");

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::Fragment), |c|
            c.variable("path", None)
             .variable("x", None)
        )
        .literal("/here")
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{#path,x}/here");
    assert_eq!(t.to_string_with_values(&v), "#/foo/bar,1024/here");

    let t = UriTemplateBuilder::new()
        .literal("X")
        .component(Some(UriTemplateOperator::PathExtension), |c|
            c.variable("var", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "X{.var}");
    assert_eq!(t.to_string_with_values(&v), "X.value");

    let t = UriTemplateBuilder::new()
        .literal("X")
        .component(Some(UriTemplateOperator::PathExtension), |c|
            c.variable("x", None)
             .variable("y", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "X{.x,y}");
    assert_eq!(t.to_string_with_values(&v), "X.1024.768");

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::PathComponent), |c|
            c.variable("var", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{/var}");
    assert_eq!(t.to_string_with_values(&v), "/value");

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::PathComponent), |c|
            c.variable("var", None)
             .variable("x", None)
        )
        .literal("/here")
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{/var,x}/here");
    assert_eq!(t.to_string_with_values(&v), "/value/1024/here");

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::PathParameter), |c|
            c.variable("x", None)
             .variable("y", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{;x,y}");
    // assert_eq!(t.to_string_with_values(&v), ";x=1024;y=768");

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::PathParameter), |c|
            c.variable("x", None)
             .variable("y", None)
             .variable("empty", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{;x,y,empty}");
    // assert_eq!(t.to_string_with_values(&v), ";x=1024;y=768;empty");

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::QueryParameter), |c|
            c.variable("x", None)
             .variable("y", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{?x,y}");
    // assert_eq!(t.to_string_with_values(&v), "?x=1024&y=768");

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::QueryParameter), |c|
            c.variable("x", None)
             .variable("y", None)
             .variable("empty", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{?x,y,empty}");
    // assert_eq!(t.to_string_with_values(&v), "?x=1024&y=768&empty=");

    let t = UriTemplateBuilder::new()
        .literal("?fixed=yes")
        .component(Some(UriTemplateOperator::QueryContinuation), |c|
            c.variable("x", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "?fixed=yes{&x}");
    // assert_eq!(t.to_string_with_values(&v), "?fixed=yes&x=1024");

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::QueryContinuation), |c|
            c.variable("x", None)
             .variable("y", None)
             .variable("empty", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{&x,y,empty}");
    // assert_eq!(t.to_string_with_values(&v), "?x=1024&y=768&empty=");
}

#[test]
fn test_level_4() {
    let mut v = UriTemplateValues::new();
    v.set_string("var", "value");
    v.set_string("hello", "Hello World!");
    v.set_string("path", "/foo/bar");
    v.set_strings("list", vec!["red", "green", "blue"]);
    // v.set_assoc("keys", {"semi": ";", "dot": ".", "comma": ","});
    v.set_string("empty", "");
    v.set_string("x", "1024");
    v.set_string("y", "768");
    let v = v;

    let t = UriTemplateBuilder::new()
        .component(None, |c|
            c.variable("var", Some(UriTemplateModifier::Prefix(3)))
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{var:3}");
    assert_eq!(t.to_string_with_values(&v), "val");

    let t = UriTemplateBuilder::new()
        .component(None, |c|
            c.variable("var", Some(UriTemplateModifier::Prefix(30)))
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{var:30}");
    assert_eq!(t.to_string_with_values(&v), "value");

    let t = UriTemplateBuilder::new()
        .component(None, |c|
            c.variable("list", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{list}");
    assert_eq!(t.to_string_with_values(&v), "red,green,blue");

    let t = UriTemplateBuilder::new()
        .component(None, |c|
            c.variable("list", Some(UriTemplateModifier::Explode))
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{list*}");
    assert_eq!(t.to_string_with_values(&v), "red,green,blue");

        // ["{keys}", [
        // "comma,%2C,dot,.,semi,%3B",
        // "comma,%2C,semi,%3B,dot,.",
        // "dot,.,comma,%2C,semi,%3B",
        // "dot,.,semi,%3B,comma,%2C",
        // "semi,%3B,comma,%2C,dot,.",
        // "semi,%3B,dot,.,comma,%2C"
        // ]],
        // ["{keys*}", [
        // "comma=%2C,dot=.,semi=%3B",
        // "comma=%2C,semi=%3B,dot=.",
        // "dot=.,comma=%2C,semi=%3B",
        // "dot=.,semi=%3B,comma=%2C",
        // "semi=%3B,comma=%2C,dot=.",
        // "semi=%3B,dot=.,comma=%2C"
        // ]],

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::ReservedCharacter), |c|
            c.variable("path", Some(UriTemplateModifier::Prefix(6)))
        )
        .literal("/here")
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{+path:6}/here");
    assert_eq!(t.to_string_with_values(&v), "/foo/b/here");

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::ReservedCharacter), |c|
            c.variable("list", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{+list}");
    assert_eq!(t.to_string_with_values(&v), "red,green,blue");

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::ReservedCharacter), |c|
            c.variable("list", Some(UriTemplateModifier::Explode))
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{+list*}");
    assert_eq!(t.to_string_with_values(&v), "red,green,blue");

        // ["{+keys}", [
        // "comma,,,dot,.,semi,;",
        // "comma,,,semi,;,dot,.",
        // "dot,.,comma,,,semi,;",
        // "dot,.,semi,;,comma,,",
        // "semi,;,comma,,,dot,.",
        // "semi,;,dot,.,comma,,"
        // ]],
        // ["{+keys*}", [
        // "comma=,,dot=.,semi=;",
        // "comma=,,semi=;,dot=.",
        // "dot=.,comma=,,semi=;",
        // "dot=.,semi=;,comma=,",
        // "semi=;,comma=,,dot=.",
        // "semi=;,dot=.,comma=,"
        // ]],

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::Fragment), |c|
            c.variable("path", Some(UriTemplateModifier::Prefix(6)))
        )
        .literal("/here")
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{#path:6}/here");
    assert_eq!(t.to_string_with_values(&v), "#/foo/b/here");

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::Fragment), |c|
            c.variable("list", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{#list}");
    assert_eq!(t.to_string_with_values(&v), "#red,green,blue");

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::Fragment), |c|
            c.variable("list", Some(UriTemplateModifier::Explode))
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{#list*}");
    assert_eq!(t.to_string_with_values(&v), "#red,green,blue");

        // ["{#keys}", [
        // "#comma,,,dot,.,semi,;",
        // "#comma,,,semi,;,dot,.",
        // "#dot,.,comma,,,semi,;",
        // "#dot,.,semi,;,comma,,",
        // "#semi,;,comma,,,dot,.",
        // "#semi,;,dot,.,comma,,"
        // ]],
        // ["{#keys*}", [
        // "#comma=,,dot=.,semi=;",
        // "#comma=,,semi=;,dot=.",
        // "#dot=.,comma=,,semi=;",
        // "#dot=.,semi=;,comma=,",
        // "#semi=;,comma=,,dot=.",
        // "#semi=;,dot=.,comma=,"
        // ]],

    let t = UriTemplateBuilder::new()
        .literal("X")
        .component(Some(UriTemplateOperator::PathExtension), |c|
            c.variable("var", Some(UriTemplateModifier::Prefix(3)))
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "X{.var:3}");
    assert_eq!(t.to_string_with_values(&v), "X.val");

    let t = UriTemplateBuilder::new()
        .literal("X")
        .component(Some(UriTemplateOperator::PathExtension), |c|
            c.variable("list", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "X{.list}");
    assert_eq!(t.to_string_with_values(&v), "X.red,green,blue");

    let t = UriTemplateBuilder::new()
        .literal("X")
        .component(Some(UriTemplateOperator::PathExtension), |c|
            c.variable("list", Some(UriTemplateModifier::Explode))
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "X{.list*}");
    assert_eq!(t.to_string_with_values(&v), "X.red.green.blue");

        // ["X{.keys}", [
        // "X.comma,%2C,dot,.,semi,%3B",
        // "X.comma,%2C,semi,%3B,dot,.",
        // "X.dot,.,comma,%2C,semi,%3B",
        // "X.dot,.,semi,%3B,comma,%2C",
        // "X.semi,%3B,comma,%2C,dot,.",
        // "X.semi,%3B,dot,.,comma,%2C"
        // ]],
    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::PathComponent), |c|
            c.variable("var", Some(UriTemplateModifier::Prefix(1)))
             .variable("var", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{/var:1,var}");
    assert_eq!(t.to_string_with_values(&v), "/v/value");

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::PathComponent), |c|
            c.variable("list", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{/list}");
    assert_eq!(t.to_string_with_values(&v), "/red,green,blue");

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::PathComponent), |c|
            c.variable("list", Some(UriTemplateModifier::Explode))
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{/list*}");
    assert_eq!(t.to_string_with_values(&v), "/red/green/blue");

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::PathComponent), |c|
            c.variable("list", Some(UriTemplateModifier::Explode))
             .variable("path", Some(UriTemplateModifier::Prefix(4)))
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{/list*,path:4}");
    assert_eq!(t.to_string_with_values(&v), "/red/green/blue/%2Ffoo");

        // ["{/keys}", [
        // "/comma,%2C,dot,.,semi,%3B",
        // "/comma,%2C,semi,%3B,dot,.",
        // "/dot,.,comma,%2C,semi,%3B",
        // "/dot,.,semi,%3B,comma,%2C",
        // "/semi,%3B,comma,%2C,dot,.",
        // "/semi,%3B,dot,.,comma,%2C"
        // ]],
        // ["{/keys*}", [
        // "/comma=%2C/dot=./semi=%3B",
        // "/comma=%2C/semi=%3B/dot=.",
        // "/dot=./comma=%2C/semi=%3B",
        // "/dot=./semi=%3B/comma=%2C",
        // "/semi=%3B/comma=%2C/dot=.",
        // "/semi=%3B/dot=./comma=%2C"
        // ]],
        let t = UriTemplateBuilder::new()
            .component(Some(UriTemplateOperator::PathParameter), |c|
                c.variable("hello", Some(UriTemplateModifier::Prefix(5)))
            )
            .into_uri_template();
        assert_eq!(t.to_template_string(), "{;hello:5}");
        // assert_eq!(t.to_string_with_values(&v), ";hello=Hello");

        let t = UriTemplateBuilder::new()
            .component(Some(UriTemplateOperator::PathParameter), |c|
                c.variable("list", None)
            )
            .into_uri_template();
        assert_eq!(t.to_template_string(), "{;list}");
        // assert_eq!(t.to_string_with_values(&v), ";list=red,green,blue");

        let t = UriTemplateBuilder::new()
            .component(Some(UriTemplateOperator::PathParameter), |c|
                c.variable("list", Some(UriTemplateModifier::Explode))
            )
            .into_uri_template();
        assert_eq!(t.to_template_string(), "{;list*}");
        // assert_eq!(t.to_string_with_values(&v), ";list=red;list=green;list=blue");

        // ["{;keys}", [
        // ";keys=comma,%2C,dot,.,semi,%3B",
        // ";keys=comma,%2C,semi,%3B,dot,.",
        // ";keys=dot,.,comma,%2C,semi,%3B",
        // ";keys=dot,.,semi,%3B,comma,%2C",
        // ";keys=semi,%3B,comma,%2C,dot,.",
        // ";keys=semi,%3B,dot,.,comma,%2C"
        // ]],
        // ["{;keys*}", [
        // ";comma=%2C;dot=.;semi=%3B",
        // ";comma=%2C;semi=%3B;dot=.",
        // ";dot=.;comma=%2C;semi=%3B",
        // ";dot=.;semi=%3B;comma=%2C",
        // ";semi=%3B;comma=%2C;dot=.",
        // ";semi=%3B;dot=.;comma=%2C"
        // ]],

        let t = UriTemplateBuilder::new()
            .component(Some(UriTemplateOperator::QueryParameter), |c|
                c.variable("var", Some(UriTemplateModifier::Prefix(3)))
            )
            .into_uri_template();
        assert_eq!(t.to_template_string(), "{?var:3}");
        // assert_eq!(t.to_string_with_values(&v), "?var=val");

        let t = UriTemplateBuilder::new()
            .component(Some(UriTemplateOperator::QueryParameter), |c|
                c.variable("list", None)
            )
            .into_uri_template();
        assert_eq!(t.to_template_string(), "{?list}");
        // assert_eq!(t.to_string_with_values(&v), "?list=red,green,blue");

        let t = UriTemplateBuilder::new()
            .component(Some(UriTemplateOperator::QueryParameter), |c|
                c.variable("list", Some(UriTemplateModifier::Explode))
            )
            .into_uri_template();
        assert_eq!(t.to_template_string(), "{?list*}");
        // assert_eq!(t.to_string_with_values(&v), "?list=red&list=green&list=blue");

        // ["{?keys}", [
        // "?keys=comma,%2C,dot,.,semi,%3B",
        // "?keys=comma,%2C,semi,%3B,dot,.",
        // "?keys=dot,.,comma,%2C,semi,%3B",
        // "?keys=dot,.,semi,%3B,comma,%2C",
        // "?keys=semi,%3B,comma,%2C,dot,.",
        // "?keys=semi,%3B,dot,.,comma,%2C"
        // ]],
        // ["{?keys*}", [
        // "?comma=%2C&dot=.&semi=%3B",
        // "?comma=%2C&semi=%3B&dot=.",
        // "?dot=.&comma=%2C&semi=%3B",
        // "?dot=.&semi=%3B&comma=%2C",
        // "?semi=%3B&comma=%2C&dot=.",
        // "?semi=%3B&dot=.&comma=%2C"
        // ]],

        let t = UriTemplateBuilder::new()
            .component(Some(UriTemplateOperator::QueryContinuation), |c|
                c.variable("var", Some(UriTemplateModifier::Prefix(3)))
            )
            .into_uri_template();
        assert_eq!(t.to_template_string(), "{&var:3}");
        // assert_eq!(t.to_string_with_values(&v), "&var=val");

        let t = UriTemplateBuilder::new()
            .component(Some(UriTemplateOperator::QueryContinuation), |c|
                c.variable("list", None)
            )
            .into_uri_template();
        assert_eq!(t.to_template_string(), "{&list}");
        // assert_eq!(t.to_string_with_values(&v), "&list=red,green,blue");

        let t = UriTemplateBuilder::new()
            .component(Some(UriTemplateOperator::QueryContinuation), |c|
                c.variable("list", Some(UriTemplateModifier::Explode))
            )
            .into_uri_template();
        assert_eq!(t.to_template_string(), "{&list*}");
        // assert_eq!(t.to_string_with_values(&v), "&list=red&list=green&list=blue");

        // ["{&keys}", [
        // "&keys=comma,%2C,dot,.,semi,%3B",
        // "&keys=comma,%2C,semi,%3B,dot,.",
        // "&keys=dot,.,comma,%2C,semi,%3B",
        // "&keys=dot,.,semi,%3B,comma,%2C",
        // "&keys=semi,%3B,comma,%2C,dot,.",
        // "&keys=semi,%3B,dot,.,comma,%2C"
        // ]],
        // ["{&keys*}", [
        // "&comma=%2C&dot=.&semi=%3B",
        // "&comma=%2C&semi=%3B&dot=.",
        // "&dot=.&comma=%2C&semi=%3B",
        // "&dot=.&semi=%3B&comma=%2C",
        // "&semi=%3B&comma=%2C&dot=.",
        // "&semi=%3B&dot=.&comma=%2C"
        // ]]
}
