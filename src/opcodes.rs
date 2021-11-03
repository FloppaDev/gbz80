// Macro seems to work but throws unused warnings.
#![allow(unused, unused_mut)]

opcodes!{
    :ADC    0x88   0 A B
            0x89   0 A C
            0x8A   0 A D
            0x8B   0 A E
            0x8C   0 A H
            0x8D   0 A L
            0x8E   0 A AT0 HL AT1
            0x8F   0 A A
            0xCE   1 A LIT

    :ADD    0x80   0 A B
            0x81   0 A C
            0x82   0 A D
            0x83   0 A E
            0x84   0 A H
            0x85   0 A L
            0x86   0 A AT0 HL AT1
            0x87   0 A A
            0xC6   1 A LIT
            0x09   0 HL BC
            0x19   0 HL DE
            0x29   0 HL HL
            0x39   0 HL SP
            0xE8   1 SP LIT

    :AND    0xA0   0 B
            0xA1   0 C
            0xA2   0 D
            0xA3   0 E
            0xA4   0 H
            0xA5   0 L
            0xA6   0 AT0 HL AT1
            0xA7   0 A
            0xE6   1 LIT

    :BIT    0xCB40 0 B0 B
            0xCB41 0 B0 C
            0xCB42 0 B0 D
            0xCB43 0 B0 E
            0xCB44 0 B0 H
            0xCB45 0 B0 L
            0xCB46 0 B0 AT0 HL AT1
            0xCB47 0 B0 A
            0xCB48 0 B1 B
            0xCB49 0 B1 C
            0xCB4A 0 B1 D
            0xCB4B 0 B1 E
            0xCB4C 0 B1 H
            0xCB4D 0 B1 L
            0xCB4E 0 B1 AT0 HL AT1
            0xCB1F 0 B1 A
            0xCB50 0 B2 B
            0xCB51 0 B2 C
            0xCB52 0 B2 D
            0xCB53 0 B2 E
            0xCB54 0 B2 H
            0xCB55 0 B2 L
            0xCB56 0 B2 AT0 HL AT1
            0xCB57 0 B2 A
            0xCB58 0 B3 B
            0xCB59 0 B3 C
            0xCB5A 0 B3 D
            0xCB5B 0 B3 E
            0xCB5C 0 B3 H
            0xCB5D 0 B3 L
            0xCB5E 0 B3 AT0 HL AT1
            0xCB5F 0 B3 A
            0xCB60 0 B4 B
            0xCB61 0 B4 C
            0xCB62 0 B4 D
            0xCB63 0 B4 E
            0xCB64 0 B4 H
            0xCB65 0 B4 L
            0xCB66 0 B4 AT0 HL AT1
            0xCB67 0 B4 A
            0xCB68 0 B5 B
            0xCB69 0 B5 C
            0xCB6A 0 B5 D
            0xCB6B 0 B5 E
            0xCB6C 0 B5 H
            0xCB6D 0 B5 L
            0xCB6E 0 B5 AT0 HL AT1
            0xCB6F 0 B5 A
            0xCB70 0 B6 B
            0xCB71 0 B6 C
            0xCB72 0 B6 D
            0xCB73 0 B6 E
            0xCB74 0 B6 H
            0xCB75 0 B6 L
            0xCB76 0 B6 AT0 HL AT1
            0xCB77 0 B6 A
            0xCB78 0 B7 B
            0xCB79 0 B7 C
            0xCB7A 0 B7 D
            0xCB7B 0 B7 E
            0xCB7C 0 B7 H
            0xCB7D 0 B7 L
            0xCB72 0 B7 AT0 HL AT1
            0xCB7F 0 B7 A

    :CALL   0xCD   2 LIT
            0xDC   2 FLAG_C LIT
            0xD4   2 FLAG_NC LIT
            0xC4   2 FLAG_NZ LIT
            0xCC   2 FLAG_Z LIT

    :CCF    0x3F   0

    :CP     0xB8   0 B
            0xB9   0 C
            0xBA   0 D
            0xBB   0 E
            0xBC   0 H
            0xBD   0 L
            0xBE   0 AT0 HL AT1
            0xBF   0 A
            0xFE   1 LIT

    :CPL    0x2F   0

    :DAA    0x27   0

    :DEC    0x05   0 B
            0x0B   0 BC
            0x0D   0 C
            0x15   0 D
            0x1B   0 DE
            0x1D   0 E
            0x25   0 H
            0xDD25 0 HIX
            0xFD25 0 HIY
            0x2B   0 HL
            0x2D   0 L
            0x35   0 AT0 HL AT1
            0x3D   0 A
            0x3B   0 SP

    :DI     0xF3   0

    :EI     0xFB   0

    :HALT   0x76   0

    :INC    0x04   0 B
            0x03   0 BC
            0x0C   0 C
            0x14   0 D
            0x13   0 DE
            0x1C   0 E
            0x24   0 H
            0x23   0 HL
            0x2C   0 L
            0x34   0 AT0 HL AT1
            0x3C   0 A
            0x33   0 SP

    :JP     0xE9   0 AT0 HL AT1
            0xC3   2 LIT
            0xDA   2 FLAG_C LIT
            0xD2   2 FLAG_NC LIT
            0xC2   2 FLAG_NZ LIT
            0xCA   2 FLAG_Z LIT

    :JR     0x38   1 FLAG_C LIT
            0x18   1 LIT
            0x30   1 FLAG_NC LIT
            0x20   1 FLAG_NZ LIT
            0x28   1 FLAG_Z LIT

    :LD     0xEA   2 AT0 LIT AT1 A
            0x08   2 AT0 LIT AT1 SP
            0x02   0 AT0 BC AT1 A
            0x12   0 AT0 DE AT1 A
            0x77   0 AT0 HL AT1 A
            0x70   0 AT0 HL AT1 B
            0x71   0 AT0 HL AT1 C
            0x72   0 AT0 HL AT1 D
            0x73   0 AT0 HL AT1 E
            0x74   0 AT0 HL AT1 H
            0x75   0 AT0 HL AT1 L
            0x36   1 AT0 HL AT1 LIT
            0xE2   0 AT0 LIT_HEX PLUS C AT1 A
            0xE0   1 AT0 LIT_HEX PLUS LIT AT1 A
            0xFA   2 A AT0 LIT AT1
            0x0A   0 A AT0 BC AT1
            0x1A   0 A AT0 DE AT1
            0x78   0 A B
            0x79   0 A C
            0x7A   0 A D
            0x7B   0 A E
            0x7C   0 A H
            0x7D   0 A L
            0x7E   0 A AT0 HL AT1
            0x7F   0 A A
            0x3E   1 A LIT
            0xF0   1 A AT0 LIT_HEX PLUS LIT AT1
            0xF2   0 A AT0 LIT_HEX PLUS C AT1
            0x40   0 B B
            0x41   0 B C
            0x42   0 B D
            0x43   0 B E
            0x44   0 B H
            0x45   0 B L
            0x46   0 B AT0 HL AT1
            0x47   0 B A
            0x06   1 B LIT
            0x01   2 BC LIT
            0x48   0 C B
            0x49   0 C C
            0x4A   0 C D
            0x4B   0 C E
            0x4C   0 C H
            0x4D   0 C L
            0x4E   0 C AT0 HL AT1
            0x4F   0 C A
            0x0E   1 C LIT
            0x50   0 D B
            0x51   0 D C
            0x52   0 D D
            0x53   0 D E
            0x54   0 D H
            0x55   0 D L
            0x56   0 D AT0 HL AT1
            0x57   0 D A
            0x16   1 D LIT
            0x11   2 DE LIT
            0x58   0 E B
            0x59   0 E C
            0x5A   0 E D
            0x5B   0 E E
            0x5C   0 E H
            0x5D   0 E L
            0x5E   0 E AT0 HL AT1
            0x5F   0 E A
            0x1E   1 E LIT
            0x60   0 H B
            0x61   0 H C
            0x62   0 H D
            0x63   0 H E
            0x64   0 H H
            0x65   0 H L
            0x66   0 H AT0 HL AT1
            0x67   0 H A
            0x26   1 H LIT
            0x21   2 HL LIT
            0xF8   1 HL SP PLUS LIT
            0x68   0 L B
            0x69   0 L C
            0x6A   0 L D
            0x6B   0 L E
            0x6C   0 L H
            0x6D   0 L L
            0x6E   0 L AT0 HL AT1
            0x6F   0 L A
            0x2E   1 L LIT

    :LDI    0x2A   0 A AT0 HL AT1
            0x22   0 AT0 HL AT1 A

    :LDD    0x32   0 A AT0 HL AT1
            0x3A   0 AT0 HL AT1 A

    :NOP    0x00   0

    :OR     0xB0   0 B
            0xB1   0 C
            0xB2   0 D
            0xB3   0 E
            0xB4   0 H
            0xB5   0 L
            0xB6   0 AT0 HL AT1
            0xB7   0 A
            0xF6   1 LIT

    :POP    0xF1   0 AF
            0xC1   0 BC
            0xD1   0 DE
            0xE1   0 HL

    :PUSH   0xF5   0 AF
            0xC5   0 BC
            0xD5   0 DE
            0xE5   0 HL

    :RES    0xCB80 0 B0 B
            0xCB81 0 B0 C
            0xCB82 0 B0 D
            0xCB83 0 B0 E
            0xCB84 0 B0 H
            0xCB85 0 B0 L
            0xCB86 0 B0 AT0 HL AT1
            0xCB87 0 B0 A
            0xCB88 0 B1 B
            0xCB89 0 B1 C
            0xCB8A 0 B1 D
            0xCB8B 0 B1 E
            0xCB8C 0 B1 H
            0xCB8D 0 B1 L
            0xCB8E 0 B1 AT0 HL AT1
            0xCB8F 0 B1 A
            0xCB90 0 B2 B
            0xCB91 0 B2 C
            0xCB92 0 B2 D
            0xCB93 0 B2 E
            0xCB94 0 B2 H
            0xCB95 0 B2 L
            0xCB96 0 B2 AT0 HL AT1
            0xCB97 0 B2 A
            0xCB98 0 B3 B
            0xCB99 0 B3 C
            0xCB9A 0 B3 D
            0xCB9B 0 B3 E
            0xCB9C 0 B3 H
            0xCB9D 0 B3 L
            0xCB9E 0 B3 AT0 HL AT1
            0xCB9F 0 B3 A
            0xCBA0 0 B4 B
            0xCBA1 0 B4 C
            0xCBA2 0 B4 D
            0xCBA3 0 B4 E
            0xCBA4 0 B4 H
            0xCBA5 0 B4 L
            0xCBA6 0 B4 AT0 HL AT1
            0xCBA7 0 B4 A
            0xCBA8 0 B5 B
            0xCBA9 0 B5 C
            0xCBAA 0 B5 D
            0xCBAB 0 B5 E
            0xCBAC 0 B5 H
            0xCBAD 0 B5 L
            0xCBAE 0 B5 AT0 HL AT1
            0xCBAF 0 B5 A
            0xCBB0 0 B6 B
            0xCBB1 0 B6 C
            0xCBB2 0 B6 D
            0xCBB3 0 B6 E
            0xCBB4 0 B6 H
            0xCBB5 0 B6 L
            0xCBB6 0 B6 AT0 HL AT1
            0xCBB7 0 B6 A
            0xCBB8 0 B7 B
            0xCBB9 0 B7 C
            0xCBBA 0 B7 D
            0xCBBB 0 B7 E
            0xCBBC 0 B7 H
            0xCBBD 0 B7 L
            0xCBBE 0 B7 AT0 HL AT1
            0xCBBF 0 B7 A

    :RET    0xC9   0
            0xD8   0 FLAG_C
            0xD0   0 FLAG_NC
            0xC0   0 FLAG_NZ
            0xC8   0 FLAG_Z
            0xD9   0

    :RL     0xCB10 0 B
            0xCB11 0 C
            0xCB12 0 D
            0xCB13 0 E
            0xCB14 0 H
            0xCB15 0 L
            0xCB16 0 AT0 HL AT1
            0xCB17 0 A

    :RLA    0x17   0

    :RLC    0xCB00 0 B
            0xCB01 0 C
            0xCB02 0 D
            0xCB03 0 E
            0xCB04 0 H
            0xCB05 0 L
            0xCB06 0 AT0 HL AT1
            0xCB07 0 A

    :RLCA   0x07   0

    :RLD    0xED6F 0

    :RR     0xCB18 0 B
            0xCB19 0 C
            0xCB1A 0 D
            0xCB1B 0 E
            0xCB1C 0 H
            0xCB1D 0 L
            0xCB1E 0 AT0 HL AT1
            0xCB1F 0 A

    :RRA    0x1F   0

    :RRC    0xCB08 0 B
            0xCB09 0 C
            0xCB0A 0 D
            0xCB0B 0 E
            0xCB0C 0 H
            0xCB0D 0 L
            0xCB0E 0 AT0 HL AT1
            0xCB0F 0 A

    :RRCA   0x0F   0

    :RRD    0xED67 0

    :RST    0xC7   0 B0
            0xCF   2 B1 LIT
            0xD7   2 B2 LIT
            0xDF   2 B3 LIT
            0xE7   0 B4
            0xEF   2 B5 LIT
            0xF7   0 B6
            0xFF   0 B7

    :SBC    0x98   0 A B
            0x99   0 A C
            0x9A   0 A D
            0x9B   0 A E
            0x9C   0 A H
            0x9D   0 A L
            0x9E   0 A AT0 HL AT1
            0x9F   0 A A
            0xDE   1 A LIT

    :SCF    0x37   0

    :SET    0xCBC0 0 B0 B
            0xCBC1 0 B0 C
            0xCBC2 0 B0 D
            0xCBC3 0 B0 E
            0xCBC4 0 B0 H
            0xCBC5 0 B0 L
            0xCBC6 0 B0 AT0 HL AT1
            0xCBC7 0 B0 A
            0xCBC8 0 B1 B
            0xCBC9 0 B1 C
            0xCBCA 0 B1 D
            0xCBCB 0 B1 E
            0xCBCC 0 B1 H
            0xCBCD 0 B1 L
            0xCBCE 0 B1 AT0 HL AT1
            0xCBCF 0 B1 A
            0xCBD0 0 B2 B
            0xCBD1 0 B2 C
            0xCBD2 0 B2 D
            0xCBD3 0 B2 E
            0xCBD4 0 B2 H
            0xCBD5 0 B2 L
            0xCBD6 0 B2 AT0 HL AT1
            0xCBD7 0 B2 A
            0xCBD8 0 B3 B
            0xCBD9 0 B3 C
            0xCBDA 0 B3 D
            0xCBDB 0 B3 E
            0xCBDC 0 B3 H
            0xCBDD 0 B3 L
            0xCBDE 0 B3 AT0 HL AT1
            0xCBDF 0 B3 A
            0xCBE0 0 B4 B
            0xCBE1 0 B4 C
            0xCBE2 0 B4 D
            0xCBE3 0 B4 E
            0xCBE4 0 B4 H
            0xCBE5 0 B4 L
            0xCBE6 0 B4 AT0 HL AT1
            0xCBE7 0 B4 A
            0xCBE8 0 B5 B
            0xCBE9 0 B5 C
            0xCBEA 0 B5 D
            0xCBEB 0 B5 E
            0xCBEC 0 B5 H
            0xCBED 0 B5 L
            0xCBEE 0 B5 AT0 HL
            0xCBEF 0 B5 A
            0xCBF0 0 B6 B
            0xCBF1 0 B6 C
            0xCBF2 0 B6 D
            0xCBF3 0 B6 E
            0xCBF4 0 B6 H
            0xCBF5 0 B6 L
            0xCBF6 0 B6 AT0 HL AT1
            0xCBF7 0 B6 A
            0xCBF8 0 B7 B
            0xCBF9 0 B7 C
            0xCBFA 0 B7 D
            0xCBFB 0 B7 E
            0xCBFC 0 B7 H
            0xCBFD 0 B7 L
            0xCBFE 0 B7 AT0 HL AT1
            0xCBFF 0 B7 A

    :SLA    0xCB20 0 B
            0xCB21 0 C
            0xCB22 0 D
            0xCB23 0 E
            0xCB24 0 H
            0xCB25 0 L
            0xCB26 0 AT0 HL AT1
            0xCB27 0 A

    :SLL    0xCB30 0 B
            0xCB31 0 C
            0xCB32 0 D
            0xCB33 0 E
            0xCB34 0 H
            0xCB35 0 L
            0xCB36 0 AT0 HL AT1
            0xCB37 0 A

    :SRA    0xCB28 0 B
            0xCB29 0 C
            0xCB2A 0 D
            0xCB2B 0 E
            0xCB2C 0 H
            0xCB2D 0 L
            0xCB2E 0 AT0 HL AT1
            0xCB2F 0 A

            0xCB38 0 B
            0xCB39 0 C
            0xCB3A 0 D
            0xCB3B 0 E
            0xCB3C 0 H
            0xCB3D 0 L
            0xCB3E 0 AT0 HL AT1
            0xCB3F 0 A

    :STOP   0x1000 0

    :SUB    0x90   0 B
            0x91   0 C
            0x92   0 D
            0x93   0 E
            0x94   0 H
            0x95   0 L
            0x96   0 AT0 HL AT1
            0x97   0 A
            0xDDAD 0 LIX
            0xFDAD 0 LIY
            0xD6   1 LIT

    :SWAP   0xCB30 0 B
            0xCB31 0 C
            0xCB32 0 D
            0xCB33 0 E
            0xCB34 0 H
            0xCB35 0 L
            0xCB36 0 AT0 HL AT1
            0xCB37 0 A

    :XOR    0xA8   0 B
            0xA9   0 C
            0xAA   0 D
            0xAB   0 E
            0xAC   0 H
            0xAD   0 L
            0xAE   0 AT0 HL AT1
            0xAF   0 A
            0xEE   1 LIT
}
