mod chain;
mod parser;
mod utils;
mod pdb_parser;
mod inspect;
use utils::{until, pword, tag};
use parser::{seq, take, skip};
use pdb_parser::ATOM;
use chain::{trimr, triml, trim, empty, to_i32, to_u8, to_byte, to_f64, opt, store};

fn parse_pdb() {
    //let line = "ATOM     89  OG  SER A 693      25.623   8.222  86.526  1.00 43.01           O ";
    //let line = "ATOM      1  N   CYS R   1      52.832  45.826  37.193  1.00  0.00";
    let line = "ATOM     23  CG1 VAL R   3      58.836  39.331  40.265  1.00  0.00";
    //let line = "ATOM     90 2HE2 GLN R   7      55.573  32.236  40.457  1.00  0.00";
    let mut atom = ATOM::new();
    atom.parse(&line);    
    println!("Serial {}", atom.serial);
    println!("Name {}", atom.name);
    println!("Alt Loc {}", atom.alt_loc);
    println!("Res Name {}", atom.res_name);
    println!("ICode {}", atom.icode as char);
    println!("X {}", atom.x);
    println!("Y {}", atom.y);
    println!("Z {}", atom.z);
}

fn parse_darxml() {
    let dml = 
"def use core {{
    extern crate glutin;
    extern crate gleam;

    use gleam::gl;
}}

def block main {{
    open {{
        fn main() {
    }}
    close  {{
        }
    }}
}}

use core
main {}";

}

fn main() {
    let test_trim = "  \t\r\n adln \n   \n";
    println!("------------ Test Trim ------------");
    println!("triml: {:?}", utils::triml(test_trim));
    println!("trimr: {:?}", utils::trimr(test_trim));
    println!("trim: {:?}", utils::trim(test_trim));
    println!("------------ Test PDB Parser ------------");
    parse_pdb();
    println!("------------ Test Until ------------");
    println!("{:?}", until(&"abcdefg", &[&"j", &"w"]));
    println!("------------ Test PWord ------------");
    println!("word {:?}", pword(&"d_7_ te2_243 st test"));
    println!("------------ Test Tag ------------");
    println!("tag {:?}", tag(&"d_7_ te2_243 st test", "d_7_ te2_243 st test"));
}
