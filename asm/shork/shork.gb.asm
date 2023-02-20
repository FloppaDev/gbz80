
;http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf

#dw VRAM &8000                  ;Vram start
#dw SCY &ff42                   ;Scroll Y
#dw LY &ff44                    ;Line Y
#dw BGP &ff47                   ;Background & Palette

#dw LCDC            &ff40       ;LCD Control
#db LCDC_ON         %10000000   ;LCD On
#db LCDC_BGON       %00000001   ;Background On
#db LCDCF_BG8000    %00010000   ;BG & Window Tile Data Select
#db LCDCF_BG9800    %00000000   ;BG Tile Map Display Select

#db PALETTE %00_01_10_11 
#dw SHORK_LEN ShorkEnd - Shork
#db SHORK_W 10
#db SHORK_H 10

#db DISPLAY_ON LCDC_ON OR LCDC_BGON OR LCDCF_BG9800 OR LCDCF_BG8000
#dw TILE_DATA VRAM
#dw TILE_IDS &9800
#dw TILE_IDS_LEN &9bff - TILE_IDS

ld TEST1

;Interrupts
&0040: reti     ;v-blank
&0048: reti     ;LCD-Stat
&0050: reti     ;Timer
&0058: reti     ;Serial
&0060: reti     ;Joypad

&0100:          ;Entry point     
    nop
    jp Start

&0104:          ;Nintendo logo, must match at boot
    &ce &ed &66 &66 &cc &0d &00 &0b &03 &73 &00 &83 &00 &0c &00 &0d
    &00 &08 &11 &1f &88 &89 &00 &0e &dc &cc &6e &e6 &dd &dd &d9 &99
    &bb &bb &67 &63 &6e &0e &ec &cc &dd &dc &99 &9f &bb &b9 &33 &3e
&0134:

    "SHORK" 0 0 0 0 0 0 0 0 0
&0143:

    &00
    0 0             ;Manufacturer code
    0               ;Super gameboy flag (&00 or &03)
    8               ;Cartridge type
    0               ;Rom size (0=32k, 1=64k, 2=128k ...)
    3               ;Cart Ram size (0, 1=2k, 2=8k, 3=32k)
    1               ;Destination (0=JPN, 1=EU/US)
    &33             ;Old licencee code, must be &33 for SGB
    0               ;Rom version
    0 &0000         ;Header & Rom checksum (calculated by the assembler)

:Start
    nop
    di
    ld sp &ffff

:StopLCDWait
    ;Loop until scan line 145 is reached
    ld a (LY)
    cp 145
    jp NZ StopLCDWait

    ;LCD Off
    ld hl LCDC
    res 7 (hl)

ld de Shork
ld hl TILE_DATA
ld bc SHORK_LEN

:CopyTiles
    ld a (de)
    ldi (hl) a
    inc de

    ;Loop until all tiles are copied
    dec bc
    ld a b
    or c
    jp NZ CopyTiles

;Set the color palette for the tiles.
:SetPalette
    ld a PALETTE
    ld hl BGP
    ldi (hl) a

ld hl TILE_IDS
ld bc TILE_IDS_LEN

:ClearTileIds
    ld 255
    ldi (hl) a 

    ld b
    or c
    dec bc
    jp NZ ClearTileIds

:Center
    #db OFFSET_X 255 XOR ((160 - 80) / 2) + 1
    #db OFFSET_Y 255 XOR ((144 - 80) / 2) + 1

    ld OFFSET_Y
    ld hl SCY
    ldi (hl) a
    ld OFFSET_X
    ld (hl) a

ld d 0
ld b SHORK_W
ld c SHORK_H
ld hl TILE_IDS

:Display
    dec b ;-1 tile to display in this row

    ld d ;The tile to read
    ldi (hl) a ;Display tile
    inc d ;Next tile

    ;Continue row or go the next one
    xor a
    or b

    ;Continue row
    jp NZ Display

    ;Start next row
    dec c ;-1 row to display
    jp NextRow

:NextRow
    ;End display if all rows are done
    xor a
    or c
    jp Z DisplayEnd

    ;Start next line;
    push de
    ld de 0022
    add hl de
    pop de
    ld b SHORK_W
    jp Display

:DisplayEnd
    ;Turn LCD on
    ld hl LCDC
    ld (hl) DISPLAY_ON

    di 
    halt
    nop

:Shork
    #include "shork.bin"
:ShorkEnd

&8000:
