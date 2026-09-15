---
title: "Descrição da Sintaxe e da Semântica"
subtitle: "Lista de Exercícios Resolvida"
---

**1. Sintaxe e semântica**

Sintaxe é a forma de uma linguagem: as regras que determinam quais sequências de símbolos são estruturalmente válidas (como as palavras e pontuação se combinam). Semântica é o significado dessas sequências: o que um programa sintaticamente válido efetivamente faz quando executado.

Exemplo: `x = y + 1;` é sintaticamente válido em C porque segue a gramática de atribuição. Sua semântica é "calcule y + 1 e armazene o resultado na variável x". Já `1 = x + y;` é sintaticamente inválido, pois não se pode atribuir a um literal — isso é um erro de sintaxe, não de significado.

---

**2. Sentenças, lexemas e tokens**

Afirmação incorreta comum: "lexema e token são a mesma coisa, apenas nomes diferentes para a mesma unidade do código-fonte."

Erro conceitual: um lexema é a sequência real de caracteres que aparece no código-fonte (por exemplo, o texto `total`), enquanto um token é a categoria sintática abstrata a que esse lexema pertence (por exemplo, `IDENTIFICADOR`). Vários lexemas diferentes (`total`, `soma`, `x`) podem corresponder ao mesmo token.

Versão corrigida: o lexema é a instância concreta de texto no código; o token é a classe/categoria geral que o analisador léxico atribui a esse lexema, usada pelo analisador sintático para verificar a estrutura da sentença.

---

**3. Reconhecedores e geradores**

Procedimento: para reconhecer se a cadeia `"aab"` pertence à linguagem gerada pela gramática G: S → aS | b, um reconhecedor (como um autômato) processa a entrada símbolo a símbolo, a partir do estado inicial, seguindo as transições definidas pelas regras da gramática.

Trecho analisado: `"aab"`. O reconhecedor consome `a` (aplica S → aS), consome outro `a` (aplica S → aS novamente), e por fim consome `b` (aplica S → b), chegando a um estado de aceitação.

Evidência observada: como a entrada foi totalmente consumida e o reconhecedor terminou em estado de aceitação, conclui-se que `"aab"` pertence à linguagem gerada por G — diferente de um gerador, que produziria cadeias da linguagem aplicando as regras a partir de S, em vez de verificar uma entrada específica.

---

**4. BNF e gramáticas livres de contexto**

Duas interpretações possíveis de BNF (Backus-Naur Form):

- Como **notação de especificação formal**: um conjunto de regras de reescrita (produções) que define precisamente quais sequências de tokens são sintaticamente válidas, servindo de referência para quem projeta a linguagem.
- Como **ferramenta operacional para compiladores**: a mesma notação, mas usada como base para gerar automaticamente analisadores sintáticos (parsers), sendo tratada como entrada de ferramentas como Yacc/Bison.

Critério que distingue as duas interpretações: o propósito de uso — na primeira, BNF é documentação/definição da linguagem voltada a humanos; na segunda, é um artefato processável por máquina que orienta a construção de software (o parser). A gramática livre de contexto subjacente é a mesma; muda o papel que ela desempenha no processo de desenvolvimento.

---

**5. Derivação e árvore sintática**

Situação: verificar se a expressão `2 + 3 * 4` respeita a precedência de operadores esperada por uma gramática.

Análise passo a passo:
1. Parte-se do símbolo inicial `<expr>` e aplica-se a produção `<expr> → <expr> + <termo>`.
2. O `<expr>` à esquerda deriva `2` (um `<termo>` simples), e o `<termo>` à direita deriva `3 * 4` via `<termo> → <termo> * <fator>`.
3. A árvore sintática resultante tem a multiplicação em um nível mais profundo (mais próximo das folhas) que a adição, refletindo que `*` tem precedência sobre `+`.

Justificativa: a estrutura hierárquica da árvore — e não apenas a ordem linear dos tokens — é o que define como a expressão deve ser avaliada, confirmando que a gramática captura corretamente a precedência desejada.

---

**6. Ambiguidade gramatical**

Uma gramática é ambígua quando existe pelo menos uma sentença que pode ser gerada por mais de uma árvore sintática distinta (ou mais de uma derivação mais à esquerda), o que significa que a estrutura — e possivelmente o significado — da sentença não é única.

Exemplo clássico: a gramática `<expr> → <expr> + <expr> | <expr> * <expr> | id` gera a sentença `id + id * id` de duas formas diferentes, dependendo de qual `<expr>` é expandida primeiro — uma árvore agrupa como `id + (id * id)` e outra como `(id + id) * id`, alterando o resultado do cálculo dependendo de como o compilador escolher desambiguar.

---

**7. BNF estendida**

Afirmação incorreta comum: "a EBNF (BNF Estendida) descreve uma classe de linguagens mais poderosa que a BNF original, permitindo expressar gramáticas que a BNF pura não conseguiria."

Erro conceitual: a EBNF não aumenta o poder expressivo formal da gramática — toda gramática em EBNF pode ser reescrita em BNF pura equivalente. A diferença é puramente notacional: EBNF acrescenta símbolos convenientes (como `{}` para repetição, `[]` para opcionalidade, `|` para alternativas dentro da mesma regra) que tornam a gramática mais compacta e legível para humanos.

Versão corrigida: EBNF é uma notação mais concisa e legível para descrever as mesmas gramáticas livres de contexto que a BNF descreve, facilitando a escrita e leitura, mas sem alterar a classe de linguagens que pode ser especificada.

---

**8. Gramáticas de atributos**

Procedimento: considere a regra de produção `<expr> → <expr1> + <termo>` associada à necessidade de calcular o tipo resultante da expressão.

Analisando o trecho `x + y` onde `x` é inteiro e `y` é real: o analisador consulta o atributo `tipo` de `<expr1>` (inteiro) e de `<termo>` (real), aplica uma regra semântica associada à produção (algo como "se os tipos diferem, o tipo resultante é o mais amplo") e atribui `tipo(<expr>) = real`.

Evidência observada: o valor calculado para o atributo `tipo` do símbolo não-terminal `<expr>` no nó pai da árvore, derivado consistentemente dos atributos dos filhos — essa é a evidência de que a gramática de atributos está corretamente associando significado (verificação/inferência de tipos) à estrutura sintática.

---

**9. Atributos sintetizados e herdados**

Duas representações/interpretações:

- **Atributo sintetizado**: seu valor é calculado a partir dos atributos dos nós filhos e "flui" de baixo para cima na árvore (das folhas em direção à raiz) — por exemplo, o tipo de uma expressão calculado a partir dos tipos de suas subexpressões.
- **Atributo herdado**: seu valor é calculado a partir dos atributos do nó pai (ou de nós irmãos à esquerda) e "flui" de cima para baixo (ou lateralmente) — por exemplo, informar a um identificador o tipo declarado que vem de uma declaração anterior na mesma linha.

Critério que as distingue: a direção do fluxo de informação na árvore sintática — sintetizado sobe (filhos → pai), herdado desce ou se propaga lateralmente (pai/irmão → filho).

---

**10. Métodos de semântica dinâmica**

Situação: verificar formalmente se um programa pequeno em uma linguagem imperativa preserva uma propriedade (por exemplo, que uma variável nunca fica negativa) antes de ser usado em um sistema crítico.

Análise passo a passo, usando **semântica axiomática**:
1. Define-se uma asserção de pré-condição antes de cada comando (por exemplo, `{x >= 0}`).
2. Aplicam-se regras de inferência (axiomas) específicas para cada tipo de comando (atribuição, condicional, laço) para derivar a pós-condição resultante.
3. Se a pós-condição final implica a propriedade desejada (`x >= 0` ao término), o programa está formalmente verificado quanto a essa propriedade.

Justificativa: diferente da semântica operacional (que simula a execução passo a passo em uma máquina abstrata) ou da denotacional (que mapeia o programa para funções matemáticas), a semântica axiomática permite raciocinar sobre a correção do programa através de lógica formal, sem necessariamente "executar" nada — o que é especialmente útil em verificação de programas críticos.
