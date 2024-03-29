
;Various syntax tests.

; Arithmetic expressions are only allowed in #def directives.
#db FOO 10
#dw BAZ &10AB
#db FLAGS %0110_1010

FOO ;insert FOO here.

;-------------------------------------------------------------------

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
    &CE &ED &66 &66 &CC &0D &00 &0B &03 &73 &00 &83 &00 &0C &00 &0D
    &00 &08 &11 &1F &88 &89 &00 &0E &DC &CC &6E &E6 &DD &DD &D9 &99
    &BB &BB &67 &63 &6E &0E &EC &CC &DD &DC &99 &9F &BB &B9 &33 &3E
&0134:

    "TITLE"         ;Game name
&0143:

#if GBC
    &80             ;GBC flag
#else
    &00
#endif
    0 0             ;Manufacturer code
    0               ;Super gameboy flag (&00 or &03)
    2               ;Cartridge type
    2               ;Rom size (0=32k, 1=64k, 2=128k ...)
    3               ;Cart Ram size (0, 1=2k, 2=8k, 3=32k)
    1               ;Destination (0=JPN, 1=EU/US)
    &33             ;Old licencee code, must be &33 for SGB
    0               ;Rom version
    0 &0000         ;Header & Rom checksum (calculated by the assembler)

;-------------------------------------------------------------------

&0150:Start
    ld sp &FFFF
    nop
    add (hl)

:ScreenOff              ;Turn off the screen so we can define our patterns
    ldh a (&44)         ;Loop until we are in VBlank
    cp 145              ;Is display on scan line 145 yet?
    jr NZ ScreenOff
    
    ld hl &FF40         ;LCDC - LCD Control (R/W)
    res 7 (hl)          ;Turn off the screen

;-------------------------------------------------------------------

#macro mul. .lhs .rhs
    add .lhs .rhs ; bs code
    nop
    add a a
#macro

; mul. a 3     mul
; 10mul. a 3   mul 10 times

    mul. a 10

;-------------------------------------------------------------------

#db FONT_LEN FontEnd - Font

:Font
    #include "../hello/Font96.FNT"
:FontEnd
    ld a b
    add Font

;-------------------------------------------------------------------

#macro db. .b
    .b
#macro

2db. 3

2mul. a b

;&99999 ;OK. it fails as expected.

set 1 a
