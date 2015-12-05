use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let mut size:i32 = 20;
    let padding = vec!["  ", " ", "", " ", "  ", "   ", "    ", "   "];

    if args.len() > 1 {
        size = args[1].parse()
                .ok()
                .expect("Wanted a number");
    }
    if size >= 0 {
        let mut i:usize = 0;

        println!("    ╚⊙ ⊙╝");
        while i < size as usize {
            println!("{}╚═(███)═╝", padding[i % 8]);
            i = i + 1;
        }
    } else {
        let mut i:usize = size.abs() as usize;

        while i > 0 {
            println!("{}╔═(███)═╗", padding[i%8]);
            i = i - 1;
        }
        println!("   ╔⊙ ⊙╗");
    }
}
