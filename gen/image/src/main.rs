
/*
    ........
    .oo..oo.  - encoding of a tile (8x8 pixels):
    XXXoXXXo
    XXXXXXXX  - 2bits per pixel
    .XXXXXX.  - display sets 1st half of the value for the whole row of 8 pixels
    ..XXXX..  - display sets 2nd half of the value for the whole row of 8 pixels
    ...X#...
    ...#....

    // The display will 'overlay' the left and the right 
    // layers to get the 2bits color for each pixel.
    let bytes = vec![
        0b0000_0000,    0b0000_0000,
        0b0110_0110,    0b0110_0110,
        0b0001_0001,    0b1111_1111,
        0b0000_0000,    0b1111_1111,
        0b0000_0000,    0b0111_1110,
        0b0000_0000,    0b0011_1100,
        0b0000_1000,    0b0001_0000,
        0b0001_0000,    0b0000_0000,
    ];
*/

// <https://netpbm.sourceforge.net/doc/ppm.html>

use std::io::{ Read, Write, BufReader };
use std::fs::File;

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

    // For each tile
    for ty in 0..(h / 8) {
        for tx in 0..(w / 8) {
            // Linear tile index
            let ti = (w / 8) * ty + tx;

            // For each row in the tile
            for ry in 0..8 {
                // Byte indices to write
                let byb = ti * 16 + ry * 2;
                let bya = byb + 1;

                // For each pixel in the tile
                for rx in 0..8 {
                    // Source value
                    let si = ty * w * 8 + tx * 8 + ry * w + rx;
                    println!("{ti}:{si}");
                    let src = in_bytes[b + si];

                    // Index of the color in the palette
                    let pi = palette.iter().position(|pv| *pv == src).unwrap() as u8;
                    let pa = ((pi & 2) >> 1) << 7 - rx;
                    let pb = (pi & 1) << 7 - rx;

                    // Write pixel
                    out_bytes[bya] |= pa;
                    out_bytes[byb] |= pb;
                }
            }
        }
    }

    let mut f = File::create(out_path).expect("Could not create output file.");
    f.write_all(&out_bytes).expect("Could not write output file.");

    println!("{}B written from {w}x{h} pixels.", out_bytes.len());
}


