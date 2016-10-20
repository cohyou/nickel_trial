#[macro_use] extern crate nickel;

// use nickel::Nickel;

use std::collections::HashMap;
use nickel::{Nickel, HttpRouter, StaticFilesHandler};

/*
BriqsDBでの'Briq'という概念は、最初はコンスセルの別名に過ぎなかったのだが、
文字列や任意のバイト列をそれで表現したいと思った時に、'128bitの塊'という役割も背負うことになった。
そのせいで、自分の中で混乱が生じているのだ。

果たして、上記のどちらの概念が、よりプリミティブなのか？
*/
trait Piq {
    fn p(&self);
    fn q(&self);
}

/*
ということで、基本はポインタとなる64bitの塊ということにしました。
上記のどちらの概念の間に、あるもの。dominoにこだわるのは、少しやめる感じでしょうか。
*/
struct Briq {
    bytes: [u8; 8]
}

struct Omino1 {
    briq: Briq
}

struct Omino2 {
    p: Briq,
    q: Briq,
}

fn main() {
    let mut server = Nickel::new();

    let o1 = Omino1 { briq: Briq { bytes: [0; 8] } };

    let o2 = Omino2 { p: Briq { bytes: [0; 8] }, q: Briq { bytes: [0; 8] } };

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
