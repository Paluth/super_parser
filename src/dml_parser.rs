use parser::{Parser, word, blank, tag};

struct DefUse {
    name: String,
    data: String,
}

impl DefUse {
    pub fn parse_def_use(du: &mut DefUse) -> Parser {
        seq([
            word().store(du.name),
            blank(),
            tag("{{"),
            //until("}}", ["\}}"]).store(du.data)
        ])
    }
}
// fn parse_def() {
//     let def = 
//         or([
//             seq([
//                 tag("def"),
//                 blank(),
//                 or([
//                     seq([tag("use"), blank(), parse_def_use()]),
//                     seq([tag("block"), blank(), parse_def_block()]),
//                 ])
//             ])
//         ]);
//     seq([
//         blank(),
        
//     ])
// }