extern crate uritemplates;

use uritemplates::UriTemplateBuilder;
use uritemplates::{UriTemplateOperator, UriTemplateModifier};
use uritemplates::{UriTemplateValues};


fn test_level_1_values() -> UriTemplateValues {
    let mut v = UriTemplateValues::new();
    v.set_string("var", "value");
    v.set_string("hello", "Hello World!");
    v
}

#[test]
fn test_level_1_a() {
    let v = test_level_1_values();

    let t = UriTemplateBuilder::new()
        .component(None, |c| c.variable("var", None))
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{var}");
    assert_eq!(t.to_string_with_values(&v), "value");
}

#[test]
fn test_level_1_b() {
    let v = test_level_1_values();

    let t = UriTemplateBuilder::new()
        .component(None, |c| c.variable("hello", None))
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{hello}");
    assert_eq!(t.to_string_with_values(&v), "Hello%20World%21");
}


fn test_level_2_values() -> UriTemplateValues {
    let mut v = UriTemplateValues::new();
    v.set_string("var", "value");
    v.set_string("hello", "Hello World!");
    v.set_string("path", "/foo/bar");
    v
}

#[test]
fn test_level_2_a() {
    let v = test_level_2_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::ReservedCharacter), |c| c.variable("var", None))
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{+var}");
    assert_eq!(t.to_string_with_values(&v), "value");
}

#[test]
fn test_level_2_b() {
    let v = test_level_2_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::ReservedCharacter), |c| c.variable("hello", None))
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{+hello}");
    assert_eq!(t.to_string_with_values(&v), "Hello%20World!");
}

#[test]
fn test_level_2_c() {
    let v = test_level_2_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::ReservedCharacter), |c| c.variable("path", None))
        .literal("/here")
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{+path}/here");
    assert_eq!(t.to_string_with_values(&v), "/foo/bar/here");
}

#[test]
fn test_level_2_d() {
    let v = test_level_2_values();

    let t = UriTemplateBuilder::new()
        .literal("here?ref=")
        .component(Some(UriTemplateOperator::ReservedCharacter), |c| c.variable("path", None))
        .into_uri_template();
    assert_eq!(t.to_template_string(), "here?ref={+path}");
    assert_eq!(t.to_string_with_values(&v), "here?ref=/foo/bar");
}


fn test_level_3_values() -> UriTemplateValues {
    let mut v = UriTemplateValues::new();
    v.set_string("var", "value");
    v.set_string("hello", "Hello World!");
    v.set_string("empty", "");
    v.set_string("path", "/foo/bar");
    v.set_string("x", "1024");
    v.set_string("y", "768");
    v
}

#[test]
fn test_level_3_a() {
    let v = test_level_3_values();

    let t = UriTemplateBuilder::new()
        .literal("map?")
        .component(None, |c|
            c.variable("x", None)
             .variable("y", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "map?{x,y}");
    assert_eq!(t.to_string_with_values(&v), "map?1024,768");
}

#[test]
fn test_level_3_b() {
    let v = test_level_3_values();

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
fn test_level_3_c() {
    let v = test_level_3_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::ReservedCharacter), |c|
            c.variable("x", None)
             .variable("hello", None)
             .variable("y", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{+x,hello,y}");
    assert_eq!(t.to_string_with_values(&v), "1024,Hello%20World!,768");
}

#[test]
fn test_level_3_d() {
    let v = test_level_3_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::ReservedCharacter), |c|
            c.variable("path", None)
             .variable("x", None)
        )
        .literal("/here")
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{+path,x}/here");
    assert_eq!(t.to_string_with_values(&v), "/foo/bar,1024/here");
}

#[test]
fn test_level_3_e() {
    let v = test_level_3_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::Fragment), |c|
            c.variable("x", None)
             .variable("hello", None)
             .variable("y", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{#x,hello,y}");
    assert_eq!(t.to_string_with_values(&v), "#1024,Hello%20World!,768");
}

#[test]
fn test_level_3_f() {
    let v = test_level_3_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::Fragment), |c|
            c.variable("path", None)
             .variable("x", None)
        )
        .literal("/here")
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{#path,x}/here");
    assert_eq!(t.to_string_with_values(&v), "#/foo/bar,1024/here");
}

#[test]
fn test_level_3_g() {
    let v = test_level_3_values();

    let t = UriTemplateBuilder::new()
        .literal("X")
        .component(Some(UriTemplateOperator::PathExtension), |c|
            c.variable("var", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "X{.var}");
    assert_eq!(t.to_string_with_values(&v), "X.value");
}

#[test]
fn test_level_3_h() {
    let v = test_level_3_values();

    let t = UriTemplateBuilder::new()
        .literal("X")
        .component(Some(UriTemplateOperator::PathExtension), |c|
            c.variable("x", None)
             .variable("y", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "X{.x,y}");
    assert_eq!(t.to_string_with_values(&v), "X.1024.768");
}

#[test]
fn test_level_3_i() {
    let v = test_level_3_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::PathComponent), |c|
            c.variable("var", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{/var}");
    assert_eq!(t.to_string_with_values(&v), "/value");
}

#[test]
fn test_level_3_j() {
    let v = test_level_3_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::PathComponent), |c|
            c.variable("var", None)
             .variable("x", None)
        )
        .literal("/here")
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{/var,x}/here");
    assert_eq!(t.to_string_with_values(&v), "/value/1024/here");
}

#[test]
fn test_level_3_k() {
    // let v = test_level_3_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::PathParameter), |c|
            c.variable("x", None)
             .variable("y", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{;x,y}");
    // assert_eq!(t.to_string_with_values(&v), ";x=1024;y=768");
}

#[test]
fn test_level_3_l() {
    // let v = test_level_3_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::PathParameter), |c|
            c.variable("x", None)
             .variable("y", None)
             .variable("empty", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{;x,y,empty}");
    // assert_eq!(t.to_string_with_values(&v), ";x=1024;y=768;empty");
}

#[test]
fn test_level_3_m() {
    // let v = test_level_3_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::QueryParameter), |c|
            c.variable("x", None)
             .variable("y", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{?x,y}");
    // assert_eq!(t.to_string_with_values(&v), "?x=1024&y=768");
}

#[test]
fn test_level_3_n() {
    // let v = test_level_3_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::QueryParameter), |c|
            c.variable("x", None)
             .variable("y", None)
             .variable("empty", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{?x,y,empty}");
    // assert_eq!(t.to_string_with_values(&v), "?x=1024&y=768&empty=");
}

#[test]
fn test_level_3_o() {
    // let v = test_level_3_values();

    let t = UriTemplateBuilder::new()
        .literal("?fixed=yes")
        .component(Some(UriTemplateOperator::QueryContinuation), |c|
            c.variable("x", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "?fixed=yes{&x}");
    // assert_eq!(t.to_string_with_values(&v), "?fixed=yes&x=1024");
}

#[test]
fn test_level_3_p() {
    // let v = test_level_3_values();

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


fn test_level_4_values() -> UriTemplateValues {
    let mut v = UriTemplateValues::new();
    v.set_string("var", "value");
    v.set_string("hello", "Hello World!");
    v.set_string("path", "/foo/bar");
    v.set_strings("list", vec!["red", "green", "blue"]);
    // v.set_assoc("keys", {"semi": ";", "dot": ".", "comma": ","});
    v.set_string("empty", "");
    v.set_string("x", "1024");
    v.set_string("y", "768");
    v
}

#[test]
fn test_level_4_a() {
    let v = test_level_4_values();

    let t = UriTemplateBuilder::new()
        .component(None, |c|
            c.variable("var", Some(UriTemplateModifier::Prefix(3)))
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{var:3}");
    assert_eq!(t.to_string_with_values(&v), "val");
}

#[test]
fn test_level_4_b() {
    let v = test_level_4_values();

    let t = UriTemplateBuilder::new()
        .component(None, |c|
            c.variable("var", Some(UriTemplateModifier::Prefix(30)))
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{var:30}");
    assert_eq!(t.to_string_with_values(&v), "value");
}

#[test]
fn test_level_4_c() {
    let v = test_level_4_values();

    let t = UriTemplateBuilder::new()
        .component(None, |c|
            c.variable("list", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{list}");
    assert_eq!(t.to_string_with_values(&v), "red,green,blue");
}

#[test]
fn test_level_4_d() {
    let v = test_level_4_values();

    let t = UriTemplateBuilder::new()
        .component(None, |c|
            c.variable("list", Some(UriTemplateModifier::Explode))
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{list*}");
    assert_eq!(t.to_string_with_values(&v), "red,green,blue");
}

#[test]
fn test_level_4_e() {
    // let v = test_level_4_values();

    let t = UriTemplateBuilder::new()
        .component(None, |c|
            c.variable("keys", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{keys}");
    // assert_eq!(t.to_string_with_values(&v), "comma,%2C,dot,.,semi,%3B");
}

#[test]
fn test_level_4_f() {
    // let v = test_level_4_values();

    let t = UriTemplateBuilder::new()
        .component(None, |c|
            c.variable("keys", Some(UriTemplateModifier::Explode))
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{keys*}");
    // assert_eq!(t.to_string_with_values(&v), "comma=%2C,dot=.,semi=%3B");
}

#[test]
fn test_level_4_g() {
    let v = test_level_4_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::ReservedCharacter), |c|
            c.variable("path", Some(UriTemplateModifier::Prefix(6)))
        )
        .literal("/here")
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{+path:6}/here");
    assert_eq!(t.to_string_with_values(&v), "/foo/b/here");
}

#[test]
fn test_level_4_h() {
    let v = test_level_4_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::ReservedCharacter), |c|
            c.variable("list", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{+list}");
    assert_eq!(t.to_string_with_values(&v), "red,green,blue");
}

#[test]
fn test_level_4_i() {
    let v = test_level_4_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::ReservedCharacter), |c|
            c.variable("list", Some(UriTemplateModifier::Explode))
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{+list*}");
    assert_eq!(t.to_string_with_values(&v), "red,green,blue");
}

#[test]
fn test_level_4_j() {
    // let v = test_level_4_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::ReservedCharacter), |c|
            c.variable("keys", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{+keys}");
    // assert_eq!(t.to_string_with_values(&v), "comma,,,dot,.,semi,;");
}

#[test]
fn test_level_4_k() {
    // let v = test_level_4_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::ReservedCharacter), |c|
            c.variable("keys", Some(UriTemplateModifier::Explode))
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{+keys*}");
    // assert_eq!(t.to_string_with_values(&v), "comma=,,dot=.,semi=;");
}

#[test]
fn test_level_4_l() {
    let v = test_level_4_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::Fragment), |c|
            c.variable("path", Some(UriTemplateModifier::Prefix(6)))
        )
        .literal("/here")
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{#path:6}/here");
    assert_eq!(t.to_string_with_values(&v), "#/foo/b/here");
}

#[test]
fn test_level_4_m() {
    let v = test_level_4_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::Fragment), |c|
            c.variable("list", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{#list}");
    assert_eq!(t.to_string_with_values(&v), "#red,green,blue");
}

#[test]
fn test_level_4_n() {
    let v = test_level_4_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::Fragment), |c|
            c.variable("list", Some(UriTemplateModifier::Explode))
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{#list*}");
    assert_eq!(t.to_string_with_values(&v), "#red,green,blue");
}

#[test]
fn test_level_4_p() {
    // let v = test_level_4_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::Fragment), |c|
            c.variable("keys", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{#keys}");
    // assert_eq!(t.to_string_with_values(&v), "#comma,,,dot,.,semi,;");
}

#[test]
fn test_level_4_q() {
    // let v = test_level_4_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::Fragment), |c|
            c.variable("keys", Some(UriTemplateModifier::Explode))
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{#keys*}");
    // assert_eq!(t.to_string_with_values(&v), "#comma=,,dot=.,semi=;");
}

#[test]
fn test_level_4_r() {
    let v = test_level_4_values();

    let t = UriTemplateBuilder::new()
        .literal("X")
        .component(Some(UriTemplateOperator::PathExtension), |c|
            c.variable("var", Some(UriTemplateModifier::Prefix(3)))
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "X{.var:3}");
    assert_eq!(t.to_string_with_values(&v), "X.val");
}

#[test]
fn test_level_4_s() {
    let v = test_level_4_values();

    let t = UriTemplateBuilder::new()
        .literal("X")
        .component(Some(UriTemplateOperator::PathExtension), |c|
            c.variable("list", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "X{.list}");
    assert_eq!(t.to_string_with_values(&v), "X.red,green,blue");
}

#[test]
fn test_level_4_t() {
    let v = test_level_4_values();

    let t = UriTemplateBuilder::new()
        .literal("X")
        .component(Some(UriTemplateOperator::PathExtension), |c|
            c.variable("list", Some(UriTemplateModifier::Explode))
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "X{.list*}");
    assert_eq!(t.to_string_with_values(&v), "X.red.green.blue");
}

#[test]
fn test_level_4_u() {
    // let v = test_level_4_values();

    let t = UriTemplateBuilder::new()
        .literal("X")
        .component(Some(UriTemplateOperator::PathExtension), |c|
            c.variable("keys", Some(UriTemplateModifier::Explode))
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "X{.keys*}");
    // assert_eq!(t.to_string_with_values(&v), "X.comma,%2C,dot,.,semi,%3B");
}

#[test]
fn test_level_4_v() {
    let v = test_level_4_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::PathComponent), |c|
            c.variable("var", Some(UriTemplateModifier::Prefix(1)))
             .variable("var", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{/var:1,var}");
    assert_eq!(t.to_string_with_values(&v), "/v/value");
}

#[test]
fn test_level_4_w() {
    let v = test_level_4_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::PathComponent), |c|
            c.variable("list", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{/list}");
    assert_eq!(t.to_string_with_values(&v), "/red,green,blue");
}

#[test]
fn test_level_4_x() {
    let v = test_level_4_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::PathComponent), |c|
            c.variable("list", Some(UriTemplateModifier::Explode))
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{/list*}");
    assert_eq!(t.to_string_with_values(&v), "/red/green/blue");
}

#[test]
fn test_level_4_y() {
    let v = test_level_4_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::PathComponent), |c|
            c.variable("list", Some(UriTemplateModifier::Explode))
             .variable("path", Some(UriTemplateModifier::Prefix(4)))
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{/list*,path:4}");
    assert_eq!(t.to_string_with_values(&v), "/red/green/blue/%2Ffoo");
}

#[test]
fn test_level_4_z() {
    // let v = test_level_4_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::PathComponent), |c|
            c.variable("keys", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{/keys}");
    // assert_eq!(t.to_string_with_values(&v), "/comma,%2C,dot,.,semi,%3B");
}

#[test]
fn test_level_4_aa() {
    // let v = test_level_4_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::PathComponent), |c|
            c.variable("keys", Some(UriTemplateModifier::Explode))
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{/keys*}");
    // assert_eq!(t.to_string_with_values(&v), "/comma=%2C/dot=./semi=%3B");
}

#[test]
fn test_level_4_ab() {
    // let v = test_level_4_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::PathParameter), |c|
            c.variable("hello", Some(UriTemplateModifier::Prefix(5)))
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{;hello:5}");
    // assert_eq!(t.to_string_with_values(&v), ";hello=Hello");
}

#[test]
fn test_level_4_ac() {
    // let v = test_level_4_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::PathParameter), |c|
            c.variable("list", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{;list}");
    // assert_eq!(t.to_string_with_values(&v), ";list=red,green,blue");
}

#[test]
fn test_level_4_ad() {
    // let v = test_level_4_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::PathParameter), |c|
            c.variable("list", Some(UriTemplateModifier::Explode))
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{;list*}");
    // assert_eq!(t.to_string_with_values(&v), ";list=red;list=green;list=blue");
}

#[test]
fn test_level_4_ae() {
    // let v = test_level_4_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::PathParameter), |c|
            c.variable("keys", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{;keys}");
    // assert_eq!(t.to_string_with_values(&v), ";keys=comma,%2C,dot,.,semi,%3B");
}

#[test]
fn test_level_4_af() {
    // let v = test_level_4_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::PathParameter), |c|
            c.variable("keys", Some(UriTemplateModifier::Explode))
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{;keys*}");
    // assert_eq!(t.to_string_with_values(&v), ";comma=%2C;dot=.;semi=%3B");
}

#[test]
fn test_level_4_ag() {
    // let v = test_level_4_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::QueryParameter), |c|
            c.variable("var", Some(UriTemplateModifier::Prefix(3)))
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{?var:3}");
    // assert_eq!(t.to_string_with_values(&v), "?var=val");
}

#[test]
fn test_level_4_ah() {
    // let v = test_level_4_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::QueryParameter), |c|
            c.variable("list", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{?list}");
    // assert_eq!(t.to_string_with_values(&v), "?list=red,green,blue");
}

#[test]
fn test_level_4_ai() {
    // let v = test_level_4_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::QueryParameter), |c|
            c.variable("list", Some(UriTemplateModifier::Explode))
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{?list*}");
    // assert_eq!(t.to_string_with_values(&v), "?list=red&list=green&list=blue");
}

#[test]
fn test_level_4_aj() {
    // let v = test_level_4_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::QueryParameter), |c|
            c.variable("keys", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{?keys}");
    // assert_eq!(t.to_string_with_values(&v), "?keys=comma,%2C,dot,.,semi,%3B");
}

#[test]
fn test_level_4_ak() {
    // let v = test_level_4_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::QueryParameter), |c|
            c.variable("keys", Some(UriTemplateModifier::Explode))
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{?keys*}");
    // assert_eq!(t.to_string_with_values(&v), "?comma=%2C&dot=.&semi=%3B");
}

#[test]
fn test_level_4_al() {
    // let v = test_level_4_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::QueryContinuation), |c|
            c.variable("var", Some(UriTemplateModifier::Prefix(3)))
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{&var:3}");
    // assert_eq!(t.to_string_with_values(&v), "&var=val");
}

#[test]
fn test_level_4_am() {
    // let v = test_level_4_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::QueryContinuation), |c|
            c.variable("list", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{&list}");
    // assert_eq!(t.to_string_with_values(&v), "&list=red,green,blue");
}

#[test]
fn test_level_4_an() {
    // let v = test_level_4_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::QueryContinuation), |c|
            c.variable("list", Some(UriTemplateModifier::Explode))
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{&list*}");
    // assert_eq!(t.to_string_with_values(&v), "&list=red&list=green&list=blue");
}

#[test]
fn test_level_4_ao() {
    // let v = test_level_4_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::QueryContinuation), |c|
            c.variable("keys", None)
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{&keys}");
    // assert_eq!(t.to_string_with_values(&v), "&keys=comma,%2C,dot,.,semi,%3B");
}

#[test]
fn test_level_4_ap() {
    // let v = test_level_4_values();

    let t = UriTemplateBuilder::new()
        .component(Some(UriTemplateOperator::QueryContinuation), |c|
            c.variable("keys", Some(UriTemplateModifier::Explode))
        )
        .into_uri_template();
    assert_eq!(t.to_template_string(), "{&keys*}");
    // assert_eq!(t.to_string_with_values(&v), "&comma=%2C&dot=.&semi=%3B");
}
