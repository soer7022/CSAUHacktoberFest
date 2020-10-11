from time import time

# Sudoku from
# https://en.wikipedia.org/wiki/Sudoku
S = [5, 3, 0, 0, 7, 0, 0, 0, 0,
     6, 0, 0, 1, 9, 5, 0, 0, 0,
     0, 9, 8, 0, 0, 0, 0, 6, 0,
     8, 0, 0, 0, 6, 0, 0, 0, 3,
     4, 0, 0, 8, 0, 3, 0, 0, 1,
     7, 0, 0, 0, 2, 0, 0, 0, 6,
     0, 6, 0, 0, 0, 0, 2, 8, 0,
     0, 0, 0, 4, 1, 9, 0, 0, 5,
     0, 0, 0, 0, 8, 0, 0, 7, 9]

start = time()

# Single list comprehension solving the sudoku
result = "".join([("-" * (9+4) + "\n" if i == 0 else "") + ("|" if i % 3 == 0 else "") + str(Sc[i][-1]) + ("|\n" if i % 9 == 8 else "") + (("-" * (9+4) + "\n") if i % 27 == 26 else "")
               for zpos in [[i for i, n in enumerate(S) if n == 0]]
               for Sc in [[[n] for n in S]]
               for k in [[0]]
               for p in k
               if p == len(zpos) or  # end
               p == -1 or  # error
               (Sc[zpos[p]][-1] == 9 and Sc[zpos[p]].append(0) is None and k.append(p - 1) is None or  # overflow - reset number, add prev p, and not check for valid
                Sc[zpos[p]].append(Sc[zpos[p]][-1] + 1) or  # increment pos
                len([j for j in [j for j, m in enumerate([Sc[i][-1] for i in range(len(S))]) if m == Sc[zpos[p]][-1] and j != zpos[p]] if
                     j // 9 == zpos[p] // 9 or j % 9 == zpos[p] % 9 or (j // 27 == zpos[p] // 27 and (j % 9) // 3 == (zpos[p] % 9) // 3)]) == 0  # the valid function
                and k.append(p + 1) is None or  # valid, goto next space
                k.append(p))
               and False  # should not return and print - no break
               for i in range(len(S))
               ])
               
print(result)
print("------ Running time", "{0:.2f}".format(time() - start), "seconds ------")
