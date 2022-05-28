BitBoard::check_win:
    lea	rax, [rcx + 448]
	cmp	r9b, 1
	cmove	rax, rcx
	mov	r9d, r8d
	mov	r10d, edx
	lea	ecx, [r8 + rdx]
    ; (VEX-encoded) move dword
	vmovd	xmm0, edx
	sub	edx, r8d
	add	edx, 18
	vmovd	xmm1, dword ptr [rax + 4*r9]
    ; Insert dword from r32/m32 and rest from xmm1 into xmm1
	vpinsrd	xmm1, xmm1, dword ptr [rax + 4*r10 + 76], 1
	vpinsrd	xmm1, xmm1, dword ptr [rax + 4*rdx + 152], 2
	vpinsrd	xmm1, xmm1, dword ptr [rax + 4*rcx + 300], 3
	vpinsrd	xmm0, xmm0, r8d, 1
    ; Broadcast the second lowest dword to the higher two
	vpshufd	xmm0, xmm0, 84
    ; xmm0 = r = xmm1.rotate_right_dword(xmm0)
	vprorvd	xmm0, xmm1, xmm0
    ; xmm1 = xmm0 = r
	vmovdqa	xmm1, xmm0
    ; xmm1 = !xmm1
	vpternlogq	xmm1, xmm0, xmm0, 15
    ; xmm2 = -1
	vpcmpeqd	xmm2, xmm2, xmm2
    ; xmm0 = !r - 1
	vpaddd	xmm2, xmm1, xmm2
    ; xmm0 = (!r - 1) & r
    ; leaving only the trailing ones in `r`
	vpand	xmm0, xmm0, xmm2
	vpopcntd	xmm0, xmm0
	vplzcntd	xmm1, xmm1
    ; xmm0 = ((!r - 1) & r).count_ones() + (!r).leading_zeros()
    ;      = r.trailing_ones() + r.leading_ones()
	vpaddd	xmm0, xmm0, xmm1
    ; xmm1 = xmm0[2, 3, 2, 3]
	vpshufd	xmm1, xmm0, 238
    ; xmm0 = [max(xmm0[0], xmm0[2]), max(xmm0[1], xmm0[3]), ..]
	vpmaxud	xmm0, xmm0, xmm1
    ; xmm1 = xmm0[1, 1, 1, 1]
	vpshufd	xmm1, xmm0, 85
    ; xmm1 = [max(xmm0[0], xmm0[1]), ..]
	vpmaxud	xmm0, xmm0, xmm1
    ; eax = xmm0[0]
	vmovd	eax, xmm0
	cmp	eax, 6
	setae	al