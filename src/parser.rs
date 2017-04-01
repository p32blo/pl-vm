

use pest::prelude::*;

use instructions::Instruction;

use errors::*;

impl_rdp! {
    grammar! {
        // Lexical Rules
        digit = _{ ['0'..'9'] }
        alpha = _{ ['A'..'Z'] | ['a'..'z'] }
        ident = @{ ( alpha | ["_"] ) ~ ( alpha | digit | ["_"] | ["'"] )* }

        integer = @{ ["-"]? ~ digit+}
        float = @{ ["-"]? ~ digit+ ~ (["."] ~ digit*)? ~ ((["e"]|["E"]) ~ (["+"]|["-"])? ~ digit+)? }

        string = @{ ["\""] ~ inner_string ~ ["\""] }
        inner_string = { (["\\\""]|!["\""] ~ any)* }

        padd = {[i"padd"]}
        add = {[i"add"]}
        sub = {[i"sub"]}
        mul = {[i"mul"]}
        div = {[i"div"]}
        mod_ = {[i"mod"]}
        not = {[i"not"]}
        inf = {[i"inf"]}
        infeq = {[i"infeq"]}
        sup = {[i"sup"]}

        supeq = {[i"supeq"]}
        fadd = {[i"fadd"]}
        fsub = {[i"fsub"]}
        fmul = {[i"fmul"]}
        fdiv = {[i"fdiv"]}
        fcos = {[i"fcos"]}
        fsin = {[i"fsin"]}

        finf = {[i"finf"]}
        finfeq = {[i"finfeq"]}
        fsup = {[i"fsup"]}
        fsupeq = {[i"fsupeq"]}
        concat = {[i"concat"]}
        equal = {[i"equal"]}
        atoi = {[i"atoi"]}
        atof = {[i"atof"]}

        itof = {[i"itof"]}
        ftoi = {[i"ftoi"]}
        stri = {[i"stri"]}
        strf = {[i"strf"]}

        pushsp = {[i"pushsp"]}
        pushfp = {[i"pushfp"]}
        pushgp = {[i"pushgp"]}
        loadn = {[i"loadn"]}
        storen = {[i"storen"]}
        swap = {[i"swap"]}

        writei = {[i"writei"]}
        writef = {[i"writef"]}
        writes = {[i"writes"]}
        read = {[i"read"]}
        call = {[i"call"]}
        return_ = {[i"return"]}

        drawpoint = {[i"drawpoint"]}
        drawline = {[i"drawline"]}
        drawcircle = {[i"drawcircle"]}

        cleardrawingarea = {[i"cleardrawingarea"]}
        opendrawingarea = {[i"opendrawingarea"]}
        setcolor = {[i"setcolor"]}
        refresh = {[i"refresh"]}

        start = {[i"start"]}
        nop = {[i"nop"]}
        stop = {[i"stop"]}
        allocn = {[i"allocn"]}
        free = {[i"free"]}
        dupn = {[i"dupn"]}
        popn = {[i"popn"]}


        pushi = {[i"pushi"]}
        pushn = {[i"pushn"]}
        pushg = {[i"pushg"]}
        pushl = {[i"pushl"]}
        load = {[i"load"]}

        dup = {[i"dup"]}
        pop = {[i"pop"]}
        storel = {[i"storel"]}
        storeg = {[i"storeg"]}
        alloc = {[i"alloc"]}

        pushf = {[i"pushf"]}

        pushs = {[i"pushs"]}
        err = {[i"err"]}

        check = {[i"check"]}

        jump = {[i"jump"]}
        jz = {[i"jz"]}
        pusha = {[i"pusha"]}

        sp = _{ ( [" "] | ["\t"] ) }

        // Grammar Rules
        code = _{ soi ~ (instr)* ~ eoi }

        instr = @{
            ident ~ sp* ~ [":"]
            | instr_atom
            | instr_int ~ sp+ ~ integer
            | pushf ~ sp+ ~ float
            | ( pushs | err) ~ sp+ ~ string
            | check ~ sp+ ~ integer ~ sp* ~ [","] ~ sp* ~ integer
            | (jump | jz | pusha) ~ sp+ ~ ident
        }
        instr_atom = {
            padd | add | sub | mul | div | mod_ | not | infeq | inf | supeq
            | sup | fadd | fsub | fmul | fdiv | fcos | fsin
            | finfeq | finf | fsupeq | fsup | concat | equal | atoi | atof
            | itof | ftoi | stri | strf
            | pushsp | pushfp | pushgp | loadn | storen | swap
            | writei | writef | writes | read | call | return_
            | drawpoint | drawline | drawcircle
            | cleardrawingarea | opendrawingarea | setcolor | refresh
            | start | nop | stop | allocn | free | dupn | popn
        }
        instr_int = {
            pushi | pushn | pushg | pushl | load
            | dup | pop | storel | storeg | alloc
        }
        comment = _{ ["//"] ~ (!["\n"] ~ any)* ~ ["\n"] }
        whitespace = _{ sp | ["\n"] | ["\r"]}
    }

    process! {
        compute(&self) -> Vec<Instruction> {
            (_: instr, head: instruction(), mut tail: compute()) => {
                tail.insert(0, head);
                tail    
            },
            () => {
                Vec::new()
            }
        }

        instruction(&self) -> Instruction {
            (&id: ident) => Instruction::Label(id.to_string()),

            (_: pushs, _: string, &s: inner_string) => Instruction::Pushs(s.to_string()),
            (_: err, _: string, &s: inner_string) => Instruction::Err(s.to_string()),

            (_:jump, &id:ident) => Instruction::Jump(id.to_string()),
            (_:jz, &id:ident) => Instruction::Jz(id.to_string()),
            (_:pusha, &id:ident) => Instruction::Pusha(id.to_string()),

            (_: instr_atom, res: atom()) => res,
            (_: instr_int, res: int()) => res,
        }

        int(&self) -> Instruction {
            (_: pushg, &i: integer) => Instruction::Pushg(i.parse().unwrap()),
            (_: storeg, &i: integer) => Instruction::Storeg(i.parse().unwrap()),
            (_: pushi, &i: integer) => Instruction::Pushi(i.parse().unwrap()),
            (_: pushn, &i: integer) => Instruction::Pushn(i.parse().unwrap()),
        }

        atom(&self) -> Instruction {
            (_: padd) => Instruction::Padd,
            (_: add) => Instruction::Add,
            (_: mul) => Instruction::Mul,
            (_: div) => Instruction::Div,
            (_: mod_) => Instruction::Mod,
            (_: inf) => Instruction::Inf,
            (_: infeq) => Instruction::Infeq,
            (_: sup) => Instruction::Sup,

            (_: supeq) => Instruction::Supeq,
            
            (_: equal) => Instruction::Equal,
            (_: atoi) => Instruction::Atoi,
           
            (_: pushgp) => Instruction::Pushgp,
            (_: loadn) => Instruction::Loadn,
            (_: storen) => Instruction::Storen,
           
            (_: writei) => Instruction::Writei,
            (_: writes) => Instruction::Writes,
            (_: read) => Instruction::Read,
            (_: call) => Instruction::Call,
            (_: return_) => Instruction::Return,

            (_: start) => Instruction::Start,
            (_: nop) => Instruction::Nop,
            (_: stop) => Instruction::Stop,
        }
    }
}

pub fn parse(input: &str) -> Result<Vec<Instruction>> {

    let mut parser = Rdp::new(StringInput::new(input));

    parser.code();

    if !parser.end() {
        let (r, pos) = parser.expected();
        let (line, col) = parser.input().line_col(pos);
        bail!(format!("line({}), col({:?}) => expected rules: {:?}", line, col, r));
    }

    Ok(parser.compute())
}