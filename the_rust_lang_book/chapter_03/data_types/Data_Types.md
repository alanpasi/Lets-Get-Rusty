##### Traduzido por deepseek em 16/04/2025
### Tipos de Dados

Todo valor em Rust é de um determinado *tipo de dado*, que informa ao Rust que tipo de dado está sendo especificado para que ele saiba como trabalhar com esses dados. Vamos ver dois subconjuntos de tipos de dados: escalares e compostos.

Tenha em mente que Rust é uma linguagem *estaticamente tipada*, o que significa que ele deve conhecer os tipos de todas as variáveis em tempo de compilação. O compilador geralmente pode inferir qual tipo queremos com base no valor e como o usamos. Nos casos em que muitos tipos são possíveis, como quando convertemos uma `String` para um tipo numérico usando `parse` na seção [“Comparando o Palpite com o Número Secreto”](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number) no Capítulo 2, devemos adicionar uma anotação de tipo, assim:

```rust
let guess: u32 = "42".parse().expect("Não é um número!");
```

Se não adicionarmos a anotação de tipo `: u32` mostrada no código anterior, o Rust exibirá o seguinte erro, que significa que o compilador precisa de mais informações para saber qual tipo queremos usar:

```text
error[E0282]: type annotations needed
 --> src/main.rs:2:9
  |
2 |     let guess = "42".parse().expect("Não é um número!");
  |         ^^^^^ consider giving `guess` a type
```

Você verá anotações de tipo diferentes para outros tipos de dados.

#### Tipos Escalares

Um tipo *escalar* representa um único valor. Rust tem quatro tipos escalares primários: inteiros, números de ponto flutuante, booleanos e caracteres. Você pode reconhecê-los de outras linguagens de programação. Vamos pular para como eles funcionam em Rust.

##### Tipos Inteiros

Um *inteiro* é um número sem componente fracional. Usamos um tipo inteiro no Capítulo 2, o `u32`. Esse tipo de declaração indica que o valor associado deve ser um inteiro sem sinal (tipos inteiros com sinal começam com `i` em vez de `u`) que ocupa 32 bits de espaço. A Tabela 3-1 mostra os tipos inteiros internos ao Rust. Podemos usar qualquer uma dessas variantes para declarar o tipo de um valor inteiro.

| Tamanho | Com Sinal | Sem Sinal |
|---------|-----------|------------|
| 8-bit   | `i8`      | `u8`       |
| 16-bit  | `i16`     | `u16`      |
| 32-bit  | `i32`     | `u32`      |
| 64-bit  | `i64`     | `u64`      |
| 128-bit | `i128`    | `u128`     |
| arch    | `isize`   | `usize`    |

*Tabela 3-1: Tipos inteiros em Rust*

Cada variante pode ser com ou sem sinal e tem um tamanho explícito. *Com sinal* e *sem sinal* referem-se à possibilidade do número ser negativo — em outras palavras, se o número precisa ter um sinal (com sinal) ou se sempre será positivo e, portanto, pode ser representado sem um sinal (sem sinal). É como escrever números no papel: quando o sinal importa, o número é exibido com um sinal de mais ou menos; no entanto, quando é seguro assumir que o número é positivo, ele é exibido sem sinal. Números com sinal são armazenados usando a representação em complemento de dois (se você não tem certeza do que é isso, você pode procurar sobre isso na internet; uma explicação está fora do escopo deste livro).

Cada variante com sinal pode armazenar números de -(2<sup>n - 1</sup>) a 2<sup>n - 1</sup> - 1 inclusive, sendo *n* o número de bits que a variante usa. Então, um `i8` pode armazenar números de -(2<sup>7</sup>) a 2<sup>7</sup> - 1, o que equivale a -128 a 127. Variantes sem sinal podem armazenar números de 0 a 2<sup>n</sup> - 1, então um `u8` pode armazenar números de 0 a 2<sup>8</sup> - 1, que é de 0 a 255.

Além disso, os tipos `isize` e `usize` dependem da arquitetura do computador em que seu programa está rodando, denotados na tabela como "arch": 64 bits se você estiver em uma arquitetura de 64 bits e 32 bits se você estiver em uma arquitetura de 32 bits.

Você pode escrever literais inteiros em qualquer uma das formas mostradas na Tabela 3-2. Note que literais numéricos que podem ser múltiplos tipos numéricos permitem um sufixo de tipo, como `57u8`, para designar o tipo. Literais numéricos também podem usar `_` como separador visual para facilitar a leitura, como `1_000`, que terá o mesmo valor que se você tivesse especificado `1000`.

| Literais Numéricos  | Exemplo       |
|---------------------|---------------|
| Decimal             | `98_222`      |
| Hexadecimal         | `0xff`        |
| Octal               | `0o77`        |
| Binário             | `0b1111_0000` |
| Byte (`u8` apenas)  | `b'A'`        |

*Tabela 3-2: Literais inteiros em Rust*

Então, como você sabe qual tipo de inteiro usar? Se você não tem certeza, as escolhas padrão do Rust geralmente são boas, e por padrão os tipos inteiros são `i32`: esse tipo geralmente é o mais rápido, mesmo em sistemas de 64 bits. A principal situação em que você usaria `isize` ou `usize` é ao indexar algum tipo de coleção.

##### Tipos de Ponto Flutuante

Rust também tem dois tipos primitivos para *números de ponto flutuante*, que são números com casas decimais. Os tipos de ponto flutuante de Rust são `f32` e `f64`, que têm tamanhos de 32 bits e 64 bits, respectivamente. O tipo padrão é `f64` porque em CPUs modernas ele tem velocidade quase igual à do `f32`, mas é capaz de mais precisão.

Aqui está um exemplo que mostra números de ponto flutuante em ação:

```rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```

Números de ponto flutuante são representados de acordo com o padrão IEEE-754. O tipo `f32` é de precisão simples, e `f64` tem precisão dupla.

##### Operações Numéricas

Rust suporta as operações matemáticas básicas que você esperaria para todos os tipos numéricos: adição, subtração, multiplicação, divisão e resto. O código a seguir mostra como você usaria cada uma em uma declaração `let`:

```rust
fn main() {
        // adição
        let sum = 5 + 10;

        // subtração
        let difference = 95.5 - 4.3;

        // multiplicação
        let product = 4 * 30;

        // divisão
        let quotient = 56.7 / 32.2;

        // resto
        let remainder = 43 % 5;
}
```

Cada expressão nessas declarações usa um operador matemático e calcula um único valor, que é então associado a uma variável. O [Apêndice B](https://doc.rust-lang.org/book/appendix-02-operators.html) contém uma lista de todos os operadores que Rust fornece.

##### O Tipo Booleano

Como em muitas outras linguagens de programação, um tipo booleano em Rust tem dois valores possíveis: `true` (verdadeiro) e `false` (falso). Booleanos têm um tamanho de um byte. O tipo booleano em Rust é especificado usando `bool`. Por exemplo:

```rust
fn main() {
    let t = true;

    let f: bool = false; // com tipo explícito
}
```

A principal forma de usar valores booleanos é através de condicionais, como uma expressão `if`. Veremos como a expressão `if` funciona em Rust na seção [“Controle de Fluxo”](https://doc.rust-lang.org/book/ch03-05-control-flow.html#control-flow).

##### O Tipo Caractere

O tipo `char` de Rust é o tipo alfabético mais primitivo da linguagem. Aqui estão alguns exemplos de declaração de valores `char`:

```rust
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}
```

Observe que especificamos literais `char` com aspas simples, em oposição a strings, que usam aspas duplas. O tipo `char` do Rust tem quatro bytes de tamanho e representa um Valor Escalar Unicode (Unicode Scalar Value), o que significa que pode representar muito mais do que apenas ASCII. Caracteres acentuados; caracteres chineses, japoneses e coreanos; emoji; e espaços em branco de largura zero são todos valores `char` válidos em Rust. Valores escalares Unicode variam de `U+0000` a `U+D7FF` e `U+E000` a `U+10FFFF` inclusive. No entanto, um "caractere" não é realmente um conceito em Unicode, então sua intuição humana sobre o que é um "caractere" pode não combinar com o que é um `char` em Rust. Discutiremos esse tópico em detalhes em [“Armazenando Texto com Strings”](https://doc.rust-lang.org/book/ch08-02-strings.html#storing-utf-8-encoded-text-with-strings) no Capítulo 8.

#### Tipos Compostos

*Tipos compostos* podem agrupar múltiplos valores em um único tipo. Rust tem dois tipos compostos primitivos: tuplas e arrays.

##### O Tipo Tupla

Uma *tupla* é uma forma geral de agrupar um número de valores com uma variedade de tipos em um único tipo composto. Tuplas têm um comprimento fixo: uma vez declaradas, elas não podem aumentar ou diminuir de tamanho.

Criamos uma tupla escrevendo uma lista de valores separados por vírgulas dentro de parênteses. Cada posição na tupla tem um tipo, e os tipos dos diferentes valores na tupla não precisam ser os mesmos. Adicionamos anotações de tipo neste exemplo:

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

A variável `tup` liga-se à tupla inteira porque uma tupla é considerada um único elemento composto. Para obter os valores individuais de uma tupla, podemos usar a correspondência de padrões para desestruturar um valor de tupla, assim:

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("O valor de y é: {}", y);
}
```

Esse programa primeiro cria uma tupla e a associa à variável `tup`. Em seguida, ele usa um padrão com `let` para tirar `tup` e transformá-lo em três variáveis separadas, `x`, `y` e `z`. Isso é chamado de *desestruturação* porque quebra a tupla única em três partes. Finalmente, o programa imprime o valor de `y`, que é `6.4`.

Além da desestruturação por correspondência de padrões, podemos acessar diretamente um elemento da tupla usando um ponto (`.`) seguido do índice do valor que queremos acessar. Por exemplo:

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

Esse programa cria a tupla `x` e então acessa cada elemento da tupla usando seus respectivos índices. Como na maioria das linguagens de programação, o primeiro índice em uma tupla é 0.

##### O Tipo Array

Outra forma de ter uma coleção de múltiplos valores é com um *array*. Diferente de uma tupla, todos os elementos de um array devem ser do mesmo tipo. Arrays em Rust são diferentes de arrays em algumas outras linguagens porque arrays em Rust têm um comprimento fixo, como tuplas.

Em Rust, os valores que entram em um array são escritos como uma lista separada por vírgulas dentro de colchetes:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

Arrays são úteis quando você quer que seus dados sejam alocados na stack (pilha) em vez de no heap (pilha dinâmica) (discutiremos mais sobre stack e heap no [Capítulo 4](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#the-stack-and-the-heap)) ou quando quer garantir que sempre terá um número fixo de elementos. No entanto, arrays não são tão flexíveis quanto o tipo vetor (vector). Um vetor é um tipo de coleção similar fornecido pela biblioteca padrão que *pode* crescer ou diminuir de tamanho. Se você não tem certeza se deve usar um array ou um vetor, provavelmente deve usar um vetor. O [Capítulo 8](https://doc.rust-lang.org/book/ch08-01-vectors.html) discute vetores em mais detalhes.

Um exemplo de quando você poderia querer usar um array em vez de um vetor é um programa no qual você precisa saber os nomes dos meses do ano. É improvável que tal programa precise adicionar ou remover meses, então você pode usar um array porque você sabe que sempre conterá 12 itens:

```rust
let months = ["Janeiro", "Fevereiro", "Março", "Abril", "Maio", "Junho", "Julho",
              "Agosto", "Setembro", "Outubro", "Novembro", "Dezembro"];
```

Você escreve o tipo de um array usando colchetes com o tipo de cada elemento, um ponto-e-vírgula e então o número de elementos no array, assim:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

Aqui, `i32` é o tipo de cada elemento. Após o ponto-e-vírgula, o número `5` indica que o array contém cinco elementos.

Você também pode inicializar um array para conter o mesmo valor para cada elemento, especificando o valor inicial, seguido de um ponto-e-vírgula e então o comprimento do array dentro de colchetes, como mostrado aqui:

```rust
let a = [3; 5];
```

O array chamado `a` conterá `5` elementos que terão o valor `3` inicialmente. Isso é o mesmo que escrever `let a = [3, 3, 3, 3, 3];`, mas de uma forma mais concisa.

##### Acessando Elementos do Array

Um array é um pedaço único de memória de tamanho fixo e conhecido que pode ser alocado na stack. Você pode acessar elementos de um array usando indexação, assim:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```

Nesse exemplo, a variável chamada `first` receberá o valor `1` porque esse é o valor no índice `[0]` no array. A variável chamada `second` receberá o valor `2` do índice `[1]` no array.

##### Acesso Inválido a Elementos do Array

O que acontece se você tentar acessar um elemento de um array que está além do fim do array? Digamos que você altere o exemplo para o seguinte código, que será compilado mas sairá com um erro quando executado:

```rust
use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Por favor, digite um índice de array.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Falha ao ler a linha");

    let index: usize = index
        .trim()
        .parse()
        .expect("Índice digitado não é um número");

    let element = a[index];

    println!(
        "O valor do elemento no índice {} é: {}",
        index, element
    );
}
```

Esse código compila com sucesso. Se você executar esse código usando `cargo run` e inserir `0`, `1`, `2`, `3` ou `4`, o programa imprimirá o valor correspondente a esse índice no array. Se, em vez disso, você inserir um número além do fim do array, como `10`, verá uma saída como esta:

```text
thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:19:19
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

O programa resultou em um erro de *runtime* no momento em que utilizou um valor inválido na operação de indexação. O programa saiu com uma mensagem de erro e não executou a declaração `println!` final. Quando você tenta acessar um elemento usando indexação, Rust verifica se o índice que você especificou é menor que o comprimento do array. Se o índice for maior ou igual ao comprimento, Rust entrará em pânico. Essa verificação tem que acontecer em tempo de execução, especialmente nesse caso, porque o compilador não pode saber que valor um usuário irá digitar quando o código for executado mais tarde.

Esse é um exemplo dos princípios de segurança de memória do Rust em ação. Em muitas linguagens de baixo nível, esse tipo de verificação não é feito, e quando você fornece um índice incorreto, memória inválida pode ser acessada. Rust protege você contra esse tipo de erro ao sair imediatamente em vez de permitir o acesso à memória e continuar. O [Capítulo 9](https://doc.rust-lang.org/book/ch09-00-error-handling.html) discute mais sobre o tratamento de erros em Rust e como você pode escrever código legível e seguro que não entre em pânico nem permita acesso inválido à memória.
