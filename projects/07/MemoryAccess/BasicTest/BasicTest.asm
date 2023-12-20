// Start: Push command 'segment: constant index: 10' 

@10
D=A
@SP
A=M
M=D
// // SP = SP +1 

@SP
M=M+1
// End: Push command 'segment: constant index: 10' 



// Start: Pop command 'segment: local index: 0' 

// // SP = SP -1 

@SP
M=M-1
@LCL
A=M
D=A
@R15
M=D
@SP
A=M
D=M
@R15
A=M
M=D
// End: Pop command 'segment: local index: 0' 



// Start: Push command 'segment: constant index: 21' 

@21
D=A
@SP
A=M
M=D
// // SP = SP +1 

@SP
M=M+1
// End: Push command 'segment: constant index: 21' 



// Start: Push command 'segment: constant index: 22' 

@22
D=A
@SP
A=M
M=D
// // SP = SP +1 

@SP
M=M+1
// End: Push command 'segment: constant index: 22' 



// Start: Pop command 'segment: argument index: 2' 

// // SP = SP -1 

@SP
M=M-1
@ARG
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
// End: Pop command 'segment: argument index: 2' 



// Start: Pop command 'segment: argument index: 1' 

// // SP = SP -1 

@SP
M=M-1
@ARG
A=M
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
// End: Pop command 'segment: argument index: 1' 



// Start: Push command 'segment: constant index: 36' 

@36
D=A
@SP
A=M
M=D
// // SP = SP +1 

@SP
M=M+1
// End: Push command 'segment: constant index: 36' 



// Start: Pop command 'segment: this index: 6' 

// // SP = SP -1 

@SP
M=M-1
@THIS
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
// End: Pop command 'segment: this index: 6' 



// Start: Push command 'segment: constant index: 42' 

@42
D=A
@SP
A=M
M=D
// // SP = SP +1 

@SP
M=M+1
// End: Push command 'segment: constant index: 42' 



// Start: Push command 'segment: constant index: 45' 

@45
D=A
@SP
A=M
M=D
// // SP = SP +1 

@SP
M=M+1
// End: Push command 'segment: constant index: 45' 



// Start: Pop command 'segment: that index: 5' 

// // SP = SP -1 

@SP
M=M-1
@THAT
A=M
D=A
@5
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
// End: Pop command 'segment: that index: 5' 



// Start: Pop command 'segment: that index: 2' 

// // SP = SP -1 

@SP
M=M-1
@THAT
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
// End: Pop command 'segment: that index: 2' 



// Start: Push command 'segment: constant index: 510' 

@510
D=A
@SP
A=M
M=D
// // SP = SP +1 

@SP
M=M+1
// End: Push command 'segment: constant index: 510' 



// Start: Pop command 'segment: temp index: 6' 

// // SP = SP -1 

@SP
M=M-1
@5
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
// End: Pop command 'segment: temp index: 6' 



// Start: Push command 'segment: local index: 0' 

@LCL
A=M
D=M
@SP
A=M
M=D
// // SP = SP +1 

@SP
M=M+1
// End: Push command 'segment: local index: 0' 



// Start: Push command 'segment: that index: 5' 

@THAT
A=M
D=A
@5
A=A+D
D=M
@SP
A=M
M=D
// // SP = SP +1 

@SP
M=M+1
// End: Push command 'segment: that index: 5' 



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



// Start: Push command 'segment: argument index: 1' 

@ARG
A=M
A=A+1
D=M
@SP
A=M
M=D
// // SP = SP +1 

@SP
M=M+1
// End: Push command 'segment: argument index: 1' 



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



// Start: Push command 'segment: this index: 6' 

@THIS
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
// End: Push command 'segment: this index: 6' 



// Start: Push command 'segment: this index: 6' 

@THIS
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
// End: Push command 'segment: this index: 6' 



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



// Start: Push command 'segment: temp index: 6' 

@5
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
// End: Push command 'segment: temp index: 6' 



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



