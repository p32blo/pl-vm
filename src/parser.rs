

use pest::prelude::*;

use instructions::Instruction;

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

        padd = {["padd"]}
        add = {["add"]}
        sub = {["sub"]}
        mul = {["mul"]}
        div = {["div"]}
        mod_ = {["mod"]}
        not = {["not"]}
        inf = {["inf"]}
        infeq = {["infeq"]}
        sup = {["sup"]}

        supeq = {["supeq"]}
        fadd = {["fadd"]}
        fsub = {["fsub"]}
        fmul = {["fmul"]}
        fdiv = {["fdiv"]}
        fcos = {["fcos"]}
        fsin = {["fsin"]}

        finf = {["finf"]}
        finfeq = {["finfeq"]}
        fsup = {["fsup"]}
        fsupeq = {["fsupeq"]}
        concat = {["concat"]}
        equal = {["equal"]}
        atoi = {["atoi"]}
        atof = {["atof"]}

        itof = {["itof"]}
        ftoi = {["ftoi"]}
        stri = {["stri"]}
        strf = {["strf"]}

        pushsp = {["pushsp"]}
        pushfp = {["pushfp"]}
        pushgp = {["pushgp"]}
        loadn = {["loadn"]}
        storen = {["storen"]}
        swap = {["swap"]}

        writei = {["writei"]}
        writef = {["writef"]}
        writes = {["writes"]}
        read = {["read"]}
        call = {["call"]}
        return_ = {["return"]}

        drawpoint = {["drawpoint"]}
        drawline = {["drawline"]}
        drawcircle = {["drawcircle"]}

        cleardrawingarea = {["cleardrawingarea"]}
        opendrawingarea = {["opendrawingarea"]}
        setcolor = {["setcolor"]}
        refresh = {["refresh"]}

        start = {["start"]}
        nop = {["nop"]}
        stop = {["stop"]}
        allocn = {["allocn"]}
        free = {["free"]}
        dupn = {["dupn"]}
        popn = {["popn"]}


        pushi = {["pushi"]}
        pushn = {["pushn"]}
        pushg = {["pushg"]}
        pushl = {["pushl"]}
        load = {["load"]}

        dup = {["dup"]}
        pop = {["pop"]}
        storel = {["storel"]}
        storeg = {["storeg"]}
        alloc = {["alloc"]}

        pushf = {["pushf"]}

        pushs = {["pushs"]}
        err = {["err"]}

        check = {["check"]}

        jump = {["jump"]}
        jz = {["jz"]}
        pusha = {["pusha"]}

        // Grammar Rules
        code = _{ soi ~ instr* ~ eoi }

        instr = {
            ident ~ [":"]
            | instr_atom
            | instr_int ~ integer
            | pushf ~ float
            | ( pushs | err) ~ string
            | check ~ integer ~ [","] ~ integer
            | (jump | jz | pusha) ~ ident
        }
        instr_atom = {
            padd | add | sub | mul | div | mod_ | not | inf | infeq | sup
            | supeq | fadd | fsub | fmul | fdiv | fcos | fsin
            | finf | finfeq | fsup | fsupeq | concat | equal | atoi | atof
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
        whitespace = _{ [" "] | ["\n"] | ["\r"] | ["\t"] }
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

            (_: instr_atom) => self.atom(),
            (_: instr_int, res: int(), &i:integer) => res(i.parse().unwrap()),

            (&a) => panic!(format!("Instruction not found: {}", a)),
        }

        int(&self) -> fn(i32) -> Instruction {
            (_: pushi) => Instruction::Pushi,
            (_: pushn) => Instruction::Pushn,
            (_: pushg) => Instruction::Pushg,
            (_: storeg) => Instruction::Storeg,
            (&a) => panic!(format!("Instruction not found: {}", a)),
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

            (&a) => panic!(format!("Instruction not found: {}", a)),
        }
    }
}

pub fn parse(input: &str) -> Vec<Instruction> {
    let mut parser = Rdp::new(StringInput::new(input));
    parser.code();
    println!("{:?}",
             parser.queue_with_captures()
                 .iter()
                 .map(|x| &x.1)
                 .collect::<Vec<_>>());
    parser.compute()
}