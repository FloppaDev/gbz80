
#db N0 -10
#db N1 20 - -10
#db N2 -3 - 1

#db A 10 + 5
#db B (A + 2) * (10 - 3)
#db C (-10 + (56 - 1) * 2) SHR 1

#dw D M1 - M0
&100:M0
&109:M1

; Should be the same.
#db E0 5 * 2 + 3 SHL 1 OR %1000
#db E1 (((5 * 2) + 3) SHL 1) OR %1000

; Circular dependency.
#db X0 X1
#db X1 X0

#db P0 10 + 2 * 1 - 5 + 3
#db P1 (10 + 2 * 1 - 5 + 3)
