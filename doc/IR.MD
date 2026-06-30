## IR

like mostly IR design of SSA and CFG

```bnf
ident ::= alphabet alphanum
type ::= uN | iN | fN | ptr type | name {type+} | void | never | array
function ::= fn fn_name () -> Type { block+ }
block ::= label(var_decl+): '\n' (stmt '\n')+
stmt ::= var_decl | expr_stmt | branch_stmt | call_stmt
var_decl ::= var = type val
var  ::= ident 
```
