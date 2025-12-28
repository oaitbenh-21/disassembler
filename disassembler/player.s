sti r1, %:live, %1
and r1, %0, r1
live %1
ld %2, r2
ld %4, r3
fork %:live
sti r1, %:bomb, r2
add r2, r3, r2
zjmp %:bomb
_____________________________
sti r1 %15 %1
and r1 %0 r1
live %1
ld %2 r2
ld %4 r3
fork %-19
sti r1 %0 r2
add r2 r3 r2
zjmp %-11