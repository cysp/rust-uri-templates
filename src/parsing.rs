// use UriTemplates::types;
//
// pub struct Parser {
//     enum Token {
//         Component()
//     }
//
//     pub fn parse_byte(&self, ctx: &)
//     pub fn from_str(template: &str) -> Parser {
//
//     }
// }

// #[cfg(test)]
// mod test {
//     use std::collections::HashMap;
//     use super::UriTemplate;
//
//     #[test]
//     fn test_1() {
//         let t = match UriTemplate::from_str("http://example.com/{?q}") {
//             Some(t) => t,
//             None => return,
//         };
//
//         let u = t.to_url().expect("");
//         assert_eq!(u.serialize(), "http://example.com/");
//     }
//
//     #[test]
//     fn test_2() {
//         let t = match UriTemplate::from_str("http://example.com/{?q}") {
//             Some(t) => t,
//             None => return,
//         };
//
//         let mut v = HashMap::<&str, &str>::new();
//         v.insert("q", "something");
//
//         let u = t.to_url_with_values(v).expect("");
//         assert_eq!(u.serialize(), "http://example.com/?q=something");
//     }
//
// }
