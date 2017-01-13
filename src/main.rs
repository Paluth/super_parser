mod chain;
mod parser;
mod utils;
mod pdb_parser;
mod inspect;

use parser::{seq, take, skip};
use pdb_parser::ATOM;
use chain::{trimr, triml, trim, empty, to_i32, to_u8, to_byte, to_f64, opt, store};

fn main() {
    //let line = "ATOM     89  OG  SER A 693      25.623   8.222  86.526  1.00 43.01           O ";
    //let line = "ATOM      1  N   CYS R   1      52.832  45.826  37.193  1.00  0.00";
    let line = "ATOM     23  CG1 VAL R   3      58.836  39.331  40.265  1.00  0.00";
    //let line = "ATOM     90 2HE2 GLN R   7      55.573  32.236  40.457  1.00  0.00";
    let mut atom = ATOM::new();
    {
        let mut parser = seq(vec![    
            take(6).chain(trimr().eq_str("ATOM")),
            take(5).chain(trim().to_i32().store(&mut atom.serial)),
            skip(1),
            take(4).chain(trim().store(&mut atom.name)),
            take(1).chain(opt(to_u8().store(&mut atom.alt_loc))),
            take(3).chain(store(&mut atom.res_name)),
            skip(1),
            take(1).chain(to_byte().store(&mut atom.icode)),
            skip(8),
            take(8).chain(trim().to_f64().store(&mut atom.x)),
            take(8).chain(trim().to_f64().store(&mut atom.y)),
            take(8).chain(trim().to_f64().store(&mut atom.z)),
        ]);

        inspect::print(&parser);
        let result = parser.parse(&line);

        match result {
            Ok(_) => println!("Parser succeeded!"),
            Err(e) => println!("Parser failed: {:?}", e),
        }
    }
    println!("Serial {}", atom.serial);
    println!("Name {}", atom.name);
    println!("Alt Loc {}", atom.alt_loc);
    println!("Res Name {}", atom.res_name);
    println!("ICode {}", atom.icode as char);
    println!("X {}", atom.x);
    println!("Y {}", atom.y);
    println!("Z {}", atom.z);
}
