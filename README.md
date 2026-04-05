# TADs Lineares
# Estruturas de Dados e Análise de Algoritmos
## Atividade: TADs Lineares (Vec, Pilha, Fila, Deque)

Este repositório contém a resolução dos exercícios práticos da Unidade Curricular de Estruturas de Dados e Análise de Algoritmos.O foco é a implementação de Tipos Abstratos de Dados lineares e a análise de sua complexidade algorítmica

**Aluna:** Ana Luiza  
**Professor:** Alexandre Montanha
**Data:** 05/04/2026

---

## 🛠️ Progresso do Projeto

### Grupo 1: Vec e Operações Básicas (Concluído)
1. **Inversão com Vec:** Inversão usando apenas `push`/`pop`.
2. **Contador de ocorrências:** Iteração com `for x in &vec` para contagem de caracteres.
3. **Remoção condicional:** Remoção de números pares de um `Vec<i32>` sem `.retain()`.
4. **Mescla ordenada:** Fusão de dois vetores ordenados.

### Grupo 2: Pilha / Stack (Concluído)
5. **Calculadora RPN:** Avaliação de expressões em Notação Polonesa Reversa com `Vec<f64>`.
6. **Histórico de Navegação:** Simulação de forward/back com duas pilhas.
7. **Desfazer/Refazer (Undo/Redo):** Implementação de lógica de editor minimalista.
8. **Sequências de Símbolos:** Verificação de balanceamento de parênteses, colchetes e chaves.
9. **Pilha com Mínimo:** Implementação de `StackMin` com operação `min()` em $O(1)$.



## 📈 Análise de Complexidade (Big O)

| Operação | Estrutura | Complexidade | Justificativa |
| :--- | :--- | :--- | :--- |
| Inversão (Ex 1) | `Vec` | $O(n)$ | Necessário percorrer todos os n elementos para inverter. |
| Remoção Pares (Ex 3) | `Vec` | $O(n)$ | Pior caso exige deslocar elementos após a remoção. |
| Pilha com Mínimo (Ex 9) | `Stack` | $O(1)$ | Uso de pilha auxiliar para manter o rastro do menor valor. |
| Janela Deslizante (Ex 15) | `Deque` | $O(n)$ | Cada elemento é inserido e removido do deque no máximo uma vez. |

### Não consegui fazer o restante, e o 1 e 2 grupo tava entendendo bem pouco também!
