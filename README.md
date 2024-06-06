# Curves Continuity

Gera as contuidades C^0, C^1 e C^2 entre uma curva B-Spline de grau 5 e uma curva Bézier de grau 5 respectivamente.

## Requisitos

- [Rust](https://www.rust-lang.org/tools/install): O compilador Rust e o gerenciador de pacotes Cargo.

## Instalação

Siga os passos abaixo para clonar o repositório e compilar o projeto:

1. Clone o repositório:
    ```sh
    git clone https://github.com/bidinpithecus/curves-continuity
    ```

2. Navegue até o diretório do projeto:
    ```sh
    cd curves-continuity
    ```

3. Compile o projeto:
    ```sh
    cargo build
    ```

## Executando o Projeto

Para rodar o projeto, use o seguinte comando:

```sh
cargo run
```

## Modificando as Curvas

Para alterar as curvas definidas por padrão, edite os arquivos de entrada localizados no diretório `input`.

- `bspline.txt`: Contém os pontos de controle da curva B-Spline.
- `bezier.txt`: Contém os pontos de controle da curva Bézier.

### Formato dos Arquivos

Ambos os arquivos devem seguir o formato abaixo:

1. A primeira linha deve conter apenas um número, representando o grau da curva.
2. As linhas seguintes devem conter os pontos de controle, no formato `x,y,z`.

Exemplo:

```txt
5
0.0, 0.0, 0.0
0.0, 1.0, 0.0
1.0, 1.0, 0.0
2.0, 0.0, 0.0
3.0, -1.0, 0.0
4.0, -1.0, 0.0
```
