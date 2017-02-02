use std::vec::Vec;
use chain::{Chain, Operation};
use utils;

pub enum ParserType<'d, 'a> {
    /// Stores a Sequence of parsers. Executes until all pass or one failure occurs.
    Sequence(Vec<Parser<'d, 'a>>),
    Take(usize),
    Skip(usize),
    PWord,
    Blank,
}

pub struct Parser<'d, 'c> {
    pub ptype: ParserType<'d, 'c>,
    pub chain: Option<Chain<'c>>,
    parsed: Option<&'d str>,
}

impl<'d, 'c> Parser<'d, 'c> {
    fn new(ptype: ParserType<'d, 'c>) -> Parser<'d, 'c> {
        Parser {
            ptype: ptype,
            chain: None,
            parsed: None,
        }
    }

    pub fn parse(&mut self, buffer: &'d str) -> Result<Option<&'d str>, ParsingError> {
        run_parser(self, buffer)
    }


    pub fn chain(mut self, c: Chain<'c>) -> Self {
        self.chain = Some(c);
        self
    }
}

pub fn seq<'d, 'a>(list_parsers: Vec<Parser<'d, 'a>>) -> Parser<'d, 'a> {
    Parser::new(ParserType::Sequence(list_parsers))
}

pub fn take<'d, 'a>(count: usize) -> Parser<'d, 'a> {
    Parser::new(ParserType::Take(count))
}

pub fn skip<'d, 'a>(count: usize) -> Parser<'d, 'a> {
    Parser::new(ParserType::Skip(count))
}

pub fn pword<'d, 'a>() -> Parser<'d, 'a> {
    Parser::new(ParserType::PWord)
}

pub fn blank<'d, 'a>() -> Parser<'d, 'a> {
    Parser::new(ParserType::Blank)
}

#[derive(Debug)]
enum InvalidParserError {
    // Sequence has no parsers
    SeqEmpty,
    // Not a sequence
    SeqNot,
    // Sequence cannot have chain
    SeqChain,
    // Not a take
    TakeNot,
    // Not a skip
    SkipNot,
    // Not a blank
    BlankNot,
    // Not a pword
    PWordNot,
    // Must take mode then zero bytes
    TakeZero,
    // Or must be last in operation chain
    ChainOrLast,
    // Store must be last in operation chain
    ChainStoreLast,
    // Unexpected error in blank parser
    BlankError,
}

#[derive(Debug)]
enum ParsingError {
    // Trying to parse with an invalid parser
    InvalidParser(InvalidParserError),
    // More bytes are required then are available
    InsufficientData,
    // Index into buffer is not a valid utf8 char boundary
    InvalidIndex,
    // Can't have chain after skip parser
    ChainAfterSkip,
    // An error ocorred in the chain
    ChainError(ChainingError),
    // Buffer is empty
    EmptyBuffer,
    // pword is invalid (must start with alpha or '_')
    InvalidPWord,
    // An Unexpected Error ocorred, this should not happen
    UnexpectedError,
}

fn run_parser<'d, 'a>(parser: &mut Parser<'d, 'a>, buffer: &'d str) -> Result<Option<&'d str>, ParsingError> {
    match parser.ptype {                
        ParserType::Sequence(_) => {
            let result = run_seq(parser, buffer);
            match result {
                Ok(()) => return Ok(None),
                Err(pe) => return Err(pe),
            }
        }
        ParserType::Take(_) => {
            let result = run_take(parser, buffer);
            match result {
                Ok(rest) => return Ok(Some(rest)),
                Err(pe) => return Err(pe),
            }
        }
        ParserType::Skip(_) => {
            let result = run_skip(parser, buffer);
            match result {
                Ok(rest) => return Ok(Some(rest)),
                Err(pe) => return Err(pe),
            }
        }
        ParserType::PWord => {
            let result = run_pword(parser, buffer);
            match result {
                Ok(rest) => return Ok(Some(rest)),
                Err(pe) => return Err(pe),
            }
        }
        ParserType::Blank => {
            let result = run_blank(parser, buffer);
            match result {
                Ok(rest) => return Ok(Some(rest)),
                Err(pe) => return Err(pe),
            }
        }
    }
}

fn run_seq<'d, 'a>(seq_parser: &mut Parser<'d, 'a>, buffer: &'d str) -> Result<(), ParsingError> {
    match seq_parser.ptype {
        ParserType::Sequence(ref mut list) => {
            let mut data = buffer;

            for parser in list.iter_mut() {
                let result = run_parser(parser, data);
                match result {
                    Err(e) => return Err(e),
                    Ok(Some(rest)) => data = rest,
                    Ok(None) => ()
                }
            }
        }
        _ => return Err(ParsingError::InvalidParser(InvalidParserError::SeqNot)),
    }
    Ok(())
}

fn run_blank<'d, 'a>(parser: &mut Parser<'d, 'a>, buffer: &'d str) -> Result<&'d str, ParsingError> {
    if let ParserType::PWord = parser.ptype {
        let ut = utils::blank(buffer);
        match ut {
            Ok(utils::Split {left, right}) => {
                parser.parsed = Some(left);
                let chain_result = run_chain(parser);
                match chain_result {
                    Err(ce) => return Err(ParsingError::ChainError(ce)),
                    Ok(_) => return Ok(right),
                }
            }
            _ => return Err(ParsingError::InvalidParser(InvalidParserError::BlankError)),
        }
    }
    Err(ParsingError::InvalidParser(InvalidParserError::BlankNot))
}

fn run_pword<'d, 'a>(parser: &mut Parser<'d, 'a>, buffer: &'d str) -> Result<&'d str, ParsingError> {
    if let ParserType::PWord = parser.ptype {
        let ut = utils::pword(buffer);
        match ut {
            Ok(utils::Split {left, right}) => {
                parser.parsed = Some(left);
                let chain_result = run_chain(parser);
                match chain_result {
                    Err(ce) => return Err(ParsingError::ChainError(ce)),
                    Ok(_) => return Ok(right),
                }
            }
            Err(utils::Error::EmptyBuffer) => return Err(ParsingError::EmptyBuffer),
            Err(utils::Error::InvalidPWord) => return Err(ParsingError::InvalidPWord),
            Err(_) => return Err(ParsingError::UnexpectedError)
            
        }
    }
    Err(ParsingError::InvalidParser(InvalidParserError::PWordNot))
}

fn run_skip<'d, 'a>(parser: &mut Parser<'d, 'a>, buffer: &'d str) -> Result<&'d str, ParsingError> {
    if let ParserType::Skip(c) = parser.ptype {
        let ut = utils::skip(buffer, c);
        match ut {
            Ok(utils::Split { left, right }) => {
                parser.parsed = Some(left);
                if let Some(_) = parser.chain {
                    return Err(ParsingError::ChainAfterSkip);
                } else {
                    return Ok(right);
                }
            }
            Err(utils::Error::InsufficientBuffer) => return Err(ParsingError::InsufficientData),
            Err(utils::Error::InvalidCharBoundary) => return Err(ParsingError::InvalidIndex),
            Err(_) => return Err(ParsingError::UnexpectedError)
        }
    }
    Err(ParsingError::InvalidParser(InvalidParserError::SkipNot))
}

fn run_take<'d, 'a>(parser: &mut Parser<'d, 'a>, buffer: &'d str) -> Result<&'d str, ParsingError> {

    if let ParserType::Take(c) = parser.ptype {
        let ut = utils::take(buffer, c);
        match ut {
            Ok(utils::Split { left, right }) => {
                parser.parsed = Some(left);
                let chain_result = run_chain(parser);
                match chain_result {
                    Err(ce) => return Err(ParsingError::ChainError(ce)),
                    Ok(_) => return Ok(right),
                }
            }
            Err(utils::Error::InsufficientBuffer) => return Err(ParsingError::InsufficientData),
            Err(utils::Error::InvalidCharBoundary) => return Err(ParsingError::InvalidIndex),
            Err(_) => return Err(ParsingError::UnexpectedError)
        }
    }
    Err(ParsingError::InvalidParser(InvalidParserError::TakeNot))
}

#[derive(Debug)]
enum ChainingError {
    // Attepting to aply chain to non-existant parsed data
    NoParsedData,
    // Previous chain operation failed to produce data
    PreviousOpFailed,
    // Equality test failed
    EqFailed,
    // Operation expected a &str found i32
    ExpectedFound(ParsingDataTypes, ParsingDataTypes),
    // Got a type error when it shouldn't. If this
    // comes up we probabily got a bug, maybe I should just
    // crash
    InvalidTypeError,
    // Tryed to convert to i32, but failed
    InvalidI32,
    // Tryed to convert to u8, but failed
    InvalidU8,
    // Tryed to convert to f64, but failed
    InvalidF64,
    // Cannot chain Store after Opt
    StoreAfterOpt,
    // Not implemented yet
    NotImplemented,
    // to_byte can only be used in a length 1 string
    BiggerThenByte,
    // Unexpected erro while trimming string
    TrimError,
}

#[derive(Debug)]
enum LastChainData<'a> {
    Str(&'a str),
    I32(i32),
    F64(f64),
    U8(u8),
    Byte(u8),
    Opt,
    NotImplemented
}
#[derive(Debug)]
enum ParsingDataTypes {
    Str,
    I32,
    F64,
    U8,
    Byte,
}

impl<'a> ParsingDataTypes {
    fn chain_type_error(self,
                        found: &LastChainData<'a>)
                        -> Result<LastChainData<'a>, ChainingError> {
        match found {
            &LastChainData::Str(_) => Err(ChainingError::ExpectedFound(ParsingDataTypes::Str, self)),
            &LastChainData::I32(_) => Err(ChainingError::ExpectedFound(ParsingDataTypes::I32, self)),
            &LastChainData::F64(_) => Err(ChainingError::ExpectedFound(ParsingDataTypes::F64, self)),
            &LastChainData::U8(_) => Err(ChainingError::ExpectedFound(ParsingDataTypes::U8, self)),
            &LastChainData::Byte(_) => Err(ChainingError::ExpectedFound(ParsingDataTypes::Byte, self)),
            &LastChainData::NotImplemented => Err(ChainingError::NotImplemented),
            &LastChainData::Opt => Err(ChainingError::NotImplemented),
        }
    }
}
fn run_operation<'c, 'd>(op: &'c mut Operation,
                         data: LastChainData<'d>)
                         -> Result<LastChainData<'d>, ChainingError> {
    match op {
        &mut Operation::Trim => {
            match data {
                LastChainData::Str(ref s) => {
                    let result = utils::trim(s);
                    if let Ok(rest) = result {
                        Ok(LastChainData::Str(rest))
                    } else {
                        Err(ChainingError::TrimError)
                    }
                }
                _ => ParsingDataTypes::Str.chain_type_error(&data),
            }
        } 
        &mut Operation::TrimR => {
            match data {
                LastChainData::Str(ref s) => {
                    let result = utils::trimr(s);
                    if let Ok(utils::Split {left, ..}) = result {
                        Ok(LastChainData::Str(left))
                    } else {
                        Err(ChainingError::TrimError)
                    }
                }
                _ => ParsingDataTypes::Str.chain_type_error(&data),
            }
        }
        &mut Operation::TrimL => {
            match data {
                LastChainData::Str(ref s) => {
                    let result = utils::triml(s);
                    if let Ok(utils::Split {right, ..}) = result {
                        Ok(LastChainData::Str(right))
                    } else {
                        Err(ChainingError::TrimError)
                    }
                }
                _ => ParsingDataTypes::Str.chain_type_error(&data),
            }
        }
        &mut Operation::EqualString(ref e) => {
            match data {
                LastChainData::Str(ref s) if e == s => Ok(LastChainData::Str(s)),
                LastChainData::Str(_) => Err(ChainingError::EqFailed),
                _ => ParsingDataTypes::Str.chain_type_error(&data),
            }
        }
        &mut Operation::ToI32 => {
            match data {
                LastChainData::Str(ref s) => {
                    let result = s.parse::<i32>();
                    match result {
                        Err(_) => Err(ChainingError::InvalidI32),
                        Ok(i) => Ok(LastChainData::I32(i)),
                    }
                } 
                _ => ParsingDataTypes::I32.chain_type_error(&data),
            }
        }
        &mut Operation::ToF64 => {
            match data {
                LastChainData::Str(ref s) => {
                    let result = s.parse::<f64>();
                    match result {
                        Err(_) => Err(ChainingError::InvalidF64),
                        Ok(f) => Ok(LastChainData::F64(f)),
                    }
                } 
                _ => ParsingDataTypes::F64.chain_type_error(&data),
            }
        }
        &mut Operation::ToU8 => {
            match data {
                LastChainData::Str(ref s) => {
                    let result = s.parse::<u8>();
                    match result {
                        Err(_) => Err(ChainingError::InvalidU8),
                        Ok(i) => Ok(LastChainData::U8(i)),
                    }
                } 
                _ => ParsingDataTypes::U8.chain_type_error(&data),
            }
        }
        &mut Operation::ToByte => {
            match data {
                LastChainData::Str(ref s) => {
                    if s.len() == 1 {
                        Ok(LastChainData::Byte(s.as_bytes()[0]))
                    } else {
                        Err(ChainingError::BiggerThenByte)
                    }
                } 
                _ => ParsingDataTypes::U8.chain_type_error(&data),
            }
        }
        &mut Operation::Store(ref mut field_ref) => {
            match data {
                LastChainData::Str(ref s) => {
                    match field_ref.downcast_mut::<String>() {
                        Some(as_string) => {
                            as_string.push_str(s);
                            Ok(LastChainData::Str(s))
                        }
                        None => ParsingDataTypes::Str.chain_type_error(&data),
                    }
                }
                LastChainData::I32(i) => {
                    match field_ref.downcast_mut::<i32>() {
                        Some(as_i32) => {
                            *as_i32 = i;
                            Ok(LastChainData::I32(i))
                        }
                        None => ParsingDataTypes::I32.chain_type_error(&data),
                    }
                }
                LastChainData::F64(i) => {
                    match field_ref.downcast_mut::<f64>() {
                        Some(as_f64) => {
                            *as_f64 = i;
                            Ok(LastChainData::F64(i))
                        }
                        None => ParsingDataTypes::F64.chain_type_error(&data),
                    }
                }
                LastChainData::U8(u) => {
                    match field_ref.downcast_mut::<u8>() {
                        Some(as_u8) => {
                            *as_u8 = u;
                            Ok(LastChainData::U8(u))
                        }
                        None => ParsingDataTypes::U8.chain_type_error(&data),
                    }
                }
                LastChainData::Byte(b) => {
                    match field_ref.downcast_mut::<u8>() {
                        Some(as_u8) => {
                            *as_u8 = b;
                            Ok(LastChainData::Byte(b))
                        }
                        None => ParsingDataTypes::Byte.chain_type_error(&data),
                    }
                }
                LastChainData::Opt => Err(ChainingError::StoreAfterOpt),
                LastChainData::NotImplemented => Err(ChainingError::NotImplemented),
            }
        }
        // TODO: do something about invalid parsers inside Opt
        &mut Operation::Opt(ref mut chain) => {
            let mut opt_data = data;
            for opt_op in chain.olist.iter_mut() {
                let result = run_operation(opt_op, opt_data);
                match result {
                    Ok(nd) => opt_data = nd, 
                    _ => break,
                }
            }
            Ok(LastChainData::Opt)
        }
        _ => {
            Ok(LastChainData::NotImplemented)
        }
    }
}

fn run_chain<'d, 'a>(parser: &mut Parser<'d, 'a>) -> Result<(), ChainingError> {
    if let Some(ref mut chain) = parser.chain {
        if let None = parser.parsed {
            return Err(ChainingError::NoParsedData);
        }
        let mut data = LastChainData::Str(parser.parsed.unwrap());

        for op in chain.olist.iter_mut() {
            let result = run_operation(op, data);
            match result {
                Ok(nd) => data = nd, 
                Err(e) => return Err(e),
            }
        }
    }
    Ok(())
}