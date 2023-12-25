

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


// File: 'Sys'


// Start: Function: Sys.init n_vars: 0
(Sys.init)
// End: Function: Sys.init n_vars: 0
// Start: Push command 'segment: constant index: 4000'
@4000
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
// End: Push command 'segment: constant index: 4000'


// Start: Pop command 'segment: pointer index: 0'
// Start: Increment SP by -1
@SP
M=M-1
// End: Increment SP by -1
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


// Start: Push command 'segment: constant index: 5000'
@5000
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
// End: Push command 'segment: constant index: 5000'


// Start: Pop command 'segment: pointer index: 1'
// Start: Increment SP by -1
@SP
M=M-1
// End: Increment SP by -1
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


// Start: Function-Call: Sys.main n_args: 0
@Sys.main$ret1
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
@Sys.main
0;JMP
(Sys.main$ret1)
// End: Function-Call: Sys.main n_args: 0


// Start: Pop command 'segment: temp index: 1'
// Start: Increment SP by -1
@SP
M=M-1
// End: Increment SP by -1
@5
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
// End: Pop command 'segment: temp index: 1'


(LOOP)
@LOOP
0;JMP
// Start: Function: Sys.main n_vars: 5
(Sys.main)
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


// End: Function: Sys.main n_vars: 5
// Start: Push command 'segment: constant index: 4001'
@4001
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
// End: Push command 'segment: constant index: 4001'


// Start: Pop command 'segment: pointer index: 0'
// Start: Increment SP by -1
@SP
M=M-1
// End: Increment SP by -1
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


// Start: Push command 'segment: constant index: 5001'
@5001
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
// End: Push command 'segment: constant index: 5001'


// Start: Pop command 'segment: pointer index: 1'
// Start: Increment SP by -1
@SP
M=M-1
// End: Increment SP by -1
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


// Start: Push command 'segment: constant index: 200'
@200
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
// End: Push command 'segment: constant index: 200'


// Start: Pop command 'segment: local index: 1'
// Start: Increment SP by -1
@SP
M=M-1
// End: Increment SP by -1
@LCL
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
// End: Pop command 'segment: local index: 1'


// Start: Push command 'segment: constant index: 40'
@40
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
// End: Push command 'segment: constant index: 40'


// Start: Pop command 'segment: local index: 2'
// Start: Increment SP by -1
@SP
M=M-1
// End: Increment SP by -1
@LCL
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
// End: Pop command 'segment: local index: 2'


// Start: Push command 'segment: constant index: 6'
@6
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
// End: Push command 'segment: constant index: 6'


// Start: Pop command 'segment: local index: 3'
// Start: Increment SP by -1
@SP
M=M-1
// End: Increment SP by -1
@LCL
A=M
D=A
@3
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
// End: Pop command 'segment: local index: 3'


// Start: Push command 'segment: constant index: 123'
@123
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
// End: Push command 'segment: constant index: 123'


// Start: Function-Call: Sys.add12 n_args: 1
@Sys.add12$ret2
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
@Sys.add12
0;JMP
(Sys.add12$ret2)
// End: Function-Call: Sys.add12 n_args: 1


// Start: Pop command 'segment: temp index: 0'
// Start: Increment SP by -1
@SP
M=M-1
// End: Increment SP by -1
@5
D=A
@R15
M=D
@SP
A=M
D=M
@R15
A=M
M=D
// End: Pop command 'segment: temp index: 0'


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


// Start: Push command 'segment: local index: 2'
@LCL
A=M
D=A
@2
A=A+D
D=M
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
// End: Push command 'segment: local index: 2'


// Start: Push command 'segment: local index: 3'
@LCL
A=M
D=A
@3
A=A+D
D=M
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
// End: Push command 'segment: local index: 3'


// Start: Push command 'segment: local index: 4'
@LCL
A=M
D=A
@4
A=A+D
D=M
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
// End: Push command 'segment: local index: 4'


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


@ARG
D=M
@SP
M=D+1
@R13
D=M
@1
A=D-A
D=M
@THAT
M=D
@R13
D=M
@2
A=D-A
D=M
@THIS
M=D
@R13
D=M
@3
A=D-A
D=M
@ARG
M=D
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


// Start: Function: Sys.add12 n_vars: 0
(Sys.add12)
// End: Function: Sys.add12 n_vars: 0
// Start: Push command 'segment: constant index: 4002'
@4002
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
// End: Push command 'segment: constant index: 4002'


// Start: Pop command 'segment: pointer index: 0'
// Start: Increment SP by -1
@SP
M=M-1
// End: Increment SP by -1
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


// Start: Push command 'segment: constant index: 5002'
@5002
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
// End: Push command 'segment: constant index: 5002'


// Start: Pop command 'segment: pointer index: 1'
// Start: Increment SP by -1
@SP
M=M-1
// End: Increment SP by -1
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


// Start: Push command 'segment: constant index: 12'
@12
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
// End: Push command 'segment: constant index: 12'


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


@ARG
D=M
@SP
M=D+1
@R13
D=M
@1
A=D-A
D=M
@THAT
M=D
@R13
D=M
@2
A=D-A
D=M
@THIS
M=D
@R13
D=M
@3
A=D-A
D=M
@ARG
M=D
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


