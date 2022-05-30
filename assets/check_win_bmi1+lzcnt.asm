BitBoard::check_win:
	mov	r10d, edx
	lea	rax, [rcx + 448]
	cmp	r9b, 1
	cmove	rax, rcx
	mov	ecx, r8d
	mov	edx, dword ptr [rax + 4*rcx]
	mov	ecx, r10d
	ror	edx, cl
	not	edx
	tzcnt	ecx, edx
	lzcnt	edx, edx
	add	edx, ecx
	cmp	edx, 6
	setae	r9b
	mov	ecx, r10d
	mov	edx, dword ptr [rax + 4*rcx + 76]
	mov	ecx, r8d
	ror	edx, cl
	not	edx
	tzcnt	ecx, edx
	lzcnt	edx, edx
	add	edx, ecx
	cmp	edx, 6
	setae	r11b
	mov	ecx, r10d
	sub	ecx, r8d
	add	ecx, 18
	mov	edx, dword ptr [rax + 4*rcx + 152]
	mov	ecx, r8d
	ror	edx, cl
	or	r11b, r9b
	not	edx
	tzcnt	ecx, edx
	lzcnt	edx, edx
	add	edx, ecx
	cmp	edx, 6
	setae	dl
	mov	ecx, r8d
	add	ecx, r10d
	mov	eax, dword ptr [rax + 4*rcx + 300]
	mov	ecx, r8d
	ror	eax, cl
	not	eax
	tzcnt	ecx, eax
	lzcnt	eax, eax
	add	eax, ecx
	cmp	eax, 6
	setae	al
	or	al, dl
	or	al, r11b
	ret