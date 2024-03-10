# Scanner
Também conhecido como <b>analisador lexico</b>.

Deve produzir uma lista de Tokens.

### Token
Deve ser uma struct com os seguintes parâmetros:

- LEXEME;
- classe gramatical;
- linha;
- coluna.

A classe gramatical será representada pelo enum <b>token_type</b>.

### LEXEME

Os lexemas podem assumir os seguintes valores:

- Literal (um valor qualquer como string);
- program;
- procedure;
- var;
- integer;
- if;
- real;
- boolean;
- begin;
- end;
- else;
- then;
- while;
- do;
- not;
- [a..z] _ [0..9];
- [0..9];
- [0..9].[0..9];
- ;;
- :;
- .;
- ,;
- (;
- );
- :=;
- =;
- \>;
- \>=;
- <;
- <=;
- <>;
- +;
- -;
- or;
- *;
- /;
- and;

### token_type
Apenas um enum, que pode assumir os seguintes valores:

- Keyword;
- identifier;
- integer;
- real;
- delimiter;
- attribution;
- relational operator;
- additive operator;
- multiplicative operator.

### Erros
O scanner deve exibir erros quando uma dessas duas situações ocorrer:

- Comentário em aberto;
- Simbolo desconhecido;


### Automato
![automato](../../adicionais/Automato%20Pascal.drawio.svg)

### Fluxograma
![fluxograma do automato](../../adicionais/fluxograma%20automato.drawio.svg)