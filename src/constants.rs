use crate::request::*;
use crate::response::*;

#[derive(PartialEq, Eq, Debug, strum_macros::Display, strum_macros::EnumString)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
}

pub struct RouteResolverMap<'a> {
    pub route: String,
    pub method: Method,
    pub handler: &'a dyn Fn(&mut Request, &mut Response) -> (),
}
