// Start: Push command 'segment: constant index: 111' 

@111
D=A
@SP
A=M
M=D
// // SP = SP +1 

@SP
M=M+1
// End: Push command 'segment: constant index: 111' 



// Start: Push command 'segment: constant index: 333' 

@333
D=A
@SP
A=M
M=D
// // SP = SP +1 

@SP
M=M+1
// End: Push command 'segment: constant index: 333' 



// Start: Push command 'segment: constant index: 888' 

@888
D=A
@SP
A=M
M=D
// // SP = SP +1 

@SP
M=M+1
// End: Push command 'segment: constant index: 888' 



// Start: Pop command 'segment: static index: 8' 

// // SP = SP -1 

@SP
M=M-1
@16
D=A
@R15
M=D
@SP
A=M
D=M
@R15
A=M
M=D
// End: Pop command 'segment: static index: 8' 



// Start: Pop command 'segment: static index: 3' 

// // SP = SP -1 

@SP
M=M-1
@17
D=A
@R15
M=D
@SP
A=M
D=M
@R15
A=M
M=D
// End: Pop command 'segment: static index: 3' 



// Start: Pop command 'segment: static index: 1' 

// // SP = SP -1 

@SP
M=M-1
@18
D=A
@R15
M=D
@SP
A=M
D=M
@R15
A=M
M=D
// End: Pop command 'segment: static index: 1' 



// Start: Push command 'segment: static index: 3' 

@17
D=M
@SP
A=M
M=D
// // SP = SP +1 

@SP
M=M+1
// End: Push command 'segment: static index: 3' 



// Start: Push command 'segment: static index: 1' 

@18
D=M
@SP
A=M
M=D
// // SP = SP +1 

@SP
M=M+1
// End: Push command 'segment: static index: 1' 



// Start: Arithmetic command 'sub' 

// Start: Pop command 'segment: R13 index: 0' 

// // SP = SP -1 

@SP
M=M-1
@R13
D=A
@R15
M=D
@SP
A=M
D=M
@R15
A=M
M=D
// End: Pop command 'segment: R13 index: 0' 



// Start: Pop command 'segment: R14 index: 0' 

// // SP = SP -1 

@SP
M=M-1
@R14
D=A
@R15
M=D
@SP
A=M
D=M
@R15
A=M
M=D
// End: Pop command 'segment: R14 index: 0' 



@R13
D=M
@R14
M=M-D
// Start: Push command 'segment: R14 index: 0' 

@R14
D=M
@SP
A=M
M=D
// // SP = SP +1 

@SP
M=M+1
// End: Push command 'segment: R14 index: 0' 



// End: Arithmetic command 'sub' 



// Start: Push command 'segment: static index: 8' 

@16
D=M
@SP
A=M
M=D
// // SP = SP +1 

@SP
M=M+1
// End: Push command 'segment: static index: 8' 



// Start: Arithmetic command 'add' 

// Start: Pop command 'segment: R13 index: 0' 

// // SP = SP -1 

@SP
M=M-1
@R13
D=A
@R15
M=D
@SP
A=M
D=M
@R15
A=M
M=D
// End: Pop command 'segment: R13 index: 0' 



// Start: Pop command 'segment: R14 index: 0' 

// // SP = SP -1 

@SP
M=M-1
@R14
D=A
@R15
M=D
@SP
A=M
D=M
@R15
A=M
M=D
// End: Pop command 'segment: R14 index: 0' 



@R13
D=M
@R14
M=M+D
// Start: Push command 'segment: R14 index: 0' 

@R14
D=M
@SP
A=M
M=D
// // SP = SP +1 

@SP
M=M+1
// End: Push command 'segment: R14 index: 0' 



// End: Arithmetic command 'add' 



