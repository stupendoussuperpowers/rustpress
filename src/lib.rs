use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use std::net::{TcpListener, TcpStream};

#[derive(PartialEq, Eq, Debug)]
enum Method {
    GET,
    POST,
    PUT,
    DELETE,
}

#[derive(PartialEq, Eq, Debug)]
struct MethodParseErr;

impl std::str::FromStr for Method {
    type Err = MethodParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "GET" {
            return Ok(Method::GET);
        }
        if s == "POST" {
            return Ok(Method::POST);
        }
        if s == "PUT" {
            return Ok(Method::PUT);
        }
        if s == "DELETE" {
            return Ok(Method::DELETE);
        }

        Err(MethodParseErr)
    }
}

struct RouteResolverMap<'a> {
    route: String,
    method: Method,
    handler: &'a dyn Fn(&mut Request, &mut Response) -> (),
}

struct App<'a> {
    route_map_list: Vec<RouteResolverMap<'a>>,
}

pub struct Response {}

#[derive(Debug)]
pub struct Request {
    method: Method,
    route: String,
    protocol: String,
    body: String,
    hash: HashMap<String, String>,
}

impl Request {
    pub fn from_string(&mut self, req_string: String) -> &mut Request {
        let req_vec: Vec<String> = req_string.split("\r\n").map(|s| s.to_string()).collect();

        let header: Vec<String> = req_vec[0].split(" ").map(|s| s.to_string()).collect();

        (self.method, self.route, self.protocol) = (
            header[0].to_string().parse().unwrap(),
            header[1].to_string(),
            header[2].to_string(),
        );

        let mut body_start = false;

        for line in req_vec.iter().skip(1) {
            if line.is_empty() {
                body_start = true;
            }

            if body_start {
                self.body.push_str(line);
            } else {
                let keyval: Vec<String> = line.split(":").map(|s| s.to_string()).collect();

                println!("{:#?}", keyval);

                self.hash
                    .insert(keyval[0].to_string(), keyval[1].to_string());
            }
        }

        self
    }

    pub fn get(self, property: &String) -> String {
        self.hash.get(property).unwrap().to_string()
    }
}

impl App {
    pub fn listen(&self, port: i32) {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
        println!("Listening on port {}", port);

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            App::handle_connection(stream, &self.route_map_list);
        }
    }

    fn register_route(
        &mut self,
        method: Method,
        route: String,
        handler: &'_ dyn Fn(&mut Request, &mut Response) -> (),
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

        let req_vec: Vec<String> = req_str.split("\r\n").map(|s| s.to_string()).collect();

        println!("{:#?}", req_vec);

        let mut req: Request = Request {
            method: Method::GET,
            route: "".to_string(),
            protocol: "".to_string(),
            body: "".to_string(),
            hash: HashMap::new(),
        };

        let mut res: Response = Response {};

        req.from_string(req_str.to_string());

        println!("{:#?}", req);

        for routemapping in route_map_list.iter() {
            if req.method == routemapping.method && req.route.starts_with(&routemapping.route) {
                (routemapping.handler)(&mut req, &mut res);
            }
        }

        stream
            .write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes())
            .unwrap();
    }
}
