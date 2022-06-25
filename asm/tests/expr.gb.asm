
#db N0 15 + 10                                  ;25
#db N1 20 - 10                                  ;10
#db N2 3 + 4                                    ;7

#db _A 10 + 5                                   ;15
#db _B (_A + 2) * (10 - 3)                      ;119
#db _C (10 + (56 - 1) * 2) SHR 1                ;60

#dw _D M1 - M0                                  ;9 TODO got 256
&100:M0                                         ;TODO
&109:M1                                         ;TODO

; Should be the same.
#db E0 5 * 2 + 3 SHL 1 OR %1000                 ;26
#db E1 (((5 * 2) + 3) SHL 1) OR %1000           ;26

; Circular dependency.
; #db X0 X1
; #db X1 X0

#db P0 10 + 2 * 1 - 5 + 3                       ;10
#db P1 (10 + 2 * 1 - 5 + 3)                     ;10

#db U1 NOT 10
#db U2 10 + NOT 10

#db V1 (10 + (56 - 1) * 2)

; Reserved identifiers /!\
; #db XOR 10
; #db C 10
; #db ld 10

; Indirect circular dependency.
; #db U V
; #db V W
; #db W U

; Dependency list 
; #db U V + W   ; eval V then W
; #db V W       ; eval W 
;
; if #db W U check for U
; if #db W V check for V
;
; -> #db U ... deps = [U]
; -> #db V ... deps = [U,V]
