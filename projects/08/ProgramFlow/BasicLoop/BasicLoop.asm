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


// Start: Pop command 'segment: local index: 0'
// Start: Increment SP by -1
@SP
M=M-1
// End: Increment SP by -1
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


(LOOP)
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


// Start: Pop command 'segment: local index: 0'
// Start: Increment SP by -1
@SP
M=M-1
// End: Increment SP by -1
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


// Start: Push command 'segment: constant index: 1'
@1
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
// End: Push command 'segment: constant index: 1'


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


// Start: Pop command 'segment: argument index: 0'
// Start: Increment SP by -1
@SP
M=M-1
// End: Increment SP by -1
@ARG
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
// End: Pop command 'segment: argument index: 0'


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


// Start: IF_GOTO label: LOOP
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


D=M
@LOOP
D;JNE
// End: IF_GOTO label: LOOP
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


// Start: Exit Code
(EXIT)
@EXIT
0;JMP
// End: Exit Code


