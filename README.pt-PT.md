# FM27 Assistant PT

Assistente independente para Football Manager 27, com interface em **português de Portugal**.

Este projeto é uma adaptação do [FM27 Trainer](https://github.com/SemHiel2003/fm27-trainer), desenvolvido originalmente por Sem Hiel e distribuído sob licença MIT. Mantemos os créditos, a licença e os avisos de terceiros do projeto original.

## O que faz

- Lê a eficácia tática e os 17 fatores apresentados no ecrã do jogo.
- Planeia sessões de treino com base nos pontos mais fracos e nos jogos agendados.
- Ajuda a gerir orçamento, moral, lesões, suspensões e transferências, nas versões compatíveis.
- Apresenta o plantel e a compreensão tática.
- Permite exportar planos e consultar o histórico das alterações.

## Windows — utilização sem instalação

A versão Windows é distribuída como um ZIP portátil. Extrai o conteúdo e executa **FM27 Assistant PT.exe**. Não é necessário instalar Rust, Python, Node.js ou outro ambiente de desenvolvimento no computador onde a aplicação é utilizada.

A compilação e os testes continuam a ser necessários para preparar cada versão. O facto de ser portátil não significa que todas as versões do Football Manager estejam automaticamente validadas.

## Segurança e compatibilidade

- Utilização destinada a jogos individuais; não usar em modos online.
- As operações de escrita dependem da versão/build do Football Manager e da validação do respetivo layout.
- Em builds não verificadas, a aplicação deve manter o comportamento de leitura apenas, salvo decisão explícita e informada do utilizador.
- Preservar cópias de segurança e a possibilidade de anular alterações.
- A aplicação não deve enviar dados do utilizador para serviços externos.

## Estado do desenvolvimento

Estamos a trabalhar na branch `feature/pt-pt-localization`.

Primeiras alterações:
- suporte técnico para o idioma `Pt`;
- deteção de idioma do sistema para `pt` e `pt-PT`;
- escolha de Português na barra lateral;
- título da janela FM27 Assistant PT;
- tradução inicial da navegação e dos estados de ligação;
- pipeline Windows preparada para gerar `FM27-Assistant-PT-Windows.zip`.

A tradução integral e a validação de compilação ainda estão em curso.

## Créditos e licença

O código-base é de Sem Hiel. Ver [LICENSE](LICENSE) e [THIRD_PARTY.md](THIRD_PARTY.md). A adaptação não é afiliada, patrocinada ou aprovada pela Sports Interactive ou pela SEGA. Football Manager é uma marca da Sports Interactive / SEGA.

Consulte também o [plano de desenvolvimento](docs/PLANO-FM27-ASSISTANT-PT.md) e as [notas técnicas originais](docs/TECHNICAL.md).
