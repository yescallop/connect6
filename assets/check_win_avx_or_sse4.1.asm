; AVX version
BitBoard::check_win:
	lea	rax, [rcx + 448]
	cmp	r9b, 1
	cmove	rax, rcx
	mov	r9d, r8d
	mov	r10d, edx
	mov	ecx, edx
	sub	ecx, r8d
	add	ecx, 18
	add	r8d, edx
	vmovd	xmm0, dword ptr [rax + 4*r9]
	vpinsrd	xmm0, xmm0, dword ptr [rax + 4*r10 + 76], 1
	vpinsrd	xmm0, xmm0, dword ptr [rax + 4*rcx + 152], 2
	vpinsrd	xmm0, xmm0, dword ptr [rax + 4*r8 + 300], 3
	vpsrld	xmm1, xmm0, 1
	vpand	xmm0, xmm1, xmm0
	vpsrld	xmm1, xmm0, 2
	vpand	xmm0, xmm1, xmm0
	vpsrld	xmm1, xmm0, 2
	vptest	xmm0, xmm1
	setne	al
	ret

; SSE4.1 version
BitBoard::check_win:
	lea	rax, [rcx + 448]
	cmp	r9b, 1
	cmove	rax, rcx
	mov	r10d, r8d
	mov	r9d, edx
	mov	ecx, edx
	sub	ecx, r8d
	add	ecx, 18
	add	r8d, edx
	movd	xmm0, dword ptr [rax + 4*r10]
	pinsrd	xmm0, dword ptr [rax + 4*r9 + 76], 1
	pinsrd	xmm0, dword ptr [rax + 4*rcx + 152], 2
	pinsrd	xmm0, dword ptr [rax + 4*r8 + 300], 3
	movdqa	xmm1, xmm0
	psrld	xmm1, 1
	pand	xmm1, xmm0
	movdqa	xmm0, xmm1
	psrld	xmm0, 2
	pand	xmm0, xmm1
	movdqa	xmm1, xmm0
	psrld	xmm1, 2
	ptest	xmm0, xmm1
	setne	al
	ret