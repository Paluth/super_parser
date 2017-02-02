use parser::{Parser, ParserType};
use chain::{Operation, Chain};

pub fn print<'a, 'b>(parser: &Parser<'a, 'b>) {
    print_aux(parser, Indent::new())
}

#[derive(Clone)]
enum Indentation {
    Space(usize),
    BlockEnd(usize),
    Block(usize),
    Word(String),
}

#[derive(Clone)]
struct Indent {
    indents: Vec<Indentation>,
}

impl Indent {
    fn new() -> Indent {
        Indent { indents: vec![] }
    }

    fn push_clone(&self, indent: Indentation) -> Indent {
        let mut ni = Indent::new();
        for i in self.indents.iter() {
            match i {
                &Indentation::BlockEnd(spaces) => {
                    if let Indentation::BlockEnd(_) = indent {
                        ni.indents.push(Indentation::Block(spaces));
                    } else {
                        ni.indents.push(Indentation::BlockEnd(spaces));
                    }
                }
                &Indentation::Space(spaces) => ni.indents.push(Indentation::Space(spaces)),
                &Indentation::Block(spaces) => ni.indents.push(Indentation::Block(spaces)),
                &Indentation::Word(ref string) => {
                    ni.indents.push(Indentation::Word(string.clone()))
                }
            }
        }
        ni.indents.push(indent);
        ni
    }

    fn print(&self) {
        for i in self.indents.iter() {
            match i {
                &Indentation::Space(c) => {
                    Indent::print_spaces(c);
                }
                &Indentation::BlockEnd(c) => {
                    Indent::print_spaces(c);
                    print!(" |- ");

                }
                &Indentation::Block(c) => {
                    Indent::print_spaces(c);
                    print!(" |  ");
                }
                &Indentation::Word(ref s) => print!("'{}'", s),
            }
        }
    }

    fn print_spaces(count: usize) {
        for _ in 0..count {
            print!(" ");
        }
    }
}

fn print_aux<'a, 'b>(parser: &Parser<'a, 'b>, indent: Indent) {
    match parser.ptype {
        ParserType::Sequence(ref list) => {
            print!("seq -|- ");
            let new_ident = indent.push_clone(Indentation::BlockEnd(4));
            for i in 0..list.len() {
                if i > 0 {
                    new_ident.print();
                }
                print_aux(&list[i], new_ident.clone());
            }
        }
        ParserType::Take(count) => {
            let ftake = format!("take {}", count);
            print!("{}", ftake);
            let new_ident = indent.push_clone(Indentation::Space(ftake.len()));
            if let Some(ref c) = parser.chain {
                print_chain(c, new_ident, false, false);
            } else {
                println!();
            }
        }
        ParserType::Skip(count) => {
            let fskip = format!("skip {}", count);
            print!("{}", fskip);
            let new_ident = indent.push_clone(Indentation::Space(fskip.len()));
            println!();
        }
        ParserType::PWord => {
            let fpword = "pword";
            print!("{}", fpword);
            let new_ident = indent.push_clone(Indentation::Space(fpword.len()));
        }
        ParserType::Blank => {
            let fblank = "blank";
            print!("{}", fblank);
            let new_ident = indent.push_clone(Indentation::Space(fblank.len()));
        }
    }
}

fn format_op(op: &Operation) -> String {
    let mut formated = String::new();
    match *op {
        Operation::TrimR => formated.push_str("trimr"),
        Operation::TrimL => formated.push_str("triml"),
        Operation::EqualString(ref s) => formated.push_str(format!("eq {}", s).as_str()),
        Operation::NotEqualString(ref s) => formated.push_str(format!("neq {}", s).as_str()),
        Operation::Trim => formated.push_str("trim"),
        Operation::Store(_) => formated.push_str("store"),
        Operation::Empty => formated.push_str("empty"),
        Operation::Or(_) => formated.push_str("or -|- "),
        Operation::ToI32 => formated.push_str("to_i32"),
        Operation::ToF64 => formated.push_str("to_f64"),
        Operation::ToU8 => formated.push_str("to_u8"),
        Operation::ToByte => formated.push_str("to_byte"),
        Operation::Opt(_) => formated.push_str("opt <"),
    }
    formated
}

fn print_chain<'a>(chain: &Chain<'a>, indent: Indent, chain_arrow: bool, opt : bool) {
    let mut sc = chain_arrow;
    let mut ni = indent.clone();
    let mut sub = false;
    for i in 0..chain.olist.len() {
        if !sc {
            print!(" -> ");
            ni = ni.push_clone(Indentation::Space(4));
        } else {
            sc = false;
        }
        let formated = format_op(&chain.olist[i]);
        print!("{}", formated);
        let spaces = formated.len();

        if let Operation::Or(ref chains) = chain.olist[i] {
            sub = true;
            ni = ni.push_clone(Indentation::BlockEnd(3));
            for i in 0..chains.len() {
                if i == 0 {
                    print_chain(&chains[i], ni.clone(), true, false);
                } else {
                    ni.print();
                    print_chain(&chains[i], ni.clone(), true, false);
                }
            }
        } else if let Operation::Opt(ref chain) = chain.olist[i] {
            sub = true;
            print_chain(chain, ni.clone(), true, true);
        } else {
            ni = ni.push_clone(Indentation::Space(spaces));
        }

        
    }
    if !sub {
        if opt {
            println!(">");
        } else {
            println!();
        }
    }
}