
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
let matchBranchEnd = ']),\n\n';

for (instr of INSTRUCTIONS) {
    let curOpStr = '';

    if (instr.name == 'prefix') continue;

    let c_is_flag = ['call', 'jp', 'jr', 'ret'].includes(instr.name);

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
        curOpStr += `${tt}${pName} => (${instr.cb}, vec![\n`;

        name = instr.name;
    }else {
        curOpStr += ",\n";
    }

    let args = instr.args.map(arg => {
        let at = false;

        if (['NC', 'Z', 'NZ'].includes(arg) || (arg == 'C' && c_is_flag)) {
            return `ty(Flag${pascal(arg)})`;
        }

        if (instr.name == 'rst') {
            return `imm(Word)`;
        }

        if(["0", "1", "2", "3", "4", "5", "6", "7"].includes(arg)) {
            return `bit(${arg})`;
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
                ? `at(imm(${ty}))` 
                : `imm(${ty})`;
        }

        if (!isNaN(parseInt(arg))) {
            return 'imm(Byte)';
        }

        return at 
            ? `at(ty(${pascal(arg)}))` 
            : `ty(${pascal(arg)})`;
    }).join(', ');

    curOpStr += `${ttt}(${instr.len}, ${instr.code}, vec![${args}])`;

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
    `match tty {
${opStr}\
        // CB instructions

${cbOpStr}\
            _ => bug!("Op not found"),
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
