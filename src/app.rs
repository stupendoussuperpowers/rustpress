use regex::Regex;
use std::io::{self, BufRead};
use std::net::{TcpListener, TcpStream};
use std::str::FromStr;

use crate::constants::{Method, RouteResolverMap};
use crate::request::*;
use crate::response::*;

pub struct App<'a> {
    route_map_list: Vec<RouteResolverMap<'a>>,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        let route_list: Vec<RouteResolverMap<'a>> = vec![];

        return Self {
            route_map_list: route_list,
        };
    }

    pub fn listen(&self, port: i32) {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
        println!("Listening on port {}", port);

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            App::handle_connection(stream, &self.route_map_list);
        }
    }

    pub fn register_route(
        &mut self,
        method: Method,
        route: String,
        handler: &'a dyn Fn(&mut Request, &mut Response) -> (),
    ) {
        self.route_map_list.push(RouteResolverMap {
            route: route,
            method: method,
            handler: handler,
        })
    }

    fn handle_connection(mut stream: TcpStream, route_map_list: &Vec<RouteResolverMap>) {
        let mut reader = io::BufReader::new(&stream);

        let revd: Vec<u8> = reader.fill_buf().unwrap().to_vec();

        reader.consume(revd.len());

        let req_str = std::str::from_utf8(&revd).unwrap();

        let mut res: Response = Response {
            content: "".to_string(),
            status: 200,
            stream: &mut stream,
            headers: std::collections::HashMap::new(),
        };

        let mut req = Request::from_str(req_str).unwrap();

        for routemapping in route_map_list.iter() {
            let re = Regex::new(&routemapping.route).unwrap();

            if req.method == routemapping.method && re.is_match(req.route.as_str()) {
                (routemapping.handler)(&mut req, &mut res);
                let s = format!(
                    "{} {} {}",
                    req.method.to_string(),
                    res.status.to_string(),
                    req.route.to_string()
                );
                println!("{}", s);
                break;
            }
        }
    }
}
