extern crate argparse;
extern crate iron;

use argparse::{ArgumentParser, StoreTrue, Store};
use iron::prelude::*;

fn http_server(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "Rust server")))
}

static HTTP_ADDR: &'static str = "localhost:4242";

fn main() {
    let mut size: usize = 20;
    let mut reverse: bool = false;
    let mut server_mode: bool = false;
    let mut skin = "███".to_string();
    let padding = vec!["  ", " ", "", " ", "  ", "   ", "    ", "   "];

    {
        // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();

        ap.set_description("Print a beautiful millipede (written in rust)");
        ap.refer(&mut size)
          .add_option(&["-w", "--width"], Store, "millipede width");
        ap.refer(&mut skin)
          .add_option(&["-s", "--skin"], Store, "millipede skin");
        ap.refer(&mut reverse)
          .add_option(&["-r", "--reverse"], StoreTrue, "reverse the millipede");
        ap.refer(&mut server_mode)
          .add_option(&["--server"], StoreTrue, "Spawn http server");
        ap.parse_args_or_exit();
    }
    if server_mode == true {
        let chain = Chain::new(http_server);

        println!("Listening on {}", HTTP_ADDR);
        Iron::new(chain).http(HTTP_ADDR).unwrap();
    }

    if reverse == false {
        let mut i: usize = 0;

        println!("    ╚⊙ ⊙╝");
        while i < size {
            println!("{}╚═({})═╝", padding[i % 8], skin);
            i = i + 1;
        }
    } else {
        let mut i: usize = size;

        while i > 0 {
            println!("{}╔═({})═╗", padding[i % 8], skin);
            i = i - 1;
        }
        println!("   ╔⊙ ⊙╗");
    }
}
