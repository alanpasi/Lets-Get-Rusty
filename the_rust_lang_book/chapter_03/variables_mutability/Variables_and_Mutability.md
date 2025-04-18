##### Traduzido por deepseek em 16/04/2025
# Variáveis e Mutabilidade

Por padrão, as variáveis em Rust são **imutáveis**, o que significa que, uma vez que um valor é atribuído, você não pode alterá-lo. Isso incentiva a escrita de código seguro e concorrente. No entanto, você ainda pode optar por tornar as variáveis mutáveis quando necessário.

## Variáveis Imutáveis

Considere este exemplo:

```rust
fn main() {
    let x = 5;
    println!("O valor de x é: {x}");
    x = 6; // Isso causará um erro!
    println!("O valor de x é: {x}");
}
```

Se tentar compilar esse código, receberá um erro:

```text
error[E0384]: cannot assign twice to immutable variable `x`
```

Isso acontece porque `x` é imutável. Rust garante que variáveis imutáveis não sejam alteradas após sua definição, ajudando a evitar bugs.

## Variáveis Mutáveis

Se você quiser que uma variável possa ser alterada, use `mut`:

```rust
fn main() {
    let mut x = 5;
    println!("O valor de x é: {x}");
    x = 6; // Agora funciona!
    println!("O valor de x é: {x}");
}
```

A saída será:

```text
O valor de x é: 5
O valor de x é: 6
```

## Constantes

Assim como variáveis imutáveis, **constantes** também não podem mudar, mas há diferenças importantes:

- Constantes são sempre imutáveis (não usam `mut`).
- São declaradas com `const` em vez de `let`.
- Seu tipo **deve** ser anotado explicitamente.
- Podem ser declaradas em qualquer escopo, incluindo o global.
- Só podem receber valores constantes (não podem ser resultados de funções ou cálculos em tempo de execução).

Exemplo:

```rust
const TRES_HORAS_EM_SEGUNDOS: u32 = 60 * 60 * 3;
```

## Shadowing

Você pode **sobrescrever** (shadow) uma variável declarando uma nova com o mesmo nome:

```rust
fn main() {
    let x = 5;
    let x = x + 1; // Shadowing

    {
        let x = x * 2; // Shadowing em um escopo interno
        println!("O valor de x no escopo interno é: {x}");
    }

    println!("O valor de x é: {x}");
}
```

Saída:

```text
O valor de x no escopo interno é: 12
O valor de x é: 6
```

O *shadowing* difere de `mut` porque:
1. Cria uma **nova** variável com o mesmo nome.
2. Permite mudar o tipo da variável.

Exemplo com mudança de tipo:

```rust
let espacos = "   "; // String
let espacos = espacos.len(); // Agora é um número
```

Isso não seria possível com `mut`, pois o tipo não pode mudar.
