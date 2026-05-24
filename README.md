# File Manager em Rust

Este projeto consiste na implementação de um **File Manager em linha de comando (CLI)** utilizando **Rust**, com foco em **operações sobre o sistema de arquivos**, busca recursiva, busca concorrente e coleta de métricas.

O projeto faz parte de um Trabalho de Conclusão de Curso (TCC), cujo objetivo é **comparar diferentes aspectos entre implementações em C++ e Rust**, incluindo desempenho, segurança, concorrência e uso de recursos.

---

# Objetivo

Desenvolver uma aplicação de gerenciamento de arquivos que permita:

- interação direta com o sistema operacional
- manipulação de arquivos e diretórios
- busca recursiva por nome
- busca concorrente com número configurável de threads
- medição de métricas relevantes

E posteriormente:

> comparar essa implementação com uma versão equivalente em C++.

---

# Tecnologias utilizadas

- Rust
- Cargo
- Linux
- Rayon
- API padrão de sistema de arquivos do Rust

---

# API e bibliotecas utilizadas

O projeto utiliza recursos da biblioteca padrão do Rust para manipulação de arquivos, diretórios e caminhos:

- std::fs::read_dir
- std::fs::metadata
- std::fs::copy
- std::fs::rename
- std::fs::remove_file
- std::fs::remove_dir_all
- std::env::set_current_dir
- std::env::current_dir
- std::path::Path
- std::time::Instant

Para busca concorrente, o projeto utiliza:

- rayon::ThreadPoolBuilder
- rayon::prelude

---

# Funcionalidades

- fm list <dir>
- fm cd <dir>
- fm info <file ou dir>
- fm copy <origin> <dest>
- fm move <origin> <dest>
- fm delete <path>
- fm search <name> <dir>
- fm search <name> <dir> <threads>
- fm tree <dir>
- fm help
- fm exit

---

# Execução

Compile o projeto com:

cargo build

Execute com:

cargo run

O programa abre um prompt interativo:

fm>

Exemplo:

list .

---

# Benchmark e Métricas

Use:

--benchmark ou -b

Exemplo:

search main . --benchmark

Saída:

[Benchmark]  
Execution time: 12 ms  
Resident memory: 5460 kb

---

# Métricas analisadas no TCC

## Tempo de execução
Medido com std::time::Instant.

## Uso de memória
Obtido via /proc/self/status (VmRSS).

## Performance de I/O
Avaliada em copy, search e tree.

## Busca concorrente
Comparação entre busca sequencial e busca com Rayon usando número configurável de threads.

## Segurança de memória
Comparação entre C++ e Rust, considerando ownership, borrowing e ausência de gerenciamento manual de memória.

## Complexidade
Avaliação qualitativa da implementação.

---

# Busca

A busca percorre diretórios recursivamente e compara o nome de cada arquivo ou diretório com o termo informado.

Os resultados encontrados são armazenados em um vetor, ordenados e impressos ao final da busca.

---

# Busca Multithread

diretórios são coletados  
↓  
Rayon distribui o processamento entre threads  
↓  
subdiretórios são processados recursivamente  
↓  
resultados são reunidos  
↓  
resultados são ordenados e impressos

Exemplo com 4 threads:

search main . 4

---

# Segurança

- ownership e borrowing para controle de memória
- tratamento de erros com Result
- validação de diretórios antes da busca
- controle do pool de threads com Rayon
- ausência de ponteiros brutos no código da aplicação

---

# Uso no TCC

Este projeto será utilizado para:

- comparação Rust vs C++
- análise de performance
- estudo de concorrência
- avaliação de memória
- avaliação de segurança de memória

---

# Referências

https://doc.rust-lang.org/stable/  
https://doc.rust-lang.org/std/fs/  
https://docs.rs/rayon/latest/rayon/  
https://man7.org/linux/man-pages/  

---

# Autor

Lucas Scommegna
