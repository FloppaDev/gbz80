
" Vim syntax file
" Language: gb
" Maintainer: Flop'q
" Latest Revision: 24 April 2020

if exists("b:current_syntax")
  finish
endif

syn match directives '#if\|#else\|#endif\|#macro\|#db\|#dw\|#include\|#import'
syn keyword flags C NC Z NZ
syn match types contained '[&%]'

syn keyword reserved adc add and bit call ccf cp cpl daa dec di ei halt inc jp jr ld ldh ldi ldd ldhl or pop push res ret rl rla rlc rld rr rra rrc rrca rrd rst sbc scf set sla sll sra srl stop sub swap xor reti rlca nop a b c d e h l af bc de hl sp

syn keyword ops MOD AND OR XOR NOT SHL SHR

syn match identifiers '\<[a-zA-Z_][a-zA-Z0-9_]*\>'

syn match literals '[\s\n]*[&%][a-fA-F0-9][_a-fA-F0-9_]*\>' contains=types
syn match literals '\<[0-9][0-9_]*\>'
syn region literals start=/"/ skip=/\\"/ end=/"/

syn keyword todos contained TODO
syn match comments ';.*$' contains=todos

let b:current_syntax = "gb"

hi def link directives Comment
hi def link flags Keyword
hi def link todos Todo
hi def link comments Comment
hi def link literals Constant
hi def link identifiers Identifier
hi def link ops Keyword

