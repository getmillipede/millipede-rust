extern crate argparse;
use argparse::{ArgumentParser, Store};

fn main() {
    let mut size:i32 = 20;
    let padding = vec!["  ", " ", "", " ", "  ", "   ", "    ", "   "];

    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();

        ap.set_description("Print a beautiful millipede (written in rust)");
        ap.refer(&mut size)
            .add_option(&["-s", "--size"], Store,
            "Be verbose");
        ap.parse_args_or_exit();
    }
    if size >= 0 {
        let mut i: usize = 0;

        println!("    ╚⊙ ⊙╝");
        while i < size as usize {
            println!("{}╚═(███)═╝", padding[i % 8]);
            i = i + 1;
        }
    } else {
        let mut i: usize = size.abs() as usize;

        while i > 0 {
            println!("{}╔═(███)═╗", padding[i % 8]);
            i = i - 1;
        }
        println!("   ╔⊙ ⊙╗");
    }
}
