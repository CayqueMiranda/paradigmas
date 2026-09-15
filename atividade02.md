---
title: "Evolução das Principais Linguagens de Programação"
subtitle: "Lista de Exercícios Resolvida"
---

**1. A genealogia das linguagens não é uma escada de progresso.**

Dizer isso significa que uma linguagem mais recente não é automaticamente "melhor" ou "superior" às anteriores num sentido absoluto; ela é uma resposta a um contexto específico (hardware disponível, domínio de aplicação, comunidade de usuários). Linguagens antigas continuam em uso e continuam influenciando novas linguagens mesmo depois de "superadas" cronologicamente.

Dois fatores históricos que explicam essa influência sem substituição:

- **Legado de código e investimento institucional**: sistemas críticos escritos em COBOL ou Fortran continuam em produção porque reescrever é caro e arriscado, então a linguagem "antiga" convive com as novas.
- **Transferência de conceitos, não de sintaxe**: uma linguagem pode influenciar outra apenas por introduzir uma ideia (por exemplo, recursão em Lisp, ou blocos estruturados em ALGOL) que reaparece reformulada em linguagens posteriores, sem que a linguagem original precise desaparecer.

---

**2. Plankalkül e sua relevância histórica.**

Plankalkül, projetada por Konrad Zuse na década de 1940, é relevante porque antecipa conceitos de programação estruturada muito antes de existir hardware ou compiladores capazes de executá-la. Ela nunca foi implementada em sua época por falta de um computador compatível e de um compilador.

Três recursos antecipados:

- Tipos de dados estruturados (arrays e registros), muito antes de FORTRAN ou COBOL.
- Atribuição e estruturas de controle com uma notação formal, prefigurando a ideia de algoritmo escrito independentemente da máquina.
- Um mecanismo de invariantes/asserções, aproximando-se de ideias posteriores de verificação de programas.

O valor do primeiro (tipos estruturados) está em mostrar que, mesmo sem hardware pronto, era possível pensar a programação em termos de organização de dados complexos — uma preocupação que só reapareceria de forma prática décadas depois.

---

**3. Short Code, Speedcoding e A-0/A-1/A-2.**

Short Code (Mauchly, ~1949) era um "pseudocódigo" interpretado: expressões matemáticas codificadas em dígitos, executadas por um interpretador — não gerava código de máquina, apenas interpretava a cada execução, o que a tornava lenta.

Speedcoding (IBM, 1953) era um interpretador para a IBM 701 com instruções de ponto flutuante, priorizando facilidade de uso à custa de desempenho, já que também interpretava em vez de compilar.

Os sistemas A-0/A-1/A-2 (Grace Hopper) já geravam código de máquina a partir de sub-rotinas pré-escritas chamadas por número, sendo o passo mais próximo da compilação real, embora ainda limitados a montar blocos existentes em vez de traduzir uma linguagem de alto nível completa.

Chamá-los simplesmente de "compiladores modernos" seria impreciso porque nenhum deles fazia tradução geral de uma linguagem algorítmica para código de máquina otimizado; eram estágios intermediários — interpretação ou montagem de sub-rotinas — que prepararam o terreno conceitual para o que viria com Fortran.

---

**4. Por que o projeto Fortran precisou provar seu valor.**

Na época (meados dos anos 1950), programadores desconfiavam que um tradutor automático pudesse gerar código tão eficiente quanto o escrito manualmente em assembly, pois máquina era cara e tempo de CPU era o recurso mais escasso. Se o código gerado fosse ineficiente, o ganho em produtividade do programador não compensaria a perda em desempenho da máquina.

A equipe de John Backus investiu pesadamente em um compilador otimizador para provar que a tradução automática podia gerar código quase tão rápido quanto o manual. Isso reduzia o custo de programação (menos tempo e menos erros) sem sacrificar desempenho, o que foi decisivo para a adoção: só assim gerentes e programadores aceitariam abandonar a escrita direta em assembly.

---

**5. Lisp comparada a Fortran.**

Fortran nasceu no domínio do cálculo científico e numérico, com dados representados principalmente por arrays e variáveis escalares, favorecendo um estilo imperativo baseado em laços e atribuições sequenciais.

Lisp nasceu no domínio da inteligência artificial (manipulação simbólica, prova de teoremas, processamento de listas), representando dados de forma recursiva através de listas ligadas (pares car/cdr), e favorecendo um estilo funcional, onde funções recursivas e a própria representação de código como dado (homoiconicidade) são centrais, em vez de laços e memória mutável explícita.

---

**6. Contribuições de ALGOL 60.**

- Introduziu a notação BNF para especificar formalmente a sintaxe, criando um padrão para descrever linguagens que perdura até hoje.
- Consolidou o conceito de bloco estruturado (begin...end) com escopo léxico, base da programação estruturada adotada por praticamente todas as linguagens imperativas posteriores.
- Formalizou a passagem de parâmetros (por valor e por referência/nome), influenciando o design de chamadas de função em linguagens futuras.

Uma linguagem pode ser muito influente sem dominar o mercado porque sua influência se dá pela adoção de suas ideias por outras linguagens (que viram sucesso comercial), e não pela adoção direta da linguagem original pelas empresas. ALGOL era vista como acadêmica e pouco voltada para aplicações comerciais, mas moldou o "vocabulário" conceitual de gerações de linguagens.

---

**7. COBOL e o processamento comercial.**

COBOL foi projetada para ser lida por gerentes não-programadores e para processar grandes volumes de dados empresariais (folhas de pagamento, estoques, faturamento). Isso levou a uma sintaxe verbal, quase em inglês (ADD A TO B GIVING C), priorizando legibilidade sobre concisão matemática.

Os registros (records) em COBOL refletem diretamente a estrutura de um formulário ou arquivo comercial, com campos hierárquicos (nível 01, 05, 10...), o que era natural para o domínio de negócios, diferente das estruturas matemáticas de Fortran.

A relação com FLOW-MATIC (linguagem de Grace Hopper) é direta: COBOL herdou dela a ideia central de comandos em inglês estruturado e a orientação para processamento de arquivos comerciais, sendo FLOW-MATIC uma das principais influências de projeto do comitê CODASYL.

---

**8. Basic e PL/I como respostas de ampliação.**

Basic (Dartmouth, 1964) buscou ampliar o *acesso* à programação: era simples, interpretada, voltada a estudantes não especialistas, priorizando facilidade de aprendizado sobre desempenho ou recursos avançados.

PL/I (IBM, 1964) buscou ampliar o *alcance*: tentou unificar em uma única linguagem os recursos de Fortran (cálculo científico) e COBOL (processamento comercial), além de recursos de sistemas, visando um público muito mais amplo de aplicações.

O compromisso em cada caso: Basic trocou poder e generalidade por simplicidade e acessibilidade; PL/I trocou simplicidade por abrangência, tornando-se uma linguagem grande e complexa, difícil de implementar e aprender por completo.

---

**9. APL, SNOBOL e SIMULA 67.**

- **APL** (Iverson) foi focada em notação matemática concisa para manipulação de arrays/matrizes; contribuição duradoura: operadores vetoriais que influenciaram linguagens de computação científica e array-oriented modernas (como R e NumPy conceitualmente).
- **SNOBOL** foi focada em processamento de strings e casamento de padrões; contribuição duradoura: popularizou a ideia de correspondência de padrões (pattern matching) que reaparece em expressões regulares e linguagens de texto.
- **SIMULA 67** foi focada em simulação de eventos discretos; contribuição duradoura: introduziu classes e objetos, sendo considerada a origem da programação orientada a objetos, base direta para Smalltalk e C++.

---

**10. Ortogonalidade e ALGOL 68.**

Ortogonalidade é o princípio de que um conjunto relativamente pequeno de construções primitivas pode ser combinado livremente, de forma consistente, sem restrições arbitrárias entre elas — assim como eixos ortogonais podem ser combinados independentemente.

ALGOL 68 levou esse princípio ao extremo, permitindo combinações de tipos e estruturas altamente regulares e simétricas. Regularidade significa que as regras se aplicam de forma uniforme em todos os contextos; simplicidade significa poucas regras e conceitos fáceis de aprender. ALGOL 68 era muito regular, mas isso gerou uma gramática grande e combinações pouco intuitivas.

Uma linguagem muito ortogonal não é automaticamente fácil de usar: a combinação livre de construções pode gerar expressões válidas, porém complexas e difíceis de ler, exigindo do programador entender todas as combinações possíveis — foi exatamente essa complexidade que dificultou a adoção comercial de ALGOL 68.

---

**11. Cadeia de influência ALGOL → Pascal → C, contrastada com Prolog.**

ALGOL 60 introduziu blocos estruturados e tipagem formal; Pascal (Wirth, 1971) simplificou e sistematizou essas ideias com fins didáticos, reforçando tipos fortes e estruturas de controle estruturadas (if/while/case); C (Ritchie, 1972) herdou a estrutura de blocos e controle de fluxo de ALGOL/Pascal, mas priorizou eficiência e proximidade do hardware, com tipagem mais fraca e acesso direto à memória via ponteiros.

Essa linhagem é imperativa: o programador especifica *como* o computador deve alcançar um resultado, passo a passo, através de atribuições e mudanças de estado.

Prolog (linguagem lógica, 1972) contrasta radicalmente: o programador declara fatos e regras (*o quê* é verdadeiro), e o motor de inferência decide *como* encontrar soluções via busca e unificação, sem sequência explícita de comandos de atribuição.

---

**12. Pequena base Prolog em linguagem natural.**

Fatos: "Maria é mãe de João." e "João é pai de Pedro."
Regra: "X é avó de Y se X é mãe de Z e Z é pai de Y."
Consulta: "Quem é avó de Pedro?"

O sistema, ao processar a consulta, tenta unificar Y com Pedro e buscar um Z tal que Z seja pai de Pedro (encontra João) e depois um X tal que X seja mãe de João (encontra Maria), respondendo "Maria".

Isso representa programação lógica, e não mero armazenamento de dados, porque a resposta não estava armazenada explicitamente em lugar nenhum — ela foi *deduzida* pelo motor de inferência a partir da combinação de fatos e regras, através de um processo de busca e unificação, o que é fundamentalmente diferente de uma simples consulta a uma tabela.

---

**13. Ada e requisitos de sistemas críticos.**

Ada foi encomendada pelo Departamento de Defesa dos EUA para substituir centenas de linguagens usadas em sistemas embarcados militares, exigindo alta confiabilidade, manutenibilidade e portabilidade em larga escala.

- **Confiabilidade**: forte verificação de tipos em tempo de compilação e tratamento de exceções reduzem erros que seriam catastróficos em sistemas embarcados (mísseis, aviônica).
- **Tipos**: sistema de tipos rígido, incluindo subtipos com faixas de valores, previne erros sutis de faixa/overflow.
- **Pacotes**: permitem encapsulamento e separação entre interface e implementação, essenciais para desenvolvimento em equipe de sistemas grandes e de longa vida útil.
- **Concorrência**: o mecanismo de tasks embutido na linguagem atende diretamente às necessidades de sistemas embarcados que controlam múltiplos processos físicos simultâneos (sensores, atuadores).

---

**14. Objetos em Smalltalk, C++ e Java.**

Smalltalk (Xerox PARC) foi a primeira linguagem "pura" orientada a objetos: tudo é objeto, mensagens são o único mecanismo de interação, sem compromisso com sintaxe ou paradigmas anteriores.

C++ (Stroustrup) adicionou orientação a objetos como uma extensão de C, mantendo compatibilidade retroativa quase total com C — esse compromisso trouxe desempenho e adoção industrial rápida, mas resultou numa linguagem híbrida, com recursos procedurais e orientados a objetos coexistindo, além de manter riscos como ponteiros não seguros.

Java (Sun) foi orientada a objetos desde o início, mas com foco em portabilidade: a estratégia de compilar para bytecode executado por uma máquina virtual (JVM) — "write once, run anywhere" — a diferenciou tanto de Smalltalk quanto de C++, priorizando segurança e portabilidade em rede sobre desempenho bruto ou proximidade do hardware.

---

**15. Java: da aplicação original à Web.**

Java foi originalmente projetada pela Sun para eletrônicos embarcados (set-top boxes e dispositivos de consumo), com foco em portabilidade entre pequenos processadores diferentes — daí a máquina virtual.

Quando a Web explodiu em popularidade em meados dos anos 1990, a mesma característica de portabilidade (bytecode executável em qualquer navegador com JVM, via applets) tornou-se extremamente valiosa nesse novo contexto, mesmo não tendo sido desenhada para isso.

Isso mostra como uma mudança no contexto tecnológico e de mercado (surgimento da Web) pode reposicionar completamente o valor percebido de uma linguagem: a mesma característica técnica (portabilidade via VM) teve pouco impacto no mercado original e um impacto enorme no novo contexto.

---

**16. Perl, JavaScript, PHP, Python, Ruby e Lua.**

| Linguagem | Domínio inicial | Estruturas de dados | Implementação |
|---|---|---|---|
| Perl | Processamento de texto/relatórios em sistemas Unix | Arrays e hashes nativos, forte suporte a regex | Interpretada, com compilação interna para bytecode |
| JavaScript | Interatividade em páginas Web (cliente) | Objetos dinâmicos e arrays associativos | Interpretada/JIT no navegador |
| PHP | Geração dinâmica de páginas Web (servidor) | Arrays associativos como estrutura universal | Interpretada, embutida em HTML |
| Python | Propósito geral, legibilidade e ensino | Listas, dicionários, tuplas como cidadãos de primeira classe | Interpretada (bytecode para CPython) |
| Ruby | Propósito geral, produtividade do programador | Tudo é objeto, incluindo tipos primitivos | Interpretada, fortemente orientada a objetos |
| Lua | Linguagem de extensão/embarcada (jogos) | Tabelas como única estrutura de dados composta | Interpretada, extremamente leve, projetada para embutir em C |

Apesar de todas serem chamadas de "scripting", cada uma nasceu para resolver um problema distinto (texto, Web cliente, Web servidor, ensino/propósito geral, produtividade, embarcado), o que resultou em filosofias de projeto de dados muito diferentes — generalizar todas como equivalentes ignora essas origens específicas.

---

**17. C# e o ambiente .NET.**

C# foi apresentada pela Microsoft como parte da plataforma .NET, competindo diretamente com Java.

Duas comparações:
- **Gerenciamento de memória**: assim como Java, C# usa coletor de lixo automático (diferente de C++, que exige gerenciamento manual), resolvendo o problema de vazamentos de memória e ponteiros inválidos comuns em C++.
- **Propriedades (properties)**: C# introduziu uma sintaxe nativa de propriedades (get/set) que simplifica o padrão getter/setter manual necessário em Java, resolvendo o problema de verbosidade ao encapsular atributos.

---

**18. XSLT e JSP.**

XSLT recebe como entrada um documento XML e uma folha de estilo com regras de transformação declarativas; seu processamento aplica essas regras (casamento de padrões em templates) para produzir como saída outro documento (XML, HTML ou texto).

JSP recebe como entrada uma página HTML com trechos de código Java embutido; seu processamento executa esse código no servidor a cada requisição; a saída é uma página HTML dinâmica gerada para o cliente.

Ambas podem ser chamadas de linguagens híbridas de marcação e programação porque combinam uma linguagem de marcação (XML/HTML) como estrutura de base com uma linguagem/lógica de programação (regras declarativas de transformação no caso do XSLT, ou código Java imperativo no caso do JSP) embutida nessa estrutura, misturando os dois paradigmas em um único documento.

---

**19. Linha do tempo com oito linguagens e quatro paradigmas.**

1. **Fortran** (1957, imperativo/procedural) → ponto de partida do cálculo numérico eficiente.
2. **Lisp** (1958, funcional) → influência: introduziu recursão e processamento simbólico, moldando linguagens funcionais futuras.
3. **ALGOL 60** (1960, imperativo/estruturado) → influência: forneceu a notação BNF e blocos estruturados usados por praticamente toda a linhagem imperativa seguinte.
4. **SIMULA 67** (1967, orientado a objetos) → influência: introduziu classes/objetos, herdando estrutura de bloco de ALGOL.
5. **Prolog** (1972, lógico) → influência: propôs um paradigma declarativo alternativo ao imperativo, com base em lógica de predicados.
6. **Smalltalk** (1980, orientado a objetos) → influência: generalizou os objetos de SIMULA para um modelo "tudo é objeto" baseado em troca de mensagens.
7. **C++** (1985, híbrido imperativo/OO) → influência: incorporou objetos ao estilo SIMULA/Smalltalk sobre a base sintática de C (linhagem ALGOL).
8. **Java** (1995, orientado a objetos) → influência: simplificou C++, adotou gerenciamento automático de memória e portabilidade via VM, consolidando o modelo OO para aplicações em rede.

Paradigmas representados: imperativo, funcional, orientado a objetos e lógico.

---

**20. Estudo de caso: escolha de tecnologias.**

- **Cálculo científico**: família Fortran/linguagens array-oriented (ou Python com bibliotecas numéricas). Justificativa histórica: Fortran nasceu exatamente para esse domínio e sua linhagem otimizou desempenho numérico por décadas.
- **Regras declarativas**: família Prolog/lógica. Justificativa: linguagens lógicas foram desenhadas para expressar "o que" deve ser verdadeiro, ideal para sistemas de regras de negócio ou inferência.
- **Aplicação Web interativa**: família JavaScript/linguagens de script para Web (front-end) combinada com uma linguagem de servidor como PHP, Python ou Java/C#. Justificativa: JavaScript é a única linguagem nativamente executada em todos os navegadores, herança direta de seu propósito original.
- **Firmware restrito**: família C/linguagens de sistemas de baixo nível (herança de ALGOL → C). Justificativa: proximidade do hardware, controle fino de memória e baixo overhead, essenciais em recursos limitados.

Dois trade-offs:
1. Linguagens de alto nível/declarativas (Prolog, scripts Web) ganham produtividade e legibilidade, mas perdem desempenho bruto e controle fino de recursos, comparadas a C para firmware.
2. Escolher múltiplas famílias de linguagens para o mesmo projeto aumenta a expressividade adequada a cada domínio, mas aumenta a complexidade de integração e o custo de manter equipes com competências diferentes.
