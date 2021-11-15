; Fail
(HL)
10 + 10
NZ
add A B C
ld a ()
ld a (hl+NZ)
ld a (hl+10+10)
&5255:test xor a :test2
&5255:test xor a &5300:test2

; Pass
add a b
xor a
ld a (sp+10)
&0201:test
    xor a
&0201:test xor a
