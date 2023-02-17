
;*
;* Gameboy Hardware definitions
;*
;* Based on Jones' hardware.inc
;* And based on Carsten Sorensen's ideas.
;*
;* Rev 1.1 - 15-Jul-97 : Added define check
;* Rev 1.2 - 18-Jul-97 : Added revision check macro
;* Rev 1.3 - 19-Jul-97 : Modified for RGBASM V1.05
;* Rev 1.4 - 27-Jul-97 : Modified for new subroutine prefixes
;* Rev 1.5 - 15-Aug-97 : Added _HRAM, PAD, CART defines
;*                     :  and Nintendo Logo
;* Rev 1.6 - 30-Nov-97 : Added rDIV, rTIMA, rTMA, & rTAC
;* Rev 1.7 - 31-Jan-98 : Added _SCRN0, _SCRN1
;* Rev 1.8 - 15-Feb-98 : Added rSB, rSC
;* Rev 1.9 - 16-Feb-98 : Converted I/O registers to &FFXX format
;* Rev 2.0 -           : Added GBC registers
;* Rev 2.1 -           : Added MBC5 & cart RAM enable/disable defines
;* Rev 2.2 -           : Fixed NR42,NR43, & NR44 equates
;* Rev 2.3 -           : Fixed incorrect _HRAM equate
;* Rev 2.4 - 27-Apr-13 : Added some cart defines (AntonioND)
;* Rev 2.5 - 03-May-15 : Fixed format (AntonioND)
;* Rev 2.6 - 09-Apr-16 : Added GBC OAM and cart defines (AntonioND)
;* Rev 2.7 - 19-Jan-19 : Added rPCMXX (ISSOtm)
;* Rev 2.8 - 03-Feb-19 : Added audio registers flags (Álvaro Cuesta)
;* Rev 2.9 - 28-Feb-20 : Added utility rP1 constants
;* Rev 3.0 - 27-Aug-20 : Register ordering, byte-based sizes, OAM additions, 
;*                       general cleanup (Blitter Object)

#dw _VRAM        &8000 ;&8000->&9FFF
#dw _VRAM8000    _VRAM
#dw _VRAM8800    _VRAM+&800
#dw _VRAM9000    _VRAM+&1000
#dw _SCRN0       &9800 ;&9800->&9BFF
#dw _SCRN1       &9C00 ;&9C00->&9FFF
#dw _SRAM        &A000 ;&A000->&BFFF
#dw _RAM         &C000 ;&C000->&CFFF / &C000->&DFFF
#dw _RAMBANK     &D000 ;&D000->&DFFF
#dw _OAMRAM      &FE00 ;&FE00->&FE9F
#dw _IO          &FF00 ;&FF00->&FF7F,&FFFF
#dw _AUD3WAVERAM &FF30 ;&FF30->&FF3F
#dw _HRAM        &FF80 ;&FF80->&FFFE

;*** MBC5 Equates ***

#dw rRAMG        &0000 ;&0000->&1fff
#dw rROMB0       &2000 ;&2000->&2fff
#dw rROMB1       &3000 ;&3000->&3fff - If more than 256 ROM banks are present.
#dw rRAMB        &4000 ;&4000->&5fff - Bit 3 enables rumble (if present)

;***************************************************************************
;*
;* Custom registers
;*
;***************************************************************************

;--
;-- P1 (&FF00)
;-- Register for reading joy pad info. (R/W)
;--
#dw rP1 &FF00

#db P1F_5 %00100000 ;P15 out port, set to 0 to get buttons
#db P1F_4 %00010000 ;P14 out port, set to 0 to get dpad
#db P1F_3 %00001000 ;P13 in port
#db P1F_2 %00000100 ;P12 in port
#db P1F_1 %00000010 ;P11 in port
#db P1F_0 %00000001 ;P10 in port

#db P1F_GET_DPAD P1F_5
#db P1F_GET_BTN  P1F_4
#db P1F_GET_NONE P1F_4 OR P1F_5

;--
;-- SB (&FF01)
;-- Serial Transfer Data (R/W)
;--
#dw rSB &FF01

;--
;-- SC (&FF02)
;-- Serial I/O Control (R/W)
;--
#dw rSC &FF02

;--
;-- DIV (&FF04)
;-- Divider register (R/W)
;--
#dw rDIV &FF04

;--
;-- TIMA (&FF05)
;-- Timer counter (R/W)
;--
#dw rTIMA &FF05

;--
;-- TMA (&FF06)
;-- Timer modulo (R/W)
;--
#dw rTMA &FF06

;--
;-- TAC (&FF07)
;-- Timer control (R/W)
;--
#dw rTAC &FF07

#db TACF_START  %00000100
#db TACF_STOP   %00000000
#db TACF_4KHZ   %00000000
#db TACF_16KHZ  %00000011
#db TACF_65KHZ  %00000010
#db TACF_262KHZ %00000001

;--
;-- IF (&FF0F)
;-- Interrupt Flag (R/W)
;--
#dw rIF &FF0F

;--
;-- AUD1SWEEP/NR10 (&FF10)
;-- Sweep register (R/W)
;--
;-- Bit 6-4 - Sweep Time
;-- Bit 3   - Sweep Increase/Decrease
;--           0: Addition    (frequency increases???)
;--           1: Subtraction (frequency increases???)
;-- Bit 2-0 - Number of sweep shift (# 0-7)
;-- Sweep Time: (n*7.8ms)
;--
#dw rNR10 &FF10
#dw rAUD1SWEEP rNR10

#db AUD1SWEEP_UP   %00000000
#db AUD1SWEEP_DOWN %00001000

;--
;-- AUD1LEN/NR11 (&FF11)
;-- Sound length/Wave pattern duty (R/W)
;--
;-- Bit 7-6 - Wave Pattern Duty (00:12.5% 01:25% 10:50% 11:75%)
;-- Bit 5-0 - Sound length data (# 0-63)
;--
#dw rNR11 &FF11
#dw rAUD1LEN rNR11

;--
;-- AUD1ENV/NR12 (&FF12)
;-- Envelope (R/W)
;--
;-- Bit 7-4 - Initial value of envelope
;-- Bit 3   - Envelope UP/DOWN
;--           0: Decrease
;--           1: Range of increase
;-- Bit 2-0 - Number of envelope sweep (# 0-7)
;--
#dw rNR12 &FF12
#dw rAUD1ENV rNR12

;--
;-- AUD1LOW/NR13 (&FF13)
;-- Frequency low byte (W)
;--
#dw rNR13 &FF13
#dw rAUD1LOW rNR13

;--
;-- AUD1HIGH/NR14 (&FF14)
;-- Frequency high byte (W)
;--
;-- Bit 7   - Initial (when set, sound restarts)
;-- Bit 6   - Counter/consecutive selection
;-- Bit 2-0 - Frequency's higher 3 bits
;--
#dw rNR14 &FF14
#dw rAUD1HIGH rNR14

;--
;-- AUD2LEN/NR21 (&FF16)
;-- Sound Length; Wave Pattern Duty (R/W)
;--
;-- see AUD1LEN for info
;--
#dw rNR21 &FF16
#dw rAUD2LEN rNR21

;--
;-- AUD2ENV/NR22 (&FF17)
;-- Envelope (R/W)
;--
;-- see AUD1ENV for info
;--
#dw rNR22 &FF17
#dw rAUD2ENV rNR22

;--
;-- AUD2LOW/NR23 (&FF18)
;-- Frequency low byte (W)
;--
#dw rNR23 &FF18
#dw rAUD2LOW rNR23

;--
;-- AUD2HIGH/NR24 (&FF19)
;-- Frequency high byte (W)
;--
;-- see AUD1HIGH for info
;--
#dw rNR24 &FF19
#dw rAUD2HIGH rNR24

;--
;-- AUD3ENA/NR30 (&FF1A)
;-- Sound on/off (R/W)
;--
;-- Bit 7   - Sound ON/OFF (1=ON,0=OFF)
;--
#dw rNR30 &FF1A
#dw rAUD3ENA rNR30

;--
;-- AUD3LEN/NR31 (&FF1B)
;-- Sound length (R/W)
;--
;-- Bit 7-0 - Sound length
;--
#dw rNR31 &FF1B
#dw rAUD3LEN rNR31

;--
;-- AUD3LEVEL/NR32 (&FF1C)
;-- Select output level
;--
;-- Bit 6-5 - Select output level
;--           00: 0/1 (mute)
;--           01: 1/1
;--           10: 1/2
;--           11: 1/4
;--
#dw rNR32 &FF1C
#dw rAUD3LEVEL rNR32

;--
;-- AUD3LOW/NR33 (&FF1D)
;-- Frequency low byte (W)
;--
;-- see AUD1LOW for info
;--
#dw rNR33 &FF1D
#dw rAUD3LOW rNR33

;--
;-- AUD3HIGH/NR34 (&FF1E)
;-- Frequency high byte (W)
;--
;-- see AUD1HIGH for info
;--
#dw rNR34 &FF1E
#dw rAUD3HIGH rNR34

;--
;-- AUD4LEN/NR41 (&FF20)
;-- Sound length (R/W)
;--
;-- Bit 5-0 - Sound length data (# 0-63)
;--
#dw rNR41 &FF20
#dw rAUD4LEN rNR41

;--
;-- AUD4ENV/NR42 (&FF21)
;-- Envelope (R/W)
;--
;-- see AUD1ENV for info
;--
#dw rNR42 &FF21
#dw rAUD4ENV rNR42

;--
;-- AUD4POLY/NR43 (&FF22)
;-- Polynomial counter (R/W)
;--
;-- Bit 7-4 - Selection of the shift clock frequency of the (scf)
;--           polynomial counter (0000-1101)
;--           freq=drf*1/2^scf (not sure)
;-- Bit 3 -   Selection of the polynomial counter's step
;--           0: 15 steps
;--           1: 7 steps
;-- Bit 2-0 - Selection of the dividing ratio of frequencies (drf)
;--           000: f/4   001: f/8   010: f/16  011: f/24
;--           100: f/32  101: f/40  110: f/48  111: f/56  (f=4.194304 Mhz)
;--
#dw rNR43 &FF22
#dw rAUD4POLY rNR43

;--
;-- AUD4GO/NR44 (&FF23)
;--
;-- Bit 7 -   Inital
;-- Bit 6 -   Counter/consecutive selection
;--
#dw rNR44 &FF23
#dw rAUD4GO rNR44

;--
;-- AUDVOL/NR50 (&FF24)
;-- Channel control / ON-OFF / Volume (R/W)
;--
;-- Bit 7   - Vin->SO2 ON/OFF (Vin??)
;-- Bit 6-4 - SO2 output level (volume) (# 0-7)
;-- Bit 3   - Vin->SO1 ON/OFF (Vin??)
;-- Bit 2-0 - SO1 output level (volume) (# 0-7)
;--
#dw rNR50 &FF24
#dw rAUDVOL rNR50

#db AUDVOL_VIN_LEFT  %10000000 ;SO2
#db AUDVOL_VIN_RIGHT %00001000 ;SO1

;--
;-- AUDTERM/NR51 (&FF25)
;-- Selection of Sound output terminal (R/W)
;--
;-- Bit 7   - Output sound 4 to SO2 terminal
;-- Bit 6   - Output sound 3 to SO2 terminal
;-- Bit 5   - Output sound 2 to SO2 terminal
;-- Bit 4   - Output sound 1 to SO2 terminal
;-- Bit 3   - Output sound 4 to SO1 terminal
;-- Bit 2   - Output sound 3 to SO1 terminal
;-- Bit 1   - Output sound 2 to SO1 terminal
;-- Bit 0   - Output sound 0 to SO1 terminal
;--
#dw rNR51 &FF25
#dw rAUDTERM rNR51

;SO2
#db AUDTERM_4_LEFT  %10000000
#db AUDTERM_3_LEFT  %01000000
#db AUDTERM_2_LEFT  %00100000
#db AUDTERM_1_LEFT  %00010000
;SO1
#db AUDTERM_4_RIGHT %00001000
#db AUDTERM_3_RIGHT %00000100
#db AUDTERM_2_RIGHT %00000010
#db AUDTERM_1_RIGHT %00000001

;--
;-- AUDENA/NR52 (&FF26)
;-- Sound on/off (R/W)
;--
;-- Bit 7   - All sound on/off (sets all audio regs to 0!)
;-- Bit 3   - Sound 4 ON flag (read only)
;-- Bit 2   - Sound 3 ON flag (read only)
;-- Bit 1   - Sound 2 ON flag (read only)
;-- Bit 0   - Sound 1 ON flag (read only)
;--
#dw rNR52 &FF26
#dw rAUDENA rNR52

#db AUDENA_ON    %10000000
#db AUDENA_OFF   %00000000  ;sets all audio regs to 0!

;--
;-- LCDC (&FF40)
;-- LCD Control (R/W)
;--
#dw rLCDC &FF40

#db LCDCF_OFF     %00000000 ;LCD Control Operation
#db LCDCF_ON      %10000000 ;LCD Control Operation
#db LCDCF_WIN9800 %00000000 ;Window Tile Map Display Select
#db LCDCF_WIN9C00 %01000000 ;Window Tile Map Display Select
#db LCDCF_WINOFF  %00000000 ;Window Display
#db LCDCF_WINON   %00100000 ;Window Display
#db LCDCF_BG8800  %00000000 ;BG & Window Tile Data Select
#db LCDCF_BG8000  %00010000 ;BG & Window Tile Data Select
#db LCDCF_BG9800  %00000000 ;BG Tile Map Display Select
#db LCDCF_BG9C00  %00001000 ;BG Tile Map Display Select
#db LCDCF_OBJ8    %00000000 ;OBJ Construction
#db LCDCF_OBJ16   %00000100 ;OBJ Construction
#db LCDCF_OBJOFF  %00000000 ;OBJ Display
#db LCDCF_OBJON   %00000010 ;OBJ Display
#db LCDCF_BGOFF   %00000000 ;BG Display
#db LCDCF_BGON    %00000001 ;BG Display
;"Window Character Data Select" follows BG

;--
;-- STAT (&FF41)
;-- LCDC Status   (R/W)
;--
#dw rSTAT &FF41

#db STATF_LYC     %01000000 ;LYC=LY Coincidence (Selectable)
#db STATF_MODE10  %00100000 ;Mode 10
#db STATF_MODE01  %00010000 ;Mode 01 (V-Blank)
#db STATF_MODE00  %00001000 ;Mode 00 (H-Blank)
#db STATF_LYCF    %00000100 ;Coincidence Flag
#db STATF_HBL     %00000000 ;H-Blank
#db STATF_VBL     %00000001 ;V-Blank
#db STATF_OAM     %00000010 ;OAM-RAM is used by system
#db STATF_LCD     %00000011 ;Both OAM and VRAM used by system
#db STATF_BUSY    %00000010 ;When set, VRAM access is unsafe

;--
;-- SCY (&FF42)
;-- Scroll Y (R/W)
;--
#dw rSCY &FF42

;--
;-- SCX (&FF43)
;-- Scroll X (R/W)
;--
#dw rSCX &FF43

;--
;-- LY (&FF44)
;-- LCDC Y-Coordinate (R)
;--
;-- Values range from 0->153. 144->153 is the VBlank period.
;--
#dw rLY &FF44

;--
;-- LYC (&FF45)
;-- LY Compare (R/W)
;--
;-- When LY==LYC, STATF_LYCF will be set in STAT
;--
#dw rLYC &FF45

;--
;-- DMA (&FF46)
;-- DMA Transfer and Start Address (W)
;--
#dw rDMA &FF46

;--
;-- BGP (&FF47)
;-- BG Palette Data (W)
;--
;-- Bit 7-6 - Intensity for %11
;-- Bit 5-4 - Intensity for %10
;-- Bit 3-2 - Intensity for %01
;-- Bit 1-0 - Intensity for %00
;--
#dw rBGP &FF47

;--
;-- OBP0 (&FF48)
;-- Object Palette 0 Data (W)
;--
;-- See BGP for info
;--
#dw rOBP0 &FF48

;--
;-- OBP1 (&FF49)
;-- Object Palette 1 Data (W)
;--
;-- See BGP for info
;--
#dw rOBP1 &FF49

;--
;-- WY (&FF4A)
;-- Window Y Position (R/W)
;--
;-- 0 <= WY <= 143
;-- When WY = 0, the window is displayed from the top edge of the LCD screen.
;--
#dw rWY &FF4A

;--
;-- WX (&FF4B)
;-- Window X Position (R/W)
;--
;-- 7 <= WX <= 166
;-- When WX = 7, the window is displayed from the left edge of the LCD screen.
;-- Values of 0-6 and 166 are unreliable due to hardware bugs.
;--
#dw rWX &FF4B

;--
;-- SPEED (&FF4D)
;-- Select CPU Speed (R/W)
;--
#dw rKEY1 &FF4D
#dw rSPD  rKEY1

#db KEY1F_DBLSPEED %10000000 ;0=Normal Speed, 1=Double Speed (R)
#db KEY1F_PREPARE  %00000001 ;0=No, 1=Prepare (R/W)

;--
;-- VBK (&FF4F)
;-- Select Video RAM Bank (R/W)
;--
;-- Bit 0 - Bank Specification (0: Specify Bank 0; 1: Specify Bank 1)
;--
#dw rVBK &FF4F

;--
;-- HDMA1 (&FF51)
;-- High byte for Horizontal Blanking/General Purpose DMA source address (W)
;-- CGB Mode Only
;--
#dw rHDMA1 &FF51

;--
;-- HDMA2 (&FF52)
;-- Low byte for Horizontal Blanking/General Purpose DMA source address (W)
;-- CGB Mode Only
;--
#dw rHDMA2 &FF52

;--
;-- HDMA3 (&FF53)
;-- High byte for Horizontal Blanking/General Purpose DMA destination address (W)
;-- CGB Mode Only
;--
#dw rHDMA3 &FF53

;--
;-- HDMA4 (&FF54)
;-- Low byte for Horizontal Blanking/General Purpose DMA destination address (W)
;-- CGB Mode Only
;--
#dw rHDMA4 &FF54

;--
;-- HDMA5 (&FF55)
;-- Transfer length (in tiles minus 1)/mode/start for Horizontal Blanking, General Purpose DMA (R/W)
;-- CGB Mode Only
;--
#dw rHDMA5 &FF55

#db HDMA5F_MODE_GP  %00000000 ;General Purpose DMA (W)
#db HDMA5F_MODE_HBL %10000000 ;HBlank DMA (W)

;-- Once DMA has started, use HDMA5F_BUSY to check when the transfer is complete
#db HDMA5F_BUSY %10000000 ;0=Busy (DMA still in progress), 1=Transfer complete (R)

;--
;-- RP (&FF56)
;-- Infrared Communications Port (R/W)
;-- CGB Mode Only
;--
#dw rRP &FF56

#db RPF_ENREAD   %11000000
#db RPF_DATAIN   %00000010 ;0=Receiving IR Signal, 1=Normal
#db RPF_WRITE_HI %00000001
#db RPF_WRITE_LO %00000000

;--
;-- BCPS (&FF68)
;-- Background Color Palette Specification (R/W)
;--
#dw rBCPS &FF68

BCPSF_AUTOINC %10000000 ;Auto Increment (0=Disabled, 1=Increment after Writing)

;--
;-- BCPD (&FF69)
;-- Background Color Palette Data (R/W)
;--
#dw rBCPD &FF69

;--
;-- OCPS (&FF6A)
;-- Object Color Palette Specification (R/W)
;--
#dw rOCPS &FF6A

#db OCPSF_AUTOINC %10000000 ;Auto Increment (0=Disabled, 1=Increment after Writing)

;--
;-- OCPD (&FF6B)
;-- Object Color Palette Data (R/W)
;--
#dw rOCPD &FF6B

;--
;-- SMBK/SVBK (&FF70)
;-- Select Main RAM Bank (R/W)
;--
;-- Bit 2-0 - Bank Specification (0,1: Specify Bank 1; 2-7: Specify Banks 2-7)
;--
#dw rSVBK &FF70
#dw rSMBK rSVBK

;--
;-- PCM12 (&FF76)
;-- Sound channel 1&2 PCM amplitude (R)
;--
;-- Bit 7-4 - Copy of sound channel 2's PCM amplitude
;-- Bit 3-0 - Copy of sound channel 1's PCM amplitude
;--
#dw rPCM12 &FF76

;--
;-- PCM34 (&FF77)
;-- Sound channel 3&4 PCM amplitude (R)
;--
;-- Bit 7-4 - Copy of sound channel 4's PCM amplitude
;-- Bit 3-0 - Copy of sound channel 3's PCM amplitude
;--
#dw rPCM34 &FF77

;--
;-- IE (&FFFF)
;-- Interrupt Enable (R/W)
;--
#dw rIE &FFFF

#db IEF_HILO   %00010000 ;Transition from High to Low of Pin number P10-P13
#db IEF_SERIAL %00001000 ;Serial I/O transfer end
#db IEF_TIMER  %00000100 ;Timer Overflow
#db IEF_LCDC   %00000010 ;LCDC (see STAT)
#db IEF_VBLANK %00000001 ;V-Blank

;***************************************************************************
;*
;* Flags common to multiple sound channels
;*
;***************************************************************************

;--
;-- Square wave duty cycle
;--
;-- Can be used with AUD1LEN and AUD2LEN
;-- See AUD1LEN for more info
;--
#db AUDLEN_DUTY_12_5    %00000000 ;12.5%
#db AUDLEN_DUTY_25      %01000000 ;25%
#db AUDLEN_DUTY_50      %10000000 ;50%
#db AUDLEN_DUTY_75      %11000000 ;75%


;--
;-- Audio envelope flags
;--
;-- Can be used with AUD1ENV, AUD2ENV, AUD4ENV
;-- See AUD1ENV for more info
;--
#db AUDENV_UP           %00001000
#db AUDENV_DOWN         %00000000


;--
;-- Audio trigger flags
;--
;-- Can be used with AUD1HIGH, AUD2HIGH, AUD3HIGH
;-- See AUD1HIGH for more info
;--

#db AUDHIGH_RESTART     %10000000
#db AUDHIGH_LENGTH_ON   %01000000
#db AUDHIGH_LENGTH_OFF  %00000000


;***************************************************************************
;*
;* CPU values on bootup (a=type, b=qualifier)
;*
;***************************************************************************

#db BOOTUP_A_DMG    &01 ;Dot Matrix Game
#db BOOTUP_A_CGB    &11 ;Color GameBoy
#db BOOTUP_A_MGB    &FF ;Mini GameBoy (Pocket GameBoy)

;if a=BOOTUP_A_CGB, bit 0 in b can be checked to determine if real CGB or
;other system running in GBC mode
#db BOOTUP_B_CGB    %00000000
#db BOOTUP_B_AGB    %00000001   ;GBA, GBA SP, Game Boy Player, or New GBA SP


;***************************************************************************
;*
;* Cart related
;*
;***************************************************************************

;&0143 Color GameBoy compatibility code
#db CART_COMPATIBLE_DMG     &00
#db CART_COMPATIBLE_DMG_GBC &80
#db CART_COMPATIBLE_GBC     &C0

;&0146 GameBoy/Super GameBoy indicator
#db CART_INDICATOR_GB       &00
#db CART_INDICATOR_SGB      &03

;&0147 Cartridge type
#db CART_ROM                     &00
#db CART_ROM_MBC1                &01
#db CART_ROM_MBC1_RAM            &02
#db CART_ROM_MBC1_RAM_BAT        &03
#db CART_ROM_MBC2                &05
#db CART_ROM_MBC2_BAT            &06
#db CART_ROM_RAM                 &08
#db CART_ROM_RAM_BAT             &09
#db CART_ROM_MMM01               &0B
#db CART_ROM_MMM01_RAM           &0C
#db CART_ROM_MMM01_RAM_BAT       &0D
#db CART_ROM_MBC3_BAT_RTC        &0F
#db CART_ROM_MBC3_RAM_BAT_RTC    &10
#db CART_ROM_MBC3                &11
#db CART_ROM_MBC3_RAM            &12
#db CART_ROM_MBC3_RAM_BAT        &13
#db CART_ROM_MBC5                &19
#db CART_ROM_MBC5_BAT            &1A
#db CART_ROM_MBC5_RAM_BAT        &1B
#db CART_ROM_MBC5_RUMBLE         &1C
#db CART_ROM_MBC5_RAM_RUMBLE     &1D
#db CART_ROM_MBC5_RAM_BAT_RUMBLE &1E
#db CART_ROM_MBC7_RAM_BAT_GYRO   &22
#db CART_ROM_POCKET_CAMERA       &FC
#db CART_ROM_BANDAI_TAMA5        &FD
#db CART_ROM_HUDSON_HUC3         &FE
#db CART_ROM_HUDSON_HUC1         &FF

;&0148 ROM size
;these are kilobytes
#db CART_ROM_32KB   &00 ;2 banks
#db CART_ROM_64KB   &01 ;4 banks
#db CART_ROM_128KB  &02 ;8 banks
#db CART_ROM_256KB  &03 ;16 banks
#db CART_ROM_512KB  &04 ;32 banks
#db CART_ROM_1024KB &05 ;64 banks
#db CART_ROM_2048KB &06 ;128 banks
#db CART_ROM_4096KB &07 ;256 banks
#db CART_ROM_8192KB &08 ;512 banks
#db CART_ROM_1152KB &52 ;72 banks
#db CART_ROM_1280KB &53 ;80 banks
#db CART_ROM_1536KB &54 ;96 banks

;&0149 SRAM size
;these are kilobytes
#db CART_SRAM_NONE  0
#db CART_SRAM_2KB   1 ;1 incomplete bank
#db CART_SRAM_8KB   2 ;1 bank
#db CART_SRAM_32KB  3 ;4 banks
#db CART_SRAM_128KB 4 ;16 banks

#db CART_SRAM_ENABLE  &0A
#db CART_SRAM_DISABLE &00

;&014A Destination code
#db CART_DEST_JAPANESE     &00
#db CART_DEST_NON_JAPANESE &01

;***************************************************************************
;*
;* Keypad related
;*
;***************************************************************************

#db PADF_DOWN   &80
#db PADF_UP     &40
#db PADF_LEFT   &20
#db PADF_RIGHT  &10
#db PADF_START  &08
#db PADF_SELECT &04
#db PADF_B      &02
#db PADF_A      &01

#db PADB_DOWN   &7
#db PADB_UP     &6
#db PADB_LEFT   &5
#db PADB_RIGHT  &4
#db PADB_START  &3
#db PADB_SELECT &2
#db PADB_B      &1
#db PADB_A      &0

;***************************************************************************
;*
;* Screen related
;*
;***************************************************************************

#db SCRN_X    160 ;Width of screen in pixels
#db SCRN_Y    144 ;Height of screen in pixels
#db SCRN_X_B  20  ;Width of screen in bytes
#db SCRN_Y_B  18  ;Height of screen in bytes

#db SCRN_VX   256 ;Virtual width of screen in pixels
#db SCRN_VY   256 ;Virtual height of screen in pixels
#db SCRN_VX_B 32  ;Virtual width of screen in bytes
#db SCRN_VY_B 32  ;Virtual height of screen in bytes

;***************************************************************************
;*
;* OAM related
;*
;***************************************************************************

;OAM attributes
;each entry in OAM RAM is 4 bytes (sizeof_OAM_ATTRS)
#db RSRESET
#db OAMA_Y              1   ;y pos
#db OAMA_X              1   ;x pos
#db OAMA_TILEID         1   ;tile id
#db OAMA_FLAGS          1   ;flags (see below)
#db sizeof_OAM_ATTRS    0

#db OAM_COUNT           40  ;number of OAM entries in OAM RAM

;flags
#db OAMF_PRI        %10000000 ;Priority
#db OAMF_YFLIP      %01000000 ;Y flip
#db OAMF_XFLIP      %00100000 ;X flip
#db OAMF_PAL0       %00000000 ;Palette number; 0,1 (DMG)
#db OAMF_PAL1       %00010000 ;Palette number; 0,1 (DMG)
#db OAMF_BANK0      %00000000 ;Bank number; 0,1 (GBC)
#db OAMF_BANK1      %00001000 ;Bank number; 0,1 (GBC)

#db OAMF_PALMASK    %00000111 ;Palette (GBC)

#db OAMB_PRI        7 ;Priority
#db OAMB_YFLIP      6 ;Y flip
#db OAMB_XFLIP      5 ;X flip
#db OAMB_PAL1       4 ;Palette number; 0,1 (DMG)
#db OAMB_BANK1      3 ;Bank number; 0,1 (GBC)

;*
;* Nintendo scrolling logo
;* (Code won't work on a real GameBoy)
;* (if next lines are altered.)
#macro nintendo_logo.
    &CE &ED &66 &66 &CC &0D &00 &0B &03 &73 &00 &83 &00 &0C &00 &0D
    &00 &08 &11 &1F &88 &89 &00 &0E &DC &CC &6E &E6 &DD &DD &D9 &99
    &BB &BB &67 &63 &6E &0E &EC &CC &DD &DC &99 &9F &BB &B9 &33 &3E
#macro
