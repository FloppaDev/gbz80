
#db A 10 + 5
#db B (A + 2) * (10 - 3)
#db C (-10 + (56 - 1) * 2) RSH 1

#dw D M1 - M0
&100:M0
&109:M1

; Should be the same.
#db E0 5 * 2 + 3 LSH 1 OR %1000
#db E1 ((((5 * 2) + 3)) LSH 1) OR %1000

; Circular dependency.
#db X0 X1
#db X1 X0
