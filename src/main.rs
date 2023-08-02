use rustpress::{app, constants, request, response};

fn main() {
    let mut app: app::App = rustpress::app::App::new();

    app.register_route(constants::Method::GET, "/hello/*".to_string(), &hello_world);

    app.listen(3000);
}

fn hello_world(request: &mut request::Request, response: &mut response::Response) {
    println!("Hello World from main.rs");

    response.content = format!(
        "Hello World from main.rs! current route: {}",
        &request.route
    )
    .to_string();

    response.send();
}
