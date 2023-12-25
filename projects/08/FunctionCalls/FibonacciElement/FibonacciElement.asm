

// Start: Init Code
@256
D=A
@SP
M=D
// Start: Function-Call: Sys.init n_args: 0
@Sys.init$ret0
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
@LCL
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
@ARG
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
@THIS
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
@THAT
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
@SP
D=M
@LCL
M=D
@5
D=D-A
@ARG
M=D
@Sys.init
0;JMP
(Sys.init$ret0)
// End: Function-Call: Sys.init n_args: 0


// End: Init Code


// File: 'Main'


// Start: Function: Main.fibonacci n_vars: 0
(Main.fibonacci)
// End: Function: Main.fibonacci n_vars: 0
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


// Start: Push command 'segment: constant index: 2'
@2
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
// End: Push command 'segment: constant index: 2'


// Start: Arithmetic command 'lt'
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
D=M-D
@100
D;JLT
@R14
M=0
@102
0; JMP
@R14
M=-1
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


// End: Arithmetic command 'lt'


// Start: IF_GOTO label: N_LT_2
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
@N_LT_2
D;JNE
// End: IF_GOTO label: N_LT_2
@N_GE_2
0;JMP
(N_LT_2)
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


(N_GE_2)
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


// Start: Push command 'segment: constant index: 2'
@2
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
// End: Push command 'segment: constant index: 2'


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


// Start: Function-Call: Main.fibonacci n_args: 1
@Main.fibonacci$ret1
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
@LCL
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
@ARG
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
@THIS
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
@THAT
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
@SP
D=M
@LCL
M=D
@6
D=D-A
@ARG
M=D
@Main.fibonacci
0;JMP
(Main.fibonacci$ret1)
// End: Function-Call: Main.fibonacci n_args: 1


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


// Start: Function-Call: Main.fibonacci n_args: 1
@Main.fibonacci$ret2
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
@LCL
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
@ARG
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
@THIS
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
@THAT
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
@SP
D=M
@LCL
M=D
@6
D=D-A
@ARG
M=D
@Main.fibonacci
0;JMP
(Main.fibonacci$ret2)
// End: Function-Call: Main.fibonacci n_args: 1


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


// File: 'Sys'


// Start: Function: Sys.init n_vars: 0
(Sys.init)
// End: Function: Sys.init n_vars: 0
// Start: Push command 'segment: constant index: 4'
@4
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
// End: Push command 'segment: constant index: 4'


// Start: Function-Call: Main.fibonacci n_args: 1
@Main.fibonacci$ret3
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
@LCL
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
@ARG
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
@THIS
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
@THAT
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
@SP
D=M
@LCL
M=D
@6
D=D-A
@ARG
M=D
@Main.fibonacci
0;JMP
(Main.fibonacci$ret3)
// End: Function-Call: Main.fibonacci n_args: 1


(END)
@END
0;JMP
// Start: Exit Code
(EXIT)
@EXIT
0;JMP
// End: Exit Code


