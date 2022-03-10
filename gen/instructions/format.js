
let output = `
// File generated automatically
//  - from <https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html>
//  - by code in 'gen/instructions'
//
// Do no edit manually.
`;

const queryString = window.location.search;
const urlParams = new URLSearchParams(queryString);

// Add a '?save' parameter to the url to generate and save a file to disk.
const save = urlParams.has('save');

let name = '';

const pascal = (str) => str.charAt(0).toUpperCase() + str.slice(1).toLowerCase();

let t = '    ';
let tt = t + t;
let ttt = tt + t;
let init = false;
let initCb = false;

let cbOpStr = '';
let opStr = '';
let matchBranchEnd = ttt + t + '])\n' + ttt + '}\n\n';

for (instr of INSTRUCTIONS) {
    let curOpStr = '';

    if (instr.name == 'prefix') continue;

    if (name != instr.name) {
        if ((instr.cb && initCb) || (!instr.cb && init)) {
            curOpStr += matchBranchEnd;
        }

        else {
            if (instr.cb) {
                initCb = true;
            }

            else {
                init = true;
            }
        }

        let pName  = pascal(instr.name);
        curOpStr += 
            `${ttt}${pName} => {\n${ttt}${t}OpCode::get_opcode(instruction, ${instr.cb}, vec![\n`;

        name = instr.name;
    }

    let args = instr.args.map(arg => {
        let at = false;

        if (['C', 'NC', 'Z', 'NZ'].includes(arg)) {
            return `Arg::Token(Flag${pascal(arg)})`;
        }

        if (instr.name == 'rst') {
            return `Arg::Const(Word)`;
        }

        if (arg.includes('(')) {
            let e = arg.length - 1;
            at = true;
            arg = arg.slice(1, e);
        }

        if (['d8', 'd16', 'a8', 'a16', 'r8'].includes(arg)) {
            let ty = "";

            if (['d8', 'a8', 'r8'].includes(arg)) {
                ty = 'Byte';
            }

            if (['d16', 'a16'].includes(arg)) {
                ty = 'Word';
            }

            return at 
                ? `Arg::At(Box::new(Arg::Const(${ty})))` 
                : `Arg::Const(${ty})`;
        }

        if (!isNaN(parseInt(arg))) {
            return 'Arg::Const(Byte)';
        }

        return at 
            ? `Arg::At(Box::new(Arg::Token(${pascal(arg)})))` 
            : `Arg::Token(${pascal(arg)})`;
    }).join(', ');

    curOpStr += `${ttt}${tt}(${instr.len}, ${instr.code}, vec![${args}]),\n`;

    if (instr.cb) {
        cbOpStr += curOpStr;
    }

    else {
        opStr += curOpStr;
    }
}

opStr += matchBranchEnd;
cbOpStr += matchBranchEnd;

let instructions_output = instructions_rs.replace(
    '// {{{ js }}}',
    `match instr_ty {
${opStr}\
            // CB instructions

${cbOpStr}\
            _ => unreachable!("Op not found"),
        }`
);

output += instructions_output; 

if (save) {
    const blob = new Blob([output], {type: 'text/plain;charset=utf-8'});

    const a = document.createElement('a');
    a.href= URL.createObjectURL(blob);
    a.download = 'instructions.rs';
    a.click(); 
}

else {
    log("Append '?save' to the url to create a file.");
    log(output);
}
