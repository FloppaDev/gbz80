
#def A 10 + 5
#def B (A + 2) * (10 - 3)
#def C (-10 + (56 - 1) * 2) >> 1
#def D M1 - M0

&100:M0
&109:M1

; Should be the same.
#def E0 5 * 2 + 3 << 1 | %1000
#def E1 ((((5 * 2) + 3)) << 1) | %1000

; Circular dependency.
#def X0 X1
#def X1 X0
