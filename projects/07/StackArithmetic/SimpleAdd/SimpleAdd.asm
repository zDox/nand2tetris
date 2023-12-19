// Push command 'segment: constant index: 7' 

@7
D=A
@SP
A=M
M=D
// SP = SP +1
@SP
M=M+1


// Push command 'segment: constant index: 8' 

@8
D=A
@SP
A=M
M=D
// SP = SP +1
@SP
M=M+1


// Arithmetic command 'add' 

// Pop command 'segment: R13 index: 0' 

// SP = SP -1
@SP
M=M-1
@SP
A=M
D=M
@R13
M=D


// Pop command 'segment: R14 index: 0' 

// SP = SP -1
@SP
M=M-1
@SP
A=M
D=M
@R14
M=D


@R13
D=M
@R14
M=M+D
// Push command 'segment: R14 index: 0' 

@R14
D=M
@SP
A=M
M=D
// SP = SP +1
@SP
M=M+1




