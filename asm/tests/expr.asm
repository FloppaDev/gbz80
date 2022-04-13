
;TODO force idents to start with a _
; there are too many potential conflicts (registers, flags...).

// ;TODO this should throw 2 errors.

#db N0 -10
#db N1 20 - -10
#db N2 -3 - 1

#db _A 10 + 5
; TODO it finds 0
#db _B (_A + 2) * (10 - 3)
; TODO it finds 0
#db _C (-10 + (56 - 1) * 2) SHR 1

; TODO it finds 0
#dw _D M1 - M0
&100:M0
&109:M1

; Should be the same.
; TODO it finds 0
#db E0 5 * 2 + 3 SHL 1 OR %1000
#db E1 (((5 * 2) + 3) SHL 1) OR %1000

; Circular dependency.
; #db X0 X1
; #db X1 X0

; TODO it finds 0
#db P0 10 + 2 * 1 - 5 + 3
; TODO it finds 0
#db P1 (10 + 2 * 1 - 5 + 3)

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
