
;Translated from: https://www.chibiakumas.com/z80/helloworld.php#LessonH9

#dw NEXT_CHAR_X &C000
#dw NEXT_CHAR_Y &C001

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

;--------------------------------------------------

:Start
	nop
	di
	ld sp &ffff		        ; set the stack pointer to highest mem location + 1
	
    ;Position tilemap											
	xor a					
	ld hl &FF42				
	ldi	(hl) a				;FF42: SCY - Tile Scroll Y
	ld	(hl) a				;FF43: SCX - Tile Scroll X
	
	ld (NEXT_CHAR_X) a		;Set cursor to pos 0,0
	ld (NEXT_CHAR_Y) a

;Turn off screen												
:StopLCD_wait				;Turn off the screen so we can define our patterns
	ld a (&FF44)		    ;Loop until we are in VBlank
	cp 145                  ;Is display on scan line 145 yet?
	jr NZ StopLCD_wait      ;no? keep waiting!
	
	ld hl &FF40		        ;LCDC - LCD Control (R/W)
	res 7 (hl)      	    ;Turn off the screen

    ;Define bitmap font											
	ld de BitmapFont		        ;Source bitmaps
	ld hl &8000 			        ;Dest in Vram
	ld bc BITMAP_FONT_SIZE

:Copy2Bitloop	
	ld a (de)		;Read in a byte and INC HL
	inc de
	ldi	(hl) a		;Fill Bitplane 1
	ldi	(hl) a		;Fill Bitplane 2
	dec	bc
	ld a b
	or c
	jr NZ Copy2Bitloop

    ;Define palette												
	#if GBC
        #db PALETTE0 0 * 8
        #db PALETTE1 7 * 8
        
		ld c PALETTE0           ;palette no 0 (back)
		call SetGBCPalettes
		
		ld c PALETTE1		    ;palette no 7 (used by font)
		call SetGBCPalettes
	#else
		ld a %00011011		    ;DDCCBBAA .... A=Background 0=Black, 3=White
		ld hl &FF47
		ldi (hl) a			    ;FF47 	BGP	BG & Window Palette Data  (R/W)	= &FC
		ldi (hl) a			    ;FF48  	OBP0	Object Palette 0 Data (R/W)	= &FF
		cpl					    ;Set sprite Palette 2 to the opposite
		ldi (hl) a			    ;FF49  	OBP1	Object Palette 1 Data (R/W)	= &FF
	#endif

;Turn on screen												
	ld hl &FF40		            ;LCDC - LCD Control (R/W)	EWwBbOoC 
    set 7 (hl)                  ;Turn on the screen
    
;Our program													
	ld hl Message			    ;Address of string
	Call PrintString		    ;Show String to screen
	
	di
	halt
	
;--------------------------------------------------

:PrintString
	ld a (hl)		            ;Print a '255' terminated string 
	cp 255
	ret Z
	inc hl
	call PrintChar
	jr PrintString

;TODO #ds is useless
:Message
    "Hello World 323!"
    255
	
:PrintChar
	push hl
	push bc
		push af
			ld a (NEXT_CHAR_Y)
			ld b a			        ;YYYYYYYY --------
			ld hl NEXT_CHAR_X
			ld a (hl)
			ld c a			        ;-------- ---XXXXX
			inc (hl)
			cp 19
			call Z NewLine
			xor a
			rr b			        ;-YYYYYYY Y-------
			rra
			rr b			        ;--YYYYYY YY------
			rra
			rr b			        ;---YYYYY YYY-----
			rra
			or c			        ;---YYYYY YYYXXXXX
			ld c a
			ld hl &9800	            ;Tilemap base
			add hl bc	
		pop af
		push af
			sub 32			        ;no char <32!
			call LCDWait	        ;Wait for VDP Sync
			ld (hl) a
			#if GBC
				ld bc &FF4F	        ;VBK - CGB Mode Only - VRAM Bank
				
				ld a 1		        ;Turn on GBC extras
				ld (bc) a	
				
				ld (hl) 7	        ;Palette 7
				
				xor a		        ;Turn off GBC extras
				ld (bc) a			
			#endif
		pop af
	pop bc
	pop hl
	ret
	
:NewLine
	push hl
		ld hl NEXT_CHAR_Y		    ;Inc Ypos
		inc (hl)
		ld hl NEXT_CHAR_X
		ld (hl) 0			        ;Reset Xpos
	pop hl
	ret	

;--------------------------------------------------
	
:LCDWait
	push af
        di
:LCDWaitAgain
        ld a (&FF41)  		        ;STAT - LCD Status (R/W)
			                        ;-LOVHCMM
        and %0000_0010		        ;MM=video mode (0/1 =Vram available)  		
        jr NZ LCDWaitAgain 
    pop af	
	ret
 

:SetGBCPalettes
	#if GBC
		ld hl GBPal		
:SetGBCPalettesb
		ldi a (hl)  	;GGGRRRRR
		ld e a
		ldi a (hl)  	;xBBBBBGG
		ld d a
		inc a 			;cp 255
		ret z
		push hl
			call LCDWait    ;Wait for VDP Sync
			ld hl &ff68	
			ld (hl) c	    ;FF68 - BCPS/BGPI - CGB Mode Only - Background Palette Index
			inc hl			
			ld (hl) e	    ;FF69 - BCPD/BGPD - CGB Mode Only - Background Palette Data
			dec hl		
			inc	c		    ;Increase palette address
			ld (hl) c	    ;FF68 - BCPS/BGPI - CGB Mode Only - Background Palette Index
			inc hl		
			ld (hl) d	    ;FF69 - BCPD/BGPD - CGB Mode Only - Background Palette Data
			inc c		    ;Increase palette address
		pop hl
		jr SetGBCPalettesb
	#endif

;xBBBBBGGGGGRRRRR
:GBPal	
    %0111_1100_0000_0000    ;col 0
    %0111_1111_1110_0000	;col 1
    %0000_0000_0001_1111	;col 2
    %0000_0011_1111_1111	;col 3
    %1111_1111_1111_1111	;End of list
	
;Font (1bpp / Black & White)										
#dw BITMAP_FONT_SIZE BitmapFontEnd - BitmapFont
:BitmapFont
		#include "Font96.FNT"	    ;Font bitmap,
:BitmapFontEnd				    ; this is common to all systems


	
