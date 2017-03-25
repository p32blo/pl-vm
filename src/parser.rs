
use nom;

use std::str::FromStr;

use instructions::Instruction;

named!(digit1<&str, &str>, flat_map!(take!(1), nom::digit));
named!(alpha1<&str, &str>, flat_map!(take!(1), nom::alpha));
named!(ident1<&str,&str>, alt!(tag!("_") | alpha1));
named!(ident2<&str,Vec<&str>>, many0!(alt!(digit1 | alpha1 | tag!("_") | tag!("'"))));

named!(ident<&str, String>,
    do_parse!(
        id1: ident1 >>
        id2: ident2 >>
        (id1.to_string() + &id2.join(""))
    )
);

named!(string<&str, &str>,
    delimited!(
        tag!("\""),
        take_until!("\""),
        tag!("\"")
    )
);

named!(comment<&str, &str>,
    delimited!(
        ws!(tag!("//")),
        take_until!("\n"),
        opt!(nom::eol)
    )
);

named!(pub code<&str, Vec<Instruction>>,
    terminated!(
        many1!(
            delimited!(
                many0!(comment),
                ws!(instr),
                many0!(comment)
            )
        ), 
        eof!()
    )
);

named!(instr<&str, Instruction>,
    alt!( instr_atom
        | do_parse!(
            id: ident >>
            ws!(tag!(":")) >>
            (Instruction::Label(id))
        )
        | do_parse!(
            ins: instr_int >>
            arg: ws!(map_res!(nom::digit, FromStr::from_str)) >>
            (ins(arg))
        )
        | do_parse!(
            ins: instr_uint >>
            arg: ws!(map_res!(nom::digit, FromStr::from_str)) >>
            (ins(arg))
        )
        // | do_parse!(
        //     tag!("pushf") >>
        //     fl: ws!(nom::float_s) >>
        //     (Instruction::Pushf(fl))
        // )
        | do_parse!(
            ins: instr_str >>
            arg: ws!(string) >>
            (ins(arg.to_string()))
        )
        // | do_parse!(
        //     tag!("check") >>
        //     st: ws!(map_res!(nom::digit, FromStr::from_str)) >>
        //     ws!(tag!(",")) >>
        //     nd: ws!(map_res!(nom::digit, FromStr::from_str)) >>
        //     (Instruction::Check(st, nd))
        // )
        | do_parse!(
            ins: instr_ident >>
            arg: ws!(ident) >>
            (ins(arg))
        )
    )
);

named!(instr_atom<&str, Instruction>,
    alt!(
        tag!("add") => { |_| Instruction::Add }
        //| tag!("sub") => { |_| Instruction::Sub }
        | tag!("mul") => { |_| Instruction::Mul }
        | tag!("div") => { |_| Instruction::Div }
        | tag!("mod") => { |_| Instruction::Mod }
        //| tag!("not") => { |_| Instruction::Not }
        | tag!("infeq") => { |_| Instruction::Infeq }
        | tag!("inf") => { |_| Instruction::Inf }

        | tag!("supeq") => { |_| Instruction::Supeq }
        | tag!("sup") => { |_| Instruction::Sup }
        //| tag!("fadd") => { |_| Instruction::FAdd }
        //| tag!("fsub") => { |_| Instruction::FSub }
        //| tag!("fmul") => { |_| Instruction::FMull }
        //| tag!("fdiv") | tag!("fcos") | tag!("fsin") |
        // tag!("finf") | tag!("finfeq") | tag!("fsup")
        //| tag!("fsupeq") | tag!("concat")
        | tag!("equal") => { |_| Instruction::Equal }
        | tag!("atoi") => { |_| Instruction::Atoi }
        //| tag!("atof") |
        //tag!("itof") | tag!("ftoi") | tag!("stri") | tag!("strf") |
        | tag!("padd") => {|_| Instruction::Padd }
        //tag!("pushsp") | tag!("pushfp")
        | tag!("pushgp") => {|_| Instruction::Pushgp}
        | tag!("loadn") => { |_| Instruction::Loadn }
        | tag!("storen") => { |_| Instruction::Storen }
        // | tag!("swap") |
        | tag!("writei") => { |_| Instruction::Writei }
        // | tag!("writef")
        | tag!("writes") => { |_| Instruction::Writes }
        | tag!("read") => { |_| Instruction::Read }
        | tag!("call") => { |_| Instruction::Call }
        | tag!("return") => { |_| Instruction::Return }
        // |tag!("drawpoint") | tag!("drawline") | tag!("drawcircle") |
        //tag!("cleardrawingarea") | tag!("opendrawingarea") | tag!("setcolor") | tag!("refresh") |
        | tag!("start") => { |_| Instruction::Start}
        | tag!("nop") =>  { |_| Instruction::Nop}
        | tag!("stop") => { |_| Instruction::Stop }
        // | tag!("allocn") | tag!("free") | tag!("dupn") | tag!("popn")
        //| tag!("pushl") | tag!("load") |
        //tag!("dup") | tag!("pop") | tag!("storel") | tag!("storeg") | tag!("alloc")
    )
);

named!(instr_str<&str, fn(String) -> Instruction>,
    alt!(
        tag!("pushs") => { |_| Instruction::Pushs }
        | tag!("err") => { |_| Instruction::Err }
    )
);

named!(instr_ident<&str, fn(String) -> Instruction>,
    alt!(
        tag!("jump") => { |_| Instruction::Jump }
        | tag!("jz") => { |_| Instruction::Jz }
        | tag!("pusha") => { |_| Instruction::Pusha }
    )
);

named!(instr_uint<&str, fn(usize) -> Instruction>,
    alt!(
         tag!("pushg") => {|_|Instruction::Pushg}
//        | tag!("pushl") => { |_| Instruction::pushl }
//        | tag!("load") => { |_| Instruction::Load }

//        | tag!("dup") => { |_| Instruction::Dup }
//        | tag!("pop") => { |_| Instruction::Pop }
//        | tag!("storel") => { |_| Instruction::Storel }
        | tag!("storeg") => {|_|Instruction::Storeg}
//        | tag!("alloc") => { |_| Instruction::Alloc }
    )
);

named!(instr_int<&str, fn(i32) -> Instruction>,
    alt!(
        tag!("pushi") => {|_|Instruction::Pushi }
        | tag!("pushn") => {|_|Instruction::Pushn }
//        | tag!("pushl") => { |_| Instruction::pushl }
//        | tag!("load") => { |_| Instruction::Load }

//        | tag!("dup") => { |_| Instruction::Dup }
//        | tag!("pop") => { |_| Instruction::Pop }
//        | tag!("storel") => { |_| Instruction::Storel }
//        | tag!("alloc") => { |_| Instruction::Alloc }
    )
);
