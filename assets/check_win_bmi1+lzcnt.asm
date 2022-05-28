BitBoard::check_win:
	mov	r10d, edx
	lea	r11, [rcx + 448]
	cmp	r9b, 1
	cmove	r11, rcx
	mov	ecx, r8d
	mov	edx, dword ptr [r11 + 4*rcx]
	mov	ecx, r10d
	ror	edx, cl
	not	edx
	tzcnt	ecx, edx
	lzcnt	edx, edx
	add	edx, ecx
	mov	ecx, r10d
	mov	eax, dword ptr [r11 + 4*rcx + 76]
	mov	ecx, r8d
	ror	eax, cl
	not	eax
	tzcnt	ecx, eax
	lzcnt	eax, eax
	add	eax, ecx
	cmp	edx, eax
	cmova	eax, edx
	mov	ecx, r10d
	sub	ecx, r8d
	add	ecx, 18
	mov	edx, dword ptr [r11 + 4*rcx + 152]
	mov	ecx, r8d
	ror	edx, cl
	not	edx
	tzcnt	ecx, edx
	lzcnt	edx, edx
	add	edx, ecx
	cmp	eax, edx
	cmova	edx, eax
	mov	eax, r8d
	add	eax, r10d
	mov	eax, dword ptr [r11 + 4*rax + 300]
	mov	ecx, r8d
	ror	eax, cl
	not	eax
	tzcnt	ecx, eax
	lzcnt	eax, eax
	add	eax, ecx
	cmp	edx, eax
	cmova	eax, edx
	cmp	eax, 6
	setae	al
	ret