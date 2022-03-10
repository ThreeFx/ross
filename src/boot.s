.section .init
.globl _start
_start:
	# debugging info: Mark the value stored in ra as garbage
	.cfi_startproc
	.cfi_undefined ra

	# Fetch our hartid into a0
	csrr a0, mhartid

	# If we are not core zero, wait
	bne a0, zero, wait_for_interrupt

	# load the initial stack address
	la sp, _stack_start

	# Store the very bottom of the stack to use for storing interrupt data
	csrw mscratch, sp
	# TODO: protect this memory as well

	# Skip the 1M we reserved before
	li t0, 1024*1024
	add sp, sp, t0

	# Store the interrupt handler address
	la a0, interrupt_handler
	csrw mtvec, a0

	# TODO: drop privileges?

	# Jump to the start of the kernel, without returning here
	# TODO: Should we call into this function here? it should never return
	# anyway...
	j start_kernel
	.cfi_endproc

wait_for_interrupt:
	#wfi
	j wait_for_interrupt
