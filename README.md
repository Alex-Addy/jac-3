jac: OK, first the terms:
jac: "digit frequencies": the number of times each digit occurs in the base 10 representation of the number. "25" maps to { 2: 1, 5: 1 } for instance
jac: "jac number": a number whos digit frequencies match the digit frequencies of its prime factors. To combine the digit frequencies of the factors, take an elementwise sum
jac: ex: 4 has { 4: 1 }, prime factor is 2 2 so combine digit frequencies is { 2: 2 }
jac: "jac-3": a jac number which has precisely 3 prime factors, ex: 2*2*2 = 8 or 2*3*5 = 30
jac: (assuming those are jac numbers, but they are not :p just to illustrate)
jac: conjecture: all jac-3 numbers have at least one prime factor of 3. If 2*3*5 is a jac number, it is also a jac-3, and agrees with the conjecture that all jac-3's have a prime factor of 3
jac: real example of a jac-3:
jac: !$> factor 1353669
rublets_dev: 1353669: 3 653 691
