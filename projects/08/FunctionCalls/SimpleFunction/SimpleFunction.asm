// Start: Function: SimpleFunction.test n_vars: 2
(SimpleFunction.test)
// Start: Push command 'segment: constant index: 0'
@0
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
// End: Push command 'segment: constant index: 0'


// Start: Push command 'segment: constant index: 0'
@0
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
// End: Push command 'segment: constant index: 0'


// End: Function: SimpleFunction.test n_vars: 2
// Start: Push command 'segment: local index: 0'
@LCL
A=M
D=M
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
// End: Push command 'segment: local index: 0'


// Start: Push command 'segment: local index: 1'
@LCL
A=M
A=A+1
D=M
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
// End: Push command 'segment: local index: 1'


// Start: Arithmetic command 'add'
// Start: Pop command 'segment: R13 index: 0'
// Start: Increment SP by -1
@SP
M=M-1
// End: Increment SP by -1
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
// Start: Increment SP by -1
@SP
M=M-1
// End: Increment SP by -1
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
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
// End: Push command 'segment: R14 index: 0'


// End: Arithmetic command 'add'


// Start: Arithmetic command 'not'
// Start: Pop command 'segment: R13 index: 0'
// Start: Increment SP by -1
@SP
M=M-1
// End: Increment SP by -1
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


@R13
D=M
@R14
M=!D
// Start: Push command 'segment: R14 index: 0'
@R14
D=M
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
// End: Push command 'segment: R14 index: 0'


// End: Arithmetic command 'not'


// Start: Push command 'segment: argument index: 0'
@ARG
A=M
D=M
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
// End: Push command 'segment: argument index: 0'


// Start: Arithmetic command 'add'
// Start: Pop command 'segment: R13 index: 0'
// Start: Increment SP by -1
@SP
M=M-1
// End: Increment SP by -1
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
// Start: Increment SP by -1
@SP
M=M-1
// End: Increment SP by -1
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
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
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
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
// End: Push command 'segment: argument index: 1'


// Start: Arithmetic command 'sub'
// Start: Pop command 'segment: R13 index: 0'
// Start: Increment SP by -1
@SP
M=M-1
// End: Increment SP by -1
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
// Start: Increment SP by -1
@SP
M=M-1
// End: Increment SP by -1
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
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
// End: Push command 'segment: R14 index: 0'


// End: Arithmetic command 'sub'


// Start: Function-Return
@LCL
D=M
@R13
M=D
@5
A=D-A
D=M
@R14
M=D
@SP
AM=M-1
D=M
@ARG
A=M
M=D
@ARG
D=M
@SP
M=D+1
// Reinitate @THAT at frame-0
@R13
D=M
@1
A=D-A
D=M
@THAT
M=D
// Reinitate @THIS at frame-1
@R13
D=M
@2
A=D-A
D=M
@THIS
M=D
// Reinitate @ARG at frame-2
@R13
D=M
@3
A=D-A
D=M
@ARG
M=D
// Reinitate @LCL at frame-3
@R13
D=M
@4
A=D-A
D=M
@LCL
M=D
@R14
A=M
0;JMP
// End: Function-Return


// Start: Exit Code
(EXIT)
@EXIT
0;JMP
// End: Exit Code


