
;--------------------
; Currently this example cannot compile:
; #import is not implemented
; jr is not fully implemented
;--------------------

#import "hardware.gb.asm"

;some game definitons

;;OAM stuff
#dw _PLAYER_SPRITE_POS_Y _OAMRAM
#dw _PLAYER_SPRITE_POS_X _PLAYER_SPRITE_POS_Y + 1
#dw _PLAYER_SPRITE_INDEX _PLAYER_SPRITE_POS_X + 1
#dw _PLAYER_SPRITE_ATTR	_PLAYER_SPRITE_INDEX + 1

#dw _ITEM_SPRITE_POS_Y _PLAYER_SPRITE_ATTR + 1
#dw _ITEM_SPRITE_POS_X _ITEM_SPRITE_POS_Y + 1
#dw _ITEM_SPRITE_INDEX _ITEM_SPRITE_POS_X + 1
#dw _ITEM_SPRITE_ATTR _ITEM_SPRITE_INDEX + 1

#dw _SCORE_DIGIT_1_SPRITE_POS_Y _ITEM_SPRITE_ATTR + 1
#dw _SCORE_DIGIT_1_SPRITE_POS_X _SCORE_DIGIT_1_SPRITE_POS_Y + 1
#dw _SCORE_DIGIT_1_SPRITE_INDEX _SCORE_DIGIT_1_SPRITE_POS_X + 1
#dw _SCORE_DIGIT_1_SPRITE_ATTR _SCORE_DIGIT_1_SPRITE_INDEX + 1

#dw _SCORE_DIGIT_2_SPRITE_POS_Y _SCORE_DIGIT_1_SPRITE_ATTR + 1
#dw _SCORE_DIGIT_2_SPRITE_POS_X _SCORE_DIGIT_2_SPRITE_POS_Y + 1
#dw _SCORE_DIGIT_2_SPRITE_INDEX _SCORE_DIGIT_2_SPRITE_POS_X + 1
#dw _SCORE_DIGIT_2_SPRITE_ATTR _SCORE_DIGIT_2_SPRITE_INDEX + 1

#dw _SCORE_DIGIT_3_SPRITE_POS_Y _SCORE_DIGIT_2_SPRITE_ATTR + 1
#dw _SCORE_DIGIT_3_SPRITE_POS_X _SCORE_DIGIT_3_SPRITE_POS_Y + 1
#dw _SCORE_DIGIT_3_SPRITE_INDEX _SCORE_DIGIT_3_SPRITE_POS_X + 1
#dw _SCORE_DIGIT_3_SPRITE_ATTR _SCORE_DIGIT_3_SPRITE_INDEX + 1

;;numeric constants
#db _PLAYER_TILE_HORIZONTAL_VALUE 8
#db _PLAYER_TILE_VERTICAL_VALUE	9
#dw _PLAYER_SPEED_DELAY_VALUE 9000
#db _PLAYER_INITIAL_POS_Y 16 + (10 * 8)
#db _PLAYER_INITIAL_POS_X 8 + (3 * 8)
;after setting the segments they're decremented, so the final viewed segments will be -1
#db _PLAYER_INITIAL_SEGMENTS 4 

#db _ITEM_TILE 10
#db _ITEM_INITIAL_POS_Y	16 + (5 * 8)
#db _ITEM_INITIAL_POS_X	8 + (10 * 8)

;the "right" part will be unused, indeed, maybe we can map this to a continuous segment...
#db _SEGMENTS_TTL_TOTAL	32 * 16 + 19 
#db _BLANK_TILE	0
#db _SEGMENT_TILE 7

#db _TILE_NUMBERS_OFFSET &10 ;tile with "0"
#db _TILE_NUMBERS_OFFSET_MAX &19 ;tile with "9"

;;ram values
#dw _JOYPAD_STATE _RAM
#dw _PLAYER_INDEX_SPRITE _RAM + 1
#dw _PLAYER_DIR_Y _RAM + 2
#dw _PLAYER_DIR_X _RAM + 3
#dw _PLAYER_POS_Y _RAM + 4
#dw _PLAYER_POS_X _RAM + 5 
#dw _PLAYER_MIRRORED_Y _RAM + 6 ;mirrored for sprites
#dw _PLAYER_MIRRORED_X _RAM + 7

#dw _ITEM_POS_Y _RAM + 8
#dw _ITEM_POS_X _RAM + 9
#dw _ITEM_PICKED _RAM + 10

;from &FF04, Divider Register, updated on every joypad interrupt
#dw _PSEUDORANDOM_VAL _RAM + 11 
#dw _SCORE_VAL _RAM + 12

#dw _PLAYER_SEGMENTS_COUNT _RAM + 13 ;limited to 255 segments (8 bits)
#dw _SEGMENTS_TTL _RAM + 14 ;the rest of the ram, basically

#db _SHOW_SCREEN_FLAGS LCDCF_ON OR LCDCF_BG8000 OR LCDCF_BG9800 OR LCDCF_BGON OR LCDCF_OBJ8 OR LCDCF_OBJON

;Interrupts
&0040: reti     ;v-blank
&0048: reti     ;LCD-Stat
&0050: reti     ;Timer
&0058: reti     ;Serial

&0060:JoypadInterrupt
	; update pseudorandom_val on every "valid" button press
	; (enough for our "random stuff")
	ld a (rDIV)
	ld (_PSEUDORANDOM_VAL) a
	call SetJoypadState
	ret ; do not enable interrupts again

&0100:          ;Entry point     
    nop
    jp Start

&0104:          
    nintendo_logo.
&0134:

    "SNAKE" 0 0 0 0 0 0 0 0 0
&0143:

#if GBC
    &80             ;GBC flag
#else
    &00
#endif
    0 0             ;Manufacturer code
    0               ;Super gameboy flag (&00 or &03)
    8               ;Cartridge type
    1               ;Rom size (0=32k, 1=64k, 2=128k ...)
    3               ;Cart Ram size (0, 1=2k, 2=8k, 3=32k)
    1               ;Destination (0=JPN, 1=EU/US)
    &33             ;Old licencee code, must be &33 for SGB
    0               ;Rom version
    0 &0000         ;Header & Rom checksum (calculated by the assembler)

:Start
	;set BG and first palette (the same)
	ld a %1110_0100
	ld (rBGP) a
	ld (rOBP0) a

	;rSCY and rSCX, the scroll
	xor a
	ld (rSCY) a
	ld (rSCX) a
	
	;prepare the interrupts (joypad only)
	ld hl rIE
	ld a IEF_HILO
	ld (hl) a

	;set default variables, clean unclear values, etc
	call InitLogic
	call ShutdownLCD

	;before entering the game loop show a title screen
	;(kinda "press any key to continue")
	;-----------------------------------------------

	;just load some initial tiles and one background

	call CleanOAM

	;load intro tiles to VRAM
	ld bc IntroTiles ;data source
	ld de IntroTilesLen ;data length
	ld hl _VRAM ;data destination, in this case VRAM + offset 0 (it's the first element)
	call MemCopy

	;load intro map
	ld bc IntroMap
	ld de IntroMapLen
	ld hl _SCRN0
	call MemCopy

	;show screen 
	ld a _SHOW_SCREEN_FLAGS
	ld (rLCDC) a

:IntroLoop
	call SetJoypadState
	ld a (_JOYPAD_STATE)
	or a
	jr Z IntroLoop

	;button press, do fade out and shutdown screen
	call WaitVBlank
	call FadeOut
	call ShutdownLCD

	;reset joypad state
	xor a
	ld (_JOYPAD_STATE) a

	;reset the palette
	ld a %1110_0100
	ld (rBGP) a
	ld (rOBP0) a

	; -----------------------------------------------

	;continue with regular game init	
	call LoadGraphics

	;show screen
	;tiles on $80000 (init of _VRAM)
	;background on $9800 (init of _SCRN0)
	;show background and enable objects (8x8)
	ld a _SHOW_SCREEN_FLAGS
	ld (rLCDC) a

:GameLoop
		call move_player

        ;disable interrupts while drawing stuff on screen 
        ;(not the best way to handle controls, I know)
		di 

		call ProcessDrawSegments

		ei

		call DrawPlayer

		call DrawItem

		call CheckCollisions

		;"speed" delay
		ld bc _PLAYER_SPEED_DELAY_VALUE
		call Delay

		;repeat
		jp GameLoop


; ------------------------------------------

;only one call when start playing; set the backgrounds, oam...
;calls some graphics functions that are called after each game_over
;event (like the one that restarts the background)
:LoadGraphics
	;load tiles to VRAM
	ld bc BackTiles ;data source
	ld de BackTilesLen
	ld hl _VRAM ;data destination, in this case VRAM + offset 0 (it's the first element)
	call MemCopy

	ld bc SnakeHeadsTiles ;data source
	ld de SnakeHeadsTilesLen

    #dw _previous_tiles1 _VRAM + BackTilesLen
	ld hl _previous_tiles
	call MemCopy

	ld bc ItemTiles ;data source
	ld de ItemTilesLen
    #dw _previous_tiles2 _VRAM + SnakeHeadsTilesLen + BackTilesLen
	ld hl _previous_tiles2
	call MemCopy

	ld bc FontTiles
	ld de FontTilesLen
    #dw _numbers_start _VRAM + $100
	ld hl _numbers_start ;numbers start at $8100 (tile 10)
	call MemCopy

	;load _SCRN0
	call LoadBoardScrn

	;clean OAM
	call CleanOAM

	;load score digits
	xor a
	ld (_SCORE_DIGIT_1_SPRITE_ATTR) a
	ld (_SCORE_DIGIT_2_SPRITE_ATTR) a
	ld (_SCORE_DIGIT_3_SPRITE_ATTR) a

	;common Y
	ld a 16 ;16
	ld (_SCORE_DIGIT_1_SPRITE_POS_Y) a
	ld (_SCORE_DIGIT_2_SPRITE_POS_Y) a
	ld (_SCORE_DIGIT_3_SPRITE_POS_Y) a

    #db _score_digit_1_sprite_pos_x 8 + (8 * 2)
	ld a _score_digit_1_sprite_pos_x
	ld (_SCORE_DIGIT_1_SPRITE_POS_X) a

    #db _score_digit_2_sprite_pos_x 8 + (8 * 3)
	ld a _score_digit_2_sprite_pos_x
	ld (_SCORE_DIGIT_2_SPRITE_POS_X) a

    #db _score_digit_3_sprite_pos_x 8 + (8 * 4)
	ld a _score_digit_3_sprite_pos_x
	ld (_SCORE_DIGIT_3_SPRITE_POS_X) a

	;set digit sprites
	call ResetScoreDigitsSpriteIndex

	;set item sprite
	call ResetItemSprite

	ret

;first init before each run; also called when reseting after crash
;set the player states vars
:InitLogic
	;set player stuff
	ld a _PLAYER_INITIAL_POS_Y ;pos Y
	ld (_PLAYER_POS_Y) a
	ld a _PLAYER_INITIAL_POS_X ;pos X
	ld (_PLAYER_POS_X) a
	ld a _PLAYER_TILE_HORIZONTAL_VALUE ;sprite right
	ld (_PLAYER_INDEX_SPRITE) a
	xor a
	ld (_PLAYER_MIRRORED_Y) a
	ld (_PLAYER_MIRRORED_X) a
	ld (_PLAYER_DIR_Y) a
	ld a 8
	ld (_PLAYER_DIR_X) a

	;set item stuff
	ld a _ITEM_INITIAL_POS_Y ;pos Y
	ld (_ITEM_POS_Y) a
	ld a _ITEM_INITIAL_POS_X ;pos X
	ld (_ITEM_POS_X) a
	xor a
	ld _ITEM_PICKED a ;item not picked

	;set segments number
	ld a (PLAYER_INITIAL_SEGMENTS)
	ld (_PLAYER_SEGMENTS_COUNT) a

	;set _SEGMENTS_TTL to 0 as ttl for segments
	ld hl _SEGMENTS_TTL
	ld bc _SEGMENTS_TTL_TOTAL
:InitLogicSegmentsTTLLoop
	xor a
	ldi (hl) a
	dec bc
	ld a b
	or c
	jr NZ InitLogicSegmentsTTLLoop

	;reset joypad info
	ld hl _JOYPAD_STATE
	xor a
	ld (hl) a

	;reset pseudorandom_val
	ld a (rDIV)
	ld (_PSEUDORANDOM_VAL) a

	;reset score
	xor a
	ld (_SCORE_VAL) a

	ret

;------------------------------------------

:MovePlayer
	;--------------------
	;SET PLAYER DIRECTION
	;--------------------

	;check directions
	ld hl _JOYPAD_STATE

	;get current dir Y
	;if it's not 0, check only for dir X
	ld a (_PLAYER_DIR_Y)
	or a ;set flags
	jr NZ CheckLeftRight
	;0 -> "moving left-right", so check up/down to change directions
	;1 -> "moving up-down", so check left/right to change directions
	;this works because cannot change from up to down or left to right,
	;it's always an "axis change"

	;check UP
	;--------
	ld a (hl)
	and _JOYPAD_BUTTON_UP
	jr Z MovePlayerCheckDown

	ld a _n8 
	ld (_PLAYER_DIR_Y) a
	xor a
	ld (_PLAYER_DIR_X) a
	ld (_PLAYER_MIRRORED_X) a ;reset the X flip option since we're changing to up/down
	;point UP, VERTICAL sprite
	;point UP, flip the sprite (Y)
	ld a _PLAYER_TILE_VERTICAL_VALUE
	ld (_PLAYER_INDEX_SPRITE) a
	ld a 1
	ld (_PLAYER_MIRRORED_Y) a
	ret

	;check DOWN
	;--------
:MovePlayerCheckDown:
	ld a (hl)
	and _JOYPAD_BUTTON_DOWN
	ret Z

	ld a 8
	ld (_PLAYER_DIR_Y) a
	xor a
	ld (_PLAYER_DIR_X) a
	ld (_PLAYER_MIRRORED_X) a ;reset the X flip option since we're changing to up/down
	;point DOWN, VERTICAL sprite
	;point DOWN, do not flip the sprite
	ld a _PLAYER_TILE_VERTICAL_VALUE
	ld (_PLAYER_INDEX_SPRITE) a
	xor a
	ld (_PLAYER_MIRRORED_Y) a
	ret


:CheckLeftRight
	;check RIGHT
	;--------
	ld a (hl)
	and _JOYPAD_BUTTON_RIGHT
	jr Z MovePlayerCheckLeft

	xor a
	ld (_PLAYER_DIR_Y) a
	ld (_PLAYER_MIRRORED_Y) a ;reset the Y flip option since we're changing to left/right
	ld a 8
	ld (_PLAYER_DIR_X) a
	;point RIGHT, HORIZONTAL sprite
	;point RIGHT, do not flip the sprite
	ld a _PLAYER_TILE_HORIZONTAL_VALUE
	ld (_PLAYER_INDEX_SPRITE) a
	xor a
	ld (_PLAYER_MIRRORED_X) a
	ret

	;check LEFT
	;----------
:MovePlayerCheckLeft
	ld a (hl)
	and _JOYPAD_BUTTON_LEFT
	ret Z

	xor a
	ld (_PLAYER_DIR_Y) a
	ld (_PLAYER_MIRRORED_Y) a ;reset the Y flip option since we're changing to left/right
    #db _n8 (8 XOR 255) + 1
	ld a _n8
	ld (_PLAYER_DIR_X) a
	;point LEFT, HORIZONTAL sprite
	;point LEFT, flip the sprite (X)
	ld a _PLAYER_TILE_HORIZONTAL_VALUE
	ld (_PLAYER_INDEX_SPRITE) a
	ld a 1
	ld (_PLAYER_MIRRORED_X) a

	ret


;since segments are part of the background this
;function will use some wait_vblanks to handle it
:ProcessDrawSegments
	;draw the current segment
	ld a (_PLAYER_POS_X)
	ld b a
	ld a (_PLAYER_POS_Y)
	ld c a

	call PixelsToMapIndex
	;now HL contains the full line from _SCRN0
	;change the background
	push hl
	call WaitVBlank
	ld bc _SCRN0
	add hl bc
	ld a _SEGMENT_TILE ;current segments as ttl
	ld (hl) a

	pop hl
	ld bc _SEGMENTS_TTL
	add hl bc
	ld a (_PLAYER_SEGMENTS_COUNT) ;current segments as ttl
	ld (hl) a

	;if _ITEM_PICKED, _PLAYER_SEGMENTS_COUNT++ and not decrement the list
	;else DECREMENT the current _SEGMENTS_TTL list without _PLAYER_SEGMENT_COUNT++
	;this will create a new segment without decrementing and the next ones will
	;have the TTL increased by 1

	;check item
	ld a (_ITEM_PICKED)
	or a
	jr Z DrawSegmentsNoItem

	;item picked
	xor a
	ld (_ITEM_PICKED) a ;reset flag

	ld a (_PLAYER_SEGMENTS_COUNT)
	add 1
	ld (_PLAYER_SEGMENTS_COUNT) a
	;check for max segments
	cp 255
	jr Z DrawSegmentsMaxSegmentsReached

	jr DrawSegmentsEnd
:DrawSegmentsNoItem:
	;check all the SEGMENTS_TTL and decrement until reaching 0
	ld hl _SEGMENTS_TTL
	ld bc _SEGMENTS_TTL_TOTAL
	ld de _SCRN0
:DrawSegmentsLoop:
	ld a (hl)
	or a ; is 0?
	jr z DrawSegmentsLoopEndIteration ;already 0, so do nothing
	dec a
	ld (hl) a

	or a
	jr NZ DrawSegmentsLoopEndIteration ;is 0 now?

	ld a _BLANK_TILE
	call WaitVBlank
	ld (de) a

:DrawSegmentsLoopEndIteration:
	inc hl
	inc de
	dec bc
	ld a b
	or c
	jr NZ DrawSegmentsLoop

:DrawSegmentsEnd:
	ret

:DrawSegmentsMaxSegmentsReached:
	;well...
	call GameOver
	ret


;player sprite (OAM)
:DrawPlayer

	;--------------------------------
	;PLAYER
	;--------------------------------

	call WaitVBlank

	;player X
	ld hl _PLAYER_POS_X 
	ld a (_PLAYER_DIR_X)
	add a (hl)
	ld (hl) a ;save position
	ld hl _PLAYER_SPRITE_POS_X ;update the OAM with the new position
	ld (hl) a

	;player Y
	ld hl _PLAYER_POS_Y
	ld a (_PLAYER_DIR_Y)
	add a (hl)
	ld (hl) a ;save position
	ld hl _PLAYER_SPRITE_POS_Y ;update the OAM with the new position
	ld (hl) a

	;player sprite index
	ld hl _PLAYER_SPRITE_INDEX
	ld a (_PLAYER_INDEX_SPRITE)
	ld (hl) a

	;since we mirror only X or Y but not X AND Y at the same time,
	;use those absolute values (always the same palette and default params)

	;mirror Y?
	ld a (_PLAYER_MIRRORED_Y)
	or a
	jr NZ DrawMirrorY
	;no mirror
	ld a %0000_0000
	ld (_PLAYER_SPRITE_ATTR) a
	jr DrawCheckMirrorX
:DrawMirrorY
	ld a %0100_0000
	ld (_PLAYER_SPRITE_ATTR) a
	jr DrawCheckMirrorEnd

	;mirror X?
:DrawCheckMirrorX
	ld a (_PLAYER_MIRRORED_X)
	or a
	ld a (_PLAYER_SPRITE_ATTR)
	jr NZ DrawMirrorX
	;no mirror
	ld a %0000_0000
	ld (_PLAYER_SPRITE_ATTR) a
	jr DrawCheckMirrorEnd
:DrawMirrorX:
	ld a %0010_0000
	ld (_PLAYER_SPRITE_ATTR) a

:DrawCheckMirrorEnd:
	ret


;check for collisions between walls, segments and/or items;
;if collided with an item, add score and relocate - in that
;case some VRAM operations will be performed
:CheckCollisions
	;--------------------------------
	;CHECK COLLISIONS WITH WALLS
	;--------------------------------

	;check colisions with WALLS

	;col X
	ld a (_PLAYER_POS_X)

	cp 160 ;20 * 8, right wall
	jr Z CheckCollisionsSetGameOver

	cp 8 ;left wall (+8 "offset")
	jr Z CheckCollisionsSetGameOver

	;col Y
	ld a (_PLAYER_POS_Y)

    ;(18 * 8) + 8, the last tile the player can move 
    ;(18 tiles height plus half of the 16 offset - our sprites are 8x8)
	cp 152 
	jr Z CheckCollisionsSetGameOver

	cp 16 ;the first tile the player can move (8 + 16 'cause it begins "off screen")
	jr Z CheckCollisionsSetGameOver

	;--------------------------------
	;CHECK COLLISIONS WITH ITSELF
	;--------------------------------

	;get the position from the segments block
	ld a (_PLAYER_POS_X)
	ld b a
	ld a (_PLAYER_POS_Y)
	ld c a
	push bc ; save x / y

	call PixelsToMapIndex
	;HL now have the _SEGMENTS_TTL position
	ld bc _SEGMENTS_TTL
	add hl bc
	ld a (hl) ;current position
	or a
	pop bc ;B, player_x / c, player_y

	jr NZ CheckCollisionsSetGameOver

	;--------------------------------
	;CHECK COLLISIONS WITH ITEM
	;--------------------------------
	
	ld a (_ITEM_POS_X)
	cp b ;X axis
	jr NZ NoColItem

	ld a (_ITEM_POS_Y)
	cp c ;Y axis
	jr NZ NoColItem

	ld a 1
	ld (_ITEM_PICKED) a

	call GetFreePosition ;BC

	ld a b
	ld (_ITEM_POS_X) a
	ld a c
	ld (_ITEM_POS_Y) a

	;draw_item in the NEW position
	;(this will call a wait_vblank)
	call DrawItem

	;inc score and draw
	;(this will call a wait_vblank)
	call IncScoreAndDraw

	;check it score > 255
	ld a (_SCORE_VAL)
	sub 255
	jr Z CheckCollisionsSetGameOver

:NoColItem
	ret

:CheckCollisionsSetGameOver
	call GameOver
	ret


;"game over" function
;stop the game for a while, turn screen black, reset
:GameOver
	ld bc 8000
	call Delay
	call WaitVBlank
	call FadeOut
	call ShutdownLCD

	call LoadBoardScrn
	call ResetScoreDigitsSpriteIndex
	call ResetItemSprite

	call InitLogic

	;show screen
	ld a _SHOW_SCREEN_FLAGS
	ld (rLCDC) a

	call DrawPlayer
	call DrawItem
	call FadeIn

	ret

; ------------------------------------------

#import "utils.gb.asm"
#import "tiles.gb.asm"
#import "maps.gb.asm"

&FFFF: &ff
