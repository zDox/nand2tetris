// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen
// by writing 'black' in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen by writing
// 'white' in every pixel;
// the screen should remain fully clear as long as no key is pressed.

//// Replace this comment with your code.
// init variables
// Load 8192 into pixels
(BEGIN)
@8192
D = A
@pixels
M = D

@i
M = 0

// if key not pressed jump to color screen black
@KBD
D = M
@BLACK
D; JGT

// -- make screen white --

(WHITE)
// if is above screen goto END
@pixels
D = M
@i
D = M - D
@END
D; JGE

// make ith pixel white
@SCREEN
D = A
@i
A = D + M
M = 0

// i = i + 1
@i
M = M + 1

// Jump to begin of white loop
@WHITE
0;JMP

// else make screen black
(BLACK)

// if is above screen goto END
@pixels
D = M
@i
D = M - D
@END
D; JGE

// make ith pixel black
@SCREEN
D = A
@i
A = D + M
M = -1

// i = i + 1
@i
M = M + 1

// Jump to begin of black loop
@BLACK
0;JMP

(END)
@BEGIN
0; JMP
