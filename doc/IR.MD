## IR

like mostly IR design of SSA and CFG

```bnf
type ::= uN | iN | fN | ptr type | name {type+} | void | never | array
function ::= fn fn_name '(' var_decl* ')' '->' type { block+ }
block ::= label'('var_decl+')' ':' (stmt';')+
stmt ::= var_decl | expr | return_stmt  
var_decl ::= var : type = val
fn_name ::= [_|alphabet]+ [alphanum|_|@]*
expr ::= expr op expr | op expr | call_expr | var
op ::= + - * / >> << & | ^ ~
var  ::= 'v' number+
call_expr ::= 'call' fn_name '(' (val|var ',') * ')'
return_stmt ::= var
```
