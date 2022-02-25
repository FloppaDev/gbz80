
// - Read instruction tables from <https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html>
// - Flatten instructions.
// - Apply the alternative mnemonics that:
//      - Modify instruction name.
//      - Do not require additional symbols or specific addresses ('+', '$FF00').

const log = console.log;

class Instruction {
    constructor(str, cb, code, name, args, len, cycles, flags) {
        this.str = str;
        this.cb = cb;
        this.code = code;
        this.name = name;
        this.args = args;
        this.len = len;
        this.cycles = cycles;
        this.flags = flags;
    }
}

const INSTRUCTIONS = (() => {

    let instructions = [];

    /// Store the list of instructions.
    const store = (cb, code, html) => {
        if (html == '&nbsp;') return;

        const lines = html.replaceAll('&nbsp;', ' ').split('<br>');
        const ln0 = lines[0].split(' ');
        const ln1 = lines[1].split(' ').filter(x => x != '');

        const str = lines.join(' ').replaceAll('  ', ' ');
        let name = ln0[0].toLowerCase();
        const args = ln0.length > 1 ? ln0[1].split(',') : [];

        for (i in args) {
            if (args[i].includes('-')) {
                name += 'd';
                args[i] = args[i].replace('-', '');
            }

            else if (args[i].includes('+')) {
                if (args[i].includes('r8')) {
                    name += 'hl';
                    args[i] = args[i].replace('+r8', '');
                }

                else {
                    name += 'i';
                    args[i] = args[i].replace('+', '');
                }
            }
        }

        const [len, cycles] = ln1;
        const flags = lines[2].split(' ');
        
        const instruction = new Instruction(str, cb, code, name, args, len, cycles, flags);

        instructions.push(instruction);
    };

    // Find instruction tables.
    const tables = Array.from(document.querySelectorAll('table'))
        .slice(0, 2)
        .map(table => table.getElementsByTagName('tbody')[0]);

    const hexValues = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'
    ];

    let t = 0;

    // Flatten and store instructions.
    for (table of tables) {
        let y = 0;

        for (tr of Array.from(table.children).slice(1)) {
            let x = 0;

            for (html of Array.from(tr.children).slice(1).map(x => x.innerHTML)) {
                store(t == 1, `0x${hexValues[y]}${hexValues[x]}`, html); 
                x++;
            }

            y++;
        }

        t++;
    }

    // Sort instructions by name;
    return instructions.sort((a, b) => a.name.localeCompare(b.name));

})();
