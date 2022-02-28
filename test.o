	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 12, 0
	.globl	_factorial                      ## -- Begin function factorial
	.p2align	4, 0x90
_factorial:                             ## @factorial
	.cfi_startproc
## %bb.0:                               ## %factorial
	pushq	%rbx
	.cfi_def_cfa_offset 16
	.cfi_offset %rbx, -16
	testq	%rdi, %rdi
	je	LBB0_1
## %bb.2:                               ## %finally
	movq	%rdi, %rbx
	decq	%rdi
	callq	_factorial
	imulq	%rbx, %rax
	popq	%rbx
	retq
LBB0_1:                                 ## %then
	movl	$1, %eax
	popq	%rbx
	retq
	.cfi_endproc
                                        ## -- End function
.subsections_via_symbols
