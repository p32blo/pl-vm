

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
        comment = _{ ["//"] ~ (!["\n"] ~ any)* ~ (["\n"] | eoi) }
        whitespace = _{ sp | ["\n"] | ["\r"] }
    }

    process! {

        compute(&self) -> Result<Vec<Instruction>> {
            (a: instr, head: instruction(), tail: compute()) => {
                let mut t = tail?;
                let h = head.chain_err(|| {
                    let i = self.input();
                    let (line, col) = i.line_col(a.start);
                    format!("Instruction '{}' at line({}), col({:?})", i.slice(a.start, a.end), line, col)
                })?;
                t.insert(0, h);
                Ok(t)
            },
            () => {
                Ok(Vec::new())
            }
        }

        instruction(&self) -> Result<Instruction> {
            (&id: ident) => Ok(Instruction::Label(id.to_string())),

            (_: pushs, _: string, &s: inner_string) => Ok(Instruction::Pushs(s.to_string())),
            (_: err, _: string, &s: inner_string) => Ok(Instruction::Err(s.to_string())),

            (_:jump, &id: ident) => Ok(Instruction::Jump(id.to_string())),
            (_:jz, &id: ident) => Ok(Instruction::Jz(id.to_string())),
            (_:pusha, &id: ident) => Ok(Instruction::Pusha(id.to_string())),

            (_: instr_atom, res: atom()) => res,
            (_: instr_int, res: int()) => res,
            () => Err("Failed to parse Instruction".into())
        }

        int(&self) -> Result<Instruction> {
            (_: pushg, &i: integer) => Ok(Instruction::Pushg(i.parse().chain_err(|| "value is not a positive integer")?)),
            (_: storeg, &i: integer) => Ok(Instruction::Storeg(i.parse().chain_err(|| "value is not a positive integer")?)),
            (_: pushi, &i: integer) => Ok(Instruction::Pushi(i.parse().chain_err(|| "value is not a integer")?)),
            (_: pushn, &i: integer) => Ok(Instruction::Pushn(i.parse().chain_err(|| "value is not a integer")?)),
        }

        atom(&self) -> Result<Instruction> {
            (_: padd) => Ok(Instruction::Padd),
            (_: add) => Ok(Instruction::Add),
            (_: mul) => Ok(Instruction::Mul),
            (_: div) => Ok(Instruction::Div),
            (_: mod_) => Ok(Instruction::Mod),
            (_: inf) => Ok(Instruction::Inf),
            (_: infeq) => Ok(Instruction::Infeq),
            (_: sup) => Ok(Instruction::Sup),

            (_: supeq) => Ok(Instruction::Supeq),

            (_: equal) => Ok(Instruction::Equal),
            (_: atoi) => Ok(Instruction::Atoi),

            (_: pushgp) => Ok(Instruction::Pushgp),
            (_: loadn) => Ok(Instruction::Loadn),
            (_: storen) => Ok(Instruction::Storen),

            (_: writei) => Ok(Instruction::Writei),
            (_: writes) => Ok(Instruction::Writes),
            (_: read) => Ok(Instruction::Read),
            (_: call) => Ok(Instruction::Call),
            (_: return_) => Ok(Instruction::Return),

            (_: start) => Ok(Instruction::Start),
            (_: nop) => Ok(Instruction::Nop),
            (_: stop) => Ok(Instruction::Stop),
        }
    }
}

pub fn parse(input: &str) -> Result<Vec<Instruction>> {

    let mut parser = Rdp::new(StringInput::new(input));

    parser.code();

    if !parser.end() {
        let (r, pos) = parser.expected();
        let (line, col) = parser.input().line_col(pos);
        bail!("line({}), col({:?}) => expected rules: {:?}", line, col, r);
    }

    parser.compute()
}