// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// Assumes that R0 >= 0, R1 >= 0, and R0 * R1 < 32768.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)

// Approach: Add RAM[1] many times RAM[0] to RAM[2]

// Init of variables
@i
M = 0
@R2
M = 0

(LOOP)
    // if i >= RAM[1] goto END
    @R1
    D = M
    @i
    D = M - D 
    @END
    D;JGE

    // Add RAM[0] to RAM[2]
    @R0
    D = M
    @R2
    M = D + M

    // i = i + 1
    @i
    M = M + 1

    // goto LOOP
    @LOOP
    0; JMP

(END)
    @END
    0; JMP
