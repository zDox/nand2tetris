// Start: Push command 'segment: constant index: 3030' 

@3030
D=A
@SP
A=M
M=D
// // SP = SP +1 

@SP
M=M+1
// End: Push command 'segment: constant index: 3030' 



// Start: Pop command 'segment: pointer index: 0' 

// // SP = SP -1 

@SP
M=M-1
@3
D=A
@R15
M=D
@SP
A=M
D=M
@R15
A=M
M=D
// End: Pop command 'segment: pointer index: 0' 



// Start: Push command 'segment: constant index: 3040' 

@3040
D=A
@SP
A=M
M=D
// // SP = SP +1 

@SP
M=M+1
// End: Push command 'segment: constant index: 3040' 



// Start: Pop command 'segment: pointer index: 1' 

// // SP = SP -1 

@SP
M=M-1
@3
A=A+1
D=A
@R15
M=D
@SP
A=M
D=M
@R15
A=M
M=D
// End: Pop command 'segment: pointer index: 1' 



// Start: Push command 'segment: constant index: 32' 

@32
D=A
@SP
A=M
M=D
// // SP = SP +1 

@SP
M=M+1
// End: Push command 'segment: constant index: 32' 



// Start: Pop command 'segment: this index: 2' 

// // SP = SP -1 

@SP
M=M-1
@THIS
A=M
D=A
@2
A=A+D
D=A
@R15
M=D
@SP
A=M
D=M
@R15
A=M
M=D
// End: Pop command 'segment: this index: 2' 



// Start: Push command 'segment: constant index: 46' 

@46
D=A
@SP
A=M
M=D
// // SP = SP +1 

@SP
M=M+1
// End: Push command 'segment: constant index: 46' 



// Start: Pop command 'segment: that index: 6' 

// // SP = SP -1 

@SP
M=M-1
@THAT
A=M
D=A
@6
A=A+D
D=A
@R15
M=D
@SP
A=M
D=M
@R15
A=M
M=D
// End: Pop command 'segment: that index: 6' 



// Start: Push command 'segment: pointer index: 0' 

@3
D=M
@SP
A=M
M=D
// // SP = SP +1 

@SP
M=M+1
// End: Push command 'segment: pointer index: 0' 



// Start: Push command 'segment: pointer index: 1' 

@3
A=A+1
D=M
@SP
A=M
M=D
// // SP = SP +1 

@SP
M=M+1
// End: Push command 'segment: pointer index: 1' 



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



// Start: Push command 'segment: this index: 2' 

@THIS
A=M
D=A
@2
A=A+D
D=M
@SP
A=M
M=D
// // SP = SP +1 

@SP
M=M+1
// End: Push command 'segment: this index: 2' 



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



// Start: Push command 'segment: that index: 6' 

@THAT
A=M
D=A
@6
A=A+D
D=M
@SP
A=M
M=D
// // SP = SP +1 

@SP
M=M+1
// End: Push command 'segment: that index: 6' 



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



