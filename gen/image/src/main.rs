
use std::io::{ Read, Write, BufReader };
use std::fs::File;

// <https://netpbm.sourceforge.net/doc/ppm.html>

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    assert!(args.len() == 3, "Expect input and output paths as arguments.");

    let in_path = &args[1];
    let out_path = &args[2];

    let f = File::open(in_path).expect("Could not open input file.");
    let mut reader = BufReader::new(f);
    let mut in_bytes = Vec::new();
    reader.read_to_end(&mut in_bytes).expect("Coul not read input file.");

    if in_bytes[0] != 0x50 || in_bytes[1] != 0x35 {
        panic!("Format not supported, PGM (PPM P5) required.");
    }

    let mut b = 2;
    let mut info = [0, 0, 0];
    let mut i = 0;

    let mut ascii = String::new();

    loop {
        b += 1;

        if matches!(in_bytes[b], 0x20|0x0a) {
            info[i] = ascii.parse::<usize>().expect("Could not parse ASCII."); 
            i += 1;
            ascii.clear();

        }

        else {
            ascii.push(in_bytes[b] as char);
        }

        if i == 3 {
            break;
        }

        if b >= in_bytes.len() - 1 {
            panic!("Invalid PPM header.");
        }
    }

    let mut palette = vec![];
    b += 1;

    for in_byte in &in_bytes[b..] {
        if !palette.contains(in_byte) {
            palette.push(*in_byte);
        }
    }

    assert!(palette.len() <= 4, "Color palette is limited to 4 colors.");
    palette.sort();

    let [w, h, _] = info;
    let mut out_bytes = vec![0; (w * h) / 4];
    #[allow(non_snake_case)]
    let mut iB = 0;
    let mut ib = 0;

    for in_byte in &in_bytes[b..] {
        let px = palette.iter().position(|b| b == in_byte).unwrap() as u8;
        let px_bits = px << ib;

        out_bytes[iB] |= px_bits;

        ib = (ib + 2) % 8;

        if ib == 0 {
            iB += 1;
        }
    }

    let mut f = File::create(out_path).expect("Could not create output file.");
    f.write_all(&out_bytes).expect("Could not write output file.");
}
