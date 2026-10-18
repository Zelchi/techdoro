# Arquivos de teste das assinaturas

Esta pasta contém fixtures usados nos testes do atualizador. Eles permitem
verificar uma assinatura válida e confirmar que um download adulterado é rejeitado.

| Arquivo | Finalidade |
| --- | --- |
| `update.pub` | Chave pública de teste, codificada em Base64. |
| `update.txt` | Conteúdo de exemplo assinado para o teste. |
| `update.txt.sig` | Assinatura Minisign desse conteúdo, codificada em Base64. |

Esses arquivos são públicos e podem ser versionados. Nenhum deles contém uma
chave privada. A assinatura permite verificar o conteúdo, mas não criar novas
assinaturas.

Os testes que usam esses arquivos estão em `src/updater/tests.rs`. Para executá-los,
rode na raiz do projeto:

```sh
cargo test --locked --bin techdoro updater::tests
```

## Chaves das releases

A chave pública usada pelo aplicativo para validar atualizações está em
`assets/updater.pub`; ela é independente da chave pública de teste desta pasta.
O build também permite substituí-la pela variável `TECHDORO_UPDATER_PUBLIC_KEY`.

A chave privada de produção deve ser guardada fora do Git. O workflow de release
a recebe pelo secret `CARGO_PACKAGER_SIGN_PRIVATE_KEY`, com senha opcional em
`CARGO_PACKAGER_SIGN_PRIVATE_KEY_PASSWORD`. Ele também aceita os secrets antigos
`TAURI_SIGNING_PRIVATE_KEY` e `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`.

## Arquivos locais para configurar o GitHub Actions

Quando gerados localmente, estes arquivos permanecem ignorados pelo Git:

| Arquivo local | Uso no GitHub Actions |
| --- | --- |
| `.keys/techdoro.key` | Copiar o conteúdo inteiro para `CARGO_PACKAGER_SIGN_PRIVATE_KEY`. |
| `.keys/techdoro-password.txt` | Copiar a senha para `CARGO_PACKAGER_SIGN_PRIVATE_KEY_PASSWORD`. |
| `.keys/techdoro.key.pub` | Cópia local da chave pública correspondente a `assets/updater.pub`. |

Os dois primeiros arquivos são secretos e não devem ser enviados ao GitHub como
arquivos do repositório. Os nomes dos secrets e dos arquivos não são secretos.
Se `TECHDORO_UPDATER_PUBLIC_KEY` estiver configurada nas variáveis do repositório,
seu valor deve corresponder a `assets/updater.pub`, ou a variável deve ser removida
para usar a chave pública versionada.

O `.gitignore` permite nesta pasta apenas os três fixtures e este README.
