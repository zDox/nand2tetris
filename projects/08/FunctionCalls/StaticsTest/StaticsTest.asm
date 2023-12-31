

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
D=M
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
@ARG
D=M
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
@THIS
D=M
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
@THAT
D=M
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


// File: 'Class1'


// Start: Function: Class1.set n_vars: 0
(Class1.set)
// End: Function: Class1.set n_vars: 0
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


// Start: Pop command 'segment: static index: 0'
// Start: Increment SP by -1
@SP
M=M-1
// End: Increment SP by -1
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
// End: Pop command 'segment: static index: 0'


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


// Start: Pop command 'segment: static index: 1'
// Start: Increment SP by -1
@SP
M=M-1
// End: Increment SP by -1
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
// End: Pop command 'segment: static index: 1'


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


// Start: Function: Class1.get n_vars: 0
(Class1.get)
// End: Function: Class1.get n_vars: 0
// Start: Push command 'segment: static index: 0'
@16
D=M
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
// End: Push command 'segment: static index: 0'


// Start: Push command 'segment: static index: 1'
@17
D=M
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
// End: Push command 'segment: static index: 1'


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


// File: 'Class2'


// Start: Function: Class2.set n_vars: 0
(Class2.set)
// End: Function: Class2.set n_vars: 0
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


// Start: Pop command 'segment: static index: 0'
// Start: Increment SP by -1
@SP
M=M-1
// End: Increment SP by -1
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
// End: Pop command 'segment: static index: 0'


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


// Start: Pop command 'segment: static index: 1'
// Start: Increment SP by -1
@SP
M=M-1
// End: Increment SP by -1
@19
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


// Start: Function: Class2.get n_vars: 0
(Class2.get)
// End: Function: Class2.get n_vars: 0
// Start: Push command 'segment: static index: 0'
@18
D=M
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
// End: Push command 'segment: static index: 0'


// Start: Push command 'segment: static index: 1'
@19
D=M
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
// End: Push command 'segment: static index: 1'


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


// File: 'Sys'


// Start: Function: Sys.init n_vars: 0
(Sys.init)
// End: Function: Sys.init n_vars: 0
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


// Start: Push command 'segment: constant index: 8'
@8
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
// End: Push command 'segment: constant index: 8'


// Start: Function-Call: Class1.set n_args: 2
@Class1.set$ret1
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
@LCL
D=M
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
@ARG
D=M
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
@THIS
D=M
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
@THAT
D=M
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
@7
D=D-A
@ARG
M=D
@Class1.set
0;JMP
(Class1.set$ret1)
// End: Function-Call: Class1.set n_args: 2


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


// Start: Push command 'segment: constant index: 23'
@23
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
// End: Push command 'segment: constant index: 23'


// Start: Push command 'segment: constant index: 15'
@15
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
// End: Push command 'segment: constant index: 15'


// Start: Function-Call: Class2.set n_args: 2
@Class2.set$ret2
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
@LCL
D=M
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
@ARG
D=M
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
@THIS
D=M
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
@THAT
D=M
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
@7
D=D-A
@ARG
M=D
@Class2.set
0;JMP
(Class2.set$ret2)
// End: Function-Call: Class2.set n_args: 2


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


// Start: Function-Call: Class1.get n_args: 0
@Class1.get$ret3
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
@LCL
D=M
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
@ARG
D=M
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
@THIS
D=M
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
@THAT
D=M
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
@Class1.get
0;JMP
(Class1.get$ret3)
// End: Function-Call: Class1.get n_args: 0


// Start: Function-Call: Class2.get n_args: 0
@Class2.get$ret4
D=A
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
@LCL
D=M
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
@ARG
D=M
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
@THIS
D=M
@SP
A=M
M=D
// Start: Increment SP by +1
@SP
M=M+1
// End: Increment SP by +1
@THAT
D=M
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
@Class2.get
0;JMP
(Class2.get$ret4)
// End: Function-Call: Class2.get n_args: 0


(END)
@END
0;JMP
// Start: Exit Code
(EXIT)
@EXIT
0;JMP
// End: Exit Code


