# FM27 Assistant PT — plano de adaptação

Este fork passa a desenvolver a variante **FM27 Assistant PT**, mantendo a atribuição e a licença MIT do projeto original.

## Objetivos confirmados

- Plataforma principal: Windows 10/11.
- Distribuição: ZIP portátil, sem instalador.
- Idioma: português de Portugal (pt-PT).
- Base técnica: Rust, egui/eframe e agente existente.
- Interface: evolução visual profissional, sem reescrever desnecessariamente o motor.
- Nome apresentado ao utilizador: FM27 Assistant PT.

## Estado técnico confirmado no código-base

- O repositório já contém uma pipeline GitHub Actions para compilar um ZIP Windows.
- A interface usa egui/eframe e tem uma camada de idioma em `app/src/i18n.rs`.
- A internacionalização atual suporta neerlandês e inglês.
- O README descreve leitura de `fm.exe` em Windows e distribuição sem instalação.
- A documentação técnica distingue builds Windows testadas ao vivo de builds apenas reconhecidas por perfil. Não se deve assumir que todas as versões do jogo têm escrita validada.

## Etapas

1. **Idioma pt-PT**
   - Acrescentar `Lang::Pt` sem quebrar chamadas existentes de `t!(nl, en)`.
   - Detetar `pt` e `pt-PT` na língua do sistema.
   - Acrescentar opção explícita de idioma nas definições.
   - Traduzir primeiro navegação, estados, mensagens e ações; depois as restantes páginas.
   - Preservar em inglês os identificadores técnicos e os nomes de sessões/fatores quando estes têm de corresponder exatamente ao texto/dados do jogo.

2. **Identidade e interface**
   - Atualizar título da janela, nome do executável no pacote e documentação para FM27 Assistant PT.
   - Refinar hierarquia, espaçamento, navegação, estados e feedback sem remover funcionalidades.
   - Manter acessibilidade, contraste e suporte a escalas diferentes.

3. **Segurança**
   - Manter leitura como comportamento seguro quando a build não está verificada.
   - Não alterar automaticamente a política de compatibilidade existente.
   - Mostrar claramente quando a escrita está bloqueada, permitida ou validada.
   - Preservar backup, validação de valores antes da escrita e anular.

4. **Windows e distribuição**
   - Manter a compilação Windows no workflow.
   - Testar o ZIP em Windows 10/11 sem ferramentas de desenvolvimento instaladas.
   - Não publicar uma build como pronta antes de compilar e testar.

## Regras para os commits

- Trabalhar nesta branch de funcionalidade; não alterar `main` diretamente.
- Fazer alterações pequenas, auditáveis e reversíveis.
- Não remover `LICENSE`, `THIRD_PARTY.md` nem os créditos do autor original.
- Não afirmar afiliação à Sports Interactive/SEGA.
