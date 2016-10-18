#[macro_use] extern crate nickel;

// use nickel::Nickel;

use std::collections::HashMap;
use nickel::{Nickel, HttpRouter, StaticFilesHandler};

fn main() {
    let mut server = Nickel::new();

     /*
     server.utilize(router! {
         get "**" => |_req, _res| {
             "Hello world! Good bye!"
         }
     });
     */

    server.utilize(StaticFilesHandler::new("assets/"));

    server.get("/", middleware! {|_, response|
        let mut data = HashMap::new();
        data.insert("color", "Green");
        data.insert("name", "California Apple");
        data.insert("price", "2.50");
        return response.render("assets/hello.tpl", &data);
    });

     let _ = server.listen("127.0.0.1:6767");
 }
