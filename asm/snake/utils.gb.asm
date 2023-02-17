
;aux functions commons and constants

;directions (1 for pressed - directions on high bits, buttons on right)
#db _JOYPAD_BUTTON_UP %01000000
#db _JOYPAD_BUTTON_RIGHT %00010000
#db _JOYPAD_BUTTON_DOWN	%10000000
#db _JOYPAD_BUTTON_LEFT	%00100000
#db _JOYPAD_BUTTON_A %00000001
#db _JOYPAD_BUTTON_B %00000010
#db _JOYPAD_BUTTON_START %00001000
#db _JOYPAD_BUTTON_SELECT %00000100

#dw _DELAY_SHORT 2000
#dw _DELAY_MED 8000

;-----------------------------------------------
;BASIC ONES
;-----------------------------------------------
;Basic functions used everywhere with little
;or zero game-logic relation (like reading
;button states, waiting for vblank...)
;-----------------------------------------------

;shutdown_LCD - wait until we can shutdown the lcd, then return
;if it's already off, instant return
;if not, wait until it's off and then return
;PARAMS - none
:ShutdownLCD
    ld a (rLCDC)
    ;rotate left the bits from the register, since the bit 7 is the on/off one, 
    ;this bit is now in the CARRY flag
	rlca
    ;ret ONLY if carry == 0, that means the 7 bit (previously rotated) is 0, 
    ;so the LCD is already off
	ret NC

	call WaitVBlank

	xor a 		    ;A = 0x00 (basically bit 7)
	ld (rLCDC) a    ;LCD off now!

	ret

;WaitVBlank - wait until
;PARAMS - none
:WaitVBlank
    ;load the current vertical line. Values range from 0->153. 144->153 is the VBlank period
	ld a (rLY)
	cp 144
    ;if there's carry that means A (rLY) is < 144, so it's a non-blank vertical line, so wait
	jp C WaitVBlank
	ret

;read joypad state and add it to _JOYPAD_STATE ram register
;PARAMS	- none
;RETURN	- none
:SetJoypadState
	push bc
	push hl
	push af
	
	ld hl rP1 ;joypad register $FF00
	ld a, P1F_4 ;get buttons
	ld (hl) a

	ld a (hl)
	ld a (hl)
	ld a (hl)
	ld a (hl) ;read multiple times to prevent bouncing

	and %0000_1111 ;remove the last bits (unused info)
	ld b a

	ld a P1F_5 ;get pad
	ld (hl) a

	ld a (hl)
	ld a (hl)
	ld a (hl)
	ld a (hl)

	swap a ;directions on the last bits, so swap
	and %1111_0000 ;remove the first bits (unused info)
    ;b have, on the low bits, the buttons, so now the 7-4 bits have 
    ;the directions and the 3-1 have the buttons
	or b 
    ;since the state is "0 for pressed" change to "1 for pressed" using complement
	cpl

	ld (_JOYPAD_STATE) a ;save the state on ram

	pop bc
	pop hl
	pop bc

	ret

;MemCopy - copy data (length set on DE) from BC to HL
;PARAMS
;		BC, memadress source
;		HL, memadress destination
; 		DE, data length
;RETURN	- none
:MemCopy
	ld a (bc)
	ld (hl) a
	dec de
	ld a d
	or e
	ret Z ;ret if all data has been copied

	inc bc
	inc hl
	jp MemCopy


;Delay - iterates some time to create a delay
;PARAMS - BC delay
;RETURN	- none
:Delay
	:DelayLoop
        dec bc
        ld a b
        or c
        ret Z
        nop
        jr DelayLoop


;turn the screen "black" gradually by changing
;all the palettes from the "regular" ones to %11111111
;(change BACKGROUND palette and OBJECT0 palette
;- ignore OBJECT1 since we're not using it here)
;PARAMS	- none
;RETURN - none
:FadeOut
	;asume initial palette -> 11100100
	;(yes, this won't work with different ones,
	;but won't be using anything different here)

	ld a %1110_0101
	ld (rBGP) a
	ld (rOBP0) a

	ld bc _DELAY_MED
	call Delay
	
	ld a %1110_1010
	ld (rBGP) a
	ld (rOBP0) a

	ld bc _DELAY_MED
	call Delay

	ld a %1111_1111
	ld (rBGP) a
	ld (rOBP0) a

	ld bc _DELAY_MED
	call Delay

	ret

;reverts the fade_out by restoring a %1111111
;palette to the "original" %11100100
;PARAMS	- none
;RETURN - none
:FadeIn
	;asume final palette -> 11100100
	;(yes, this won't work with different ones,
	;but won't be using anything different here)

	ld a %1110_1010
	ld (rBGP) a
	ld (rOBP0) a

	ld bc _DELAY_MED
	call Delay
	
	ld a %1110_0101
	ld (rBGP) a
	ld (rOBP0) a

	ld bc _DELAY_MED
	call Delay

	ld a %1110_0100
	ld (rBGP) a
	ld (rOBP0) a

	ld bc _DELAY_MED
	call Delay

	ret

;-----------------------------------------------
;GAME LOGIC
;-----------------------------------------------
;Minor game logic utility, like coordinates
;conversion or score modification
;-----------------------------------------------

;given two x-y pixel coordinates, return
;the index for that tile in the map 
;(assume scroll 0, so the range will be the one
;available in the top-left corner)
;PARAMS
; 		B - X
; 		C - Y
;RETURN HL - the index in the map
:PixelsToMapIndex
	;X / 8
	;first remove the "X padding" (-8)
	ld a b
	sub 8
	ld b a

	srl b
	srl b
	srl b

	;Y / 8
	;first remove the "Y padding" (-16)
	ld a c
	sub 16
	ld c a

	srl c
	srl c
	srl c

	xor a
	ld h a
	ld l b

	;increment Y until c == 0
	:PixelsToMapIndexLoop
        ld a c
        or 0
        ret Z

        ld de 32
        add hl de ;full line

        dec c
        jr PixelsToMapIndexLoop

;return a pair of pixel coordinates X - Y (on BC)
;PARAMS none
;RETURN
; 		B - X coordinate
; 		C - Y coordinate
;if all the positions are invalid, this will be
;trapped in an endless loop, but since the max segments
;is limited by a single 8-bits register, we cannot fill
;the whole background with segments, so no worries for now
:GetFreePosition
	;--------------------------------
	;X POSITION
	;--------------------------------

	ld a (_PSEUDORANDOM_VAL)
    :PseudorandomXLoop
        cp 18 ;18 valid tiles on x (20 - 2 walls)
        ;jr Z will be exact 18, so if we start at 8, 18 will be the right wall
        jr C PseudorandomXEndLoop 

        sub 18
        jr PseudorandomXLoop

        :PseudorandomXEndLoop
    ld d a

	;----------------------

	;initial point, the upper-left pixels (the actual "0,0" valid playground)
	ld a 16 ;remember the 8 offset
	ld b a
	
	:PositionXLoop
        ld a d
        or a
        jr z PositionXDone

        ld a b
        add 8
        ld b a
        dec d
        jr PositionXLoop
        :PositionXDone

	;--------------------------------
	;Y POSITION
	;--------------------------------

	ld a (_PSEUDORANDOM_VAL)
	:pseudorandom_y_loop
        cp 16 ;16 valid tiles on y (16 - 2 walls)
        jr C PseudorandomYEndLoop

        sub 16
        jr PseudorandomYLoop

        :PseudorandomYEndLoop
	ld d a

	;----------------------

	;initial point, the upper-left pixels (the actual "0,0" valid playground)
	ld a 24 ;16 offset on Y
	ld c a
	
	:PositionYLoop
        ld a d
        or a
        jr Z PositionYDone

        ld a c
        add 8
        ld c a
        dec d
        jr PositionYLoop
        :PositionYDone

	;--------------------------------
	;CHECK IF POSITION IS "FREE"
	;--------------------------------

	;position free if
	;- different than current item
	;- different than player
	;- different than any segment background

	;check current item
	ld a (_ITEM_POS_X)
	cp b
	jr NZ CheckPlayer
	ld a (_ITEM_POS_Y)
	cp c
	jr NZ CheckPlayer
	;position not empty, try again with differnet _PSEUDORANDOM
	ld a (_PSEUDORANDOM_VAL)
	add a
	ld (_PSEUDORANDOM_VAL) a
	jp GetFreePosition

	;check player
	:CheckPlayer
	ld a (_PLAYER_POS_X)
	cp b
	jr NZ CheckSegments
	ld a [_PLAYER_POS_Y]
	cp c
	jr NZ CheckSegments
	;position not empty, try again
	ld a (_PSEUDORANDOM_VAL)
	add a
	ld (_PSEUDORANDOM_VAL) a
	jp GetFreePosition

	;check background segment
	:CheckSegments

	push bc
	call PixelsToMapIndex
	pop bc
	ld de _SEGMENTS_TTL
	add hl de
	ld a (hl)
	or a
	jr Z PositionFree
	ld a (_PSEUDORANDOM_VAL)
	add a
	ld (_PSEUDORANDOM_VAL) a
	jp GetFreePosition

	:PositionFree
	ret

;increments the score by 1
;also draws the proper sprites
:IncScoreAndDraw
	ld a (_SCORE_VAL)
	add 1
	ld (_SCORE_VAL) a

	call WaitVBlank

	ld a (_SCORE_DIGIT_3_SPRITE_INDEX)
	add 1
    #db invalid_tiles_start _TILE_NUMBERS_OFFSET_MAX + 1
	cp _invalid_tiles_start
	jr Z DrawScore2

	;just inc 1 and return
	ld (_SCORE_DIGIT_3_SPRITE_INDEX) a
	ret

	:DrawScore2
	ld a _TILE_NUMBERS_OFFSET
	ld (_SCORE_DIGIT_3_SPRITE_INDEX) a

	ld a (_SCORE_DIGIT_2_SPRITE_INDEX)
	add 1
	cp _invalid_tiles_start
	jr z, .draw_score_1

	;just inc 1 and return
	ld (_SCORE_DIGIT_2_SPRITE_INDEX) a
	ret

	:DrawScore1
	ld a _TILE_NUMBERS_OFFSET
	ld (_SCORE_DIGIT_1_SPRITE_INDEX) a

	ret

;-----------------------------------------------
;GRAPHICS
;-----------------------------------------------
;Only graphic operations like loading _SCRN0
;with a tilemap, reseting the score tiles to
;the initial index ("0") or drawing the item
;-----------------------------------------------


;load the board map in _SCRN0 and reset the SEGMENTS_TTL values
;(also called when reseting the game)
;notice that this DOESN'T CALL WAIT_VBLANK
;PARAMS	- none
;RETURN - none
:LoadBoardScrn
	ld bc BoardMap
	ld de BoardMapLen
	ld hl _SCRN0 ;$9800
	call MemCopy
	ret

;reset the score digits back to 0
;(also called when reseting the game)
;notice that this DOESN'T CALL WAIT_VBLANK
;PARAMS	- none
;RETURN - none
:ResetScoreDigitsSpriteIndex
	ld a _TILE_NUMBERS_OFFSET
	ld (_SCORE_DIGIT_1_SPRITE_INDEX) a
	ld (_SCORE_DIGIT_2_SPRITE_INDEX) a
	ld (_SCORE_DIGIT_3_SPRITE_INDEX) a
	ret

;reset the item sprite
;notice that this DOESN'T CALL WAIT_VBLANK
;PARAMS	- none
;RETURN - none
:ResetItemSprite
	;item X
	ld a (_ITEM_POS_X)
	ld (_ITEM_SPRITE_POS_X) a

	;item Y
	ld a (_ITEM_POS_Y)
	ld (_ITEM_SPRITE_POS_Y) a

	;item sprite index
	ld hl _ITEM_SPRITE_INDEX
	ld a _ITEM_TILE
	ld (hl) a

	ret

;draws an item on the given positions
;(do not change anything)
;this will be called only when the item needs
;to be moved (after colliding) NOT on every step
;PARAMS	- none
;RETURN - none
:DrawItem
	call WaitVBlank
	;item X
	ld a (_ITEM_POS_X)
	ld (_ITEM_SPRITE_POS_X) a

	;item Y
	ld a (_ITEM_POS_Y)
	ld (_ITEM_SPRITE_POS_Y) a

	ret

;clean the OAM
;PARAMS	- none
;RETURN - none
:CleanOAM
	ld hl _OAMRAM ; _OAM END
	ld de 160; _OAM length ($FE9F - $FE00)
	:CleanOAMLoop
        ld a 0
        ldi (hl) a
        dec de
        ld a d
        or e
        jr NZ CleanOAMLoop
        ret
