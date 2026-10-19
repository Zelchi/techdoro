# Techdoro

Timer Pomodoro desktop com lista de tarefas, reescrito em Rust.

O código do projeto está sob a [licença MIT](LICENSE). A fonte Press Start 2P
é distribuída sob a [SIL Open Font License 1.1](assets/PressStart2P-OFL.txt),
incluída também nos instaladores.

| Camada | Tecnologia |
| --- | --- |
| Interface e renderização | GPUI + GPUI Kit 0.7.3 |
| Regras e persistência | Rust, Serde e JSON |
| Bandeja | gpui-tray 0.1.4, com backend GPUI Kit |
| Áudio e notificações | Rodio e notify-rust |
| Distribuição | cargo-packager 0.11.8 (.deb/.exe) e cargo-generate-rpm 0.21.0 (.rpm) |
| Atualizações assinadas | cargo-packager-updater 0.2.3 no Windows; integração APT/DNF com Minisign no Linux |
| CI/CD | GitHub Actions, Linux e Windows x86-64 |

O aplicativo não usa frontend JavaScript, WebView ou Tauri. Node.js 22 é usado
somente nos scripts de release e seus testes, sem dependências npm; instale-o
para executar `make test` ou montar releases localmente.

## Desenvolvimento

Instale Rust estável, Rustfmt e Clippy. A versão mínima declarada é Rust 1.92;
use uma versão estável atual para o grafo de dependências travado em `Cargo.lock`.

No Ubuntu 24.04:

```sh
sudo apt-get update
sudo apt-get install -y build-essential clang cmake pkg-config \
  libfontconfig1-dev libwayland-dev libxkbcommon-x11-dev libx11-xcb-dev \
  libssl-dev libzstd-dev libasound2-dev libvulkan1 libxkbcommon-dev \
  patchelf librsvg2-bin file xdg-utils libarchive-tools desktop-file-utils
```

No Fedora, as dependências de desenvolvimento correspondentes incluem:

```sh
sudo dnf install gcc gcc-c++ clang cmake pkgconf-pkg-config fontconfig-devel \
  freetype-devel wayland-devel libxkbcommon-devel libxkbcommon-x11-devel \
  libX11-devel libxcb-devel openssl-devel libzstd-devel alsa-lib-devel vulkan-loader
```

Se a linkedição informar `unable to find library -lxkbcommon-x11`, instale
`libxkbcommon-x11-devel` (Fedora) ou `libxkbcommon-x11-dev` (Ubuntu).

Use uma sessão gráfica Wayland ou X11 com driver Vulkan funcional. O pacote
`libvulkan1` sozinho não instala o driver da GPU. A bandeja no Linux usa
StatusNotifierItem/D-Bus; o ambiente desktop precisa oferecer um host compatível.
No GNOME, pode ser necessária uma extensão de AppIndicator.

No Windows, instale Visual Studio 2022 Build Tools com **Desktop development
with C++**, Windows SDK e CMake; use o toolchain Rust MSVC.

```sh
cargo run --locked
```

Esse comando inicia o aplicativo uma vez. Para desenvolvimento com recarga
automática, use `make install` e `make dev`; alterações em `src`, `assets` ou
nos arquivos Cargo recompilam e reiniciam o aplicativo. O `make dev` instala o
[Watchexec](https://github.com/watchexec/watchexec) automaticamente se ele ainda
não estiver disponível. Como o processo reinicia, o timer em memória volta ao
estado inicial; tarefas e configurações salvas são preservadas. `make dev-once`
mantém a execução sem monitoramento. Sons, fonte e ícone são embutidos no binário.
O primeiro build compila o framework gráfico e pode demorar.

## Funcionalidades

- Foco, pausa curta e pausa longa com durações configuráveis.
- Pausa longa após a quantidade configurada de focos concluídos; a próxima sessão
  aguarda Start. Avançar manualmente percorre foco → curta → longa → foco.
- Iniciar, pausar e reiniciar, com medição por relógio monotônico.
- Criar, editar, concluir, excluir e reordenar tarefas por arraste.
- Clique no texto para editar; Enter salva e Escape cancela. O mesmo campo serve
  para adicionar tarefas.
- Volume, sons de clique/alarme e notificação ao concluir a sessão.
- Fechar a janela mantém o aplicativo na bandeja. O menu alterna entre Hide,
  quando a janela está aberta, e Show, quando está oculta. Também oferece
  Start/Pause, Reset, Next, atualizações e Exit. Abrir outra instância
  ativa a janela existente, sem criar outra janela ou outro processo do app.
- Se a criação da bandeja falhar, a interface informa isso e fechar encerra o app.

O estado do timer fica em memória e continua ativo sem janela. Tarefas e
configurações são salvas após cada alteração. Reiniciar o aplicativo reinicia o timer.

## Dados e migração da versão 3

O app usa `state.json`, com versão de esquema, no diretório de dados local do
usuário. Para localizar esse diretório:

```sh
cargo run --locked -- --data-dir
```

É possível definir `TECHDORO_DATA_DIR` para usar outro diretório, inclusive em testes.
Os arquivos são gravados por substituição atômica; JSON inválido ou de uma versão
futura gera erro sem sobrescrever os dados existentes.

Com o aplicativo encerrado pelo menu Exit:

```sh
techdoro --export backup.json
techdoro --import backup.json
```

A importação cria uma cópia dos dados anteriores antes de substituí-los.
A exportação exige um caminho que ainda não exista.

**A atualização de Tauri 3.x para Rust 4.x exige instalação manual.** O novo app
não lê diretamente os bancos de `localStorage` do WebKitGTK/WebView2 e não remove
esses dados antigos. Exporte os dados antes de desinstalar a versão antiga.

Se a sua versão 3 permite abrir as ferramentas de desenvolvimento, execute no
console da janela antiga:

```js
const keys = ['volume', 'timeLongMax', 'timeShortMax', 'timeFinalMax', 'cyclesBeforeFinal', 'tarefas'];
console.log(JSON.stringify(Object.fromEntries(keys.map(key => [key, localStorage.getItem(key)])), null, 2));
```

Salve o JSON exibido em `techdoro-v3.json` e importe com
`techdoro --import techdoro-v3.json`. Não execute esse código em um navegador
separado: ele precisa acessar o perfil da aplicação antiga. Se as ferramentas de
desenvolvimento não estiverem disponíveis, será necessária uma versão 3 com
recurso de exportação; a importação não extrai automaticamente o banco da WebView.

## Estrutura

```text
src/
  main.rs             Entrada e tratamento de falhas de inicialização
  lib.rs              Exportações do domínio e persistência
  app/
    actions.rs        Ações globais do aplicativo
    bootstrap.rs      Inicialização GPUI e ciclo em segundo plano
    cli.rs            Argumentos, importação e exportação
    launch.rs         Coordenação da CLI e inicialização gráfica
    model.rs          Estado vivo e persistência das alterações
    runtime.rs        Referências globais para estado, janela e bandeja
    timer.rs          Comandos e avanço do timer
    tray.rs           Menu e ícone da bandeja
    updates.rs        Comandos e mensagens do atualizador
    window.rs         Criação e ativação da janela
  domain/
    settings.rs       Configurações e durações
    state.rs          Estado persistido e normalização
    tasks.rs          Tarefas, edição e ordenação
    timer.rs          Sessões, contagem e ciclos
  desktop/
    audio.rs          Reprodução e volume
    instance.rs       Instância única e ativação
    notifications.rs  Notificações do sistema
  storage/
    codec.rs          Decodificação, validação e migração dos dados
    store.rs          Arquivos, gravação atômica e backups
    tests.rs          Testes de persistência e migração
  ui/
    view.rs           Estado da tela e composição dos componentes
    titlebar.rs       Barra de título
    timer.rs          Contador, progresso e controles
    tasks.rs          Entrada, edição, linhas e lista de tarefas
    settings.rs       Painel de configurações
    updates.rs        Rodapé e botão de atualização
    style.rs          Cores, ícone e botões compartilhados
    theme.rs          Tema GPUI e fonte
  updater/
    mod.rs            Coordenação das verificações e instalações
    linux.rs          Pacotes DEB/RPM e instalação via APT/DNF
    native.rs         Integração com cargo-packager-updater
    signature.rs      Chave pública e validação Minisign
    tests.rs          Manifestos, assinaturas e comandos de instalação
assets/               Fonte, sons, ícones e chave pública
scripts/              Montagem e validação dos artefatos de release
```

Os demais `mod.rs` declaram os arquivos de cada módulo e suas exportações.

## Qualidade

```sh
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --all-targets
node scripts/test-release.mjs
```

`make check` executa essas verificações. Os testes do domínio verificam pausa e
retomada, ciclos, transições, alteração das durações, tarefas e persistência. Os
scripts de release têm testes para manifestos, formatos e artefatos ausentes.
O atualizador testa seleção de formato/versão, rejeição de HTTP, assinatura válida,
pacote adulterado e argumentos dos comandos de instalação.
O CI executa as verificações Rust em Linux e Windows; testes de interface, áudio,
bandeja e instalação precisam também de uma sessão desktop real.

## Pacotes

```sh
cargo install cargo-packager --version 0.11.8 --locked
cargo install cargo-generate-rpm --version 0.21.0 --locked # Linux
cargo packager --release --formats deb  # Linux, também compila em release
cargo generate-rpm -o target/packages  # Linux, usa o mesmo binário de release
cargo packager --release --formats nsis # Windows
```

Os pacotes são gerados em `target/packages`. No CI, os pacotes Linux são
compilados no Ubuntu 24.04; o RPM usa o mesmo binário e descobre as dependências
ELF automaticamente. O instalador Windows é compilado no runner Windows. Builds
locais dependem das versões das bibliotecas do sistema usado na compilação.
Para distribuir o `.deb`, use o build do Ubuntu 24.04 do CI: um binário compilado
em um Fedora mais recente pode exigir uma glibc ausente no Ubuntu.

Os dois pacotes Linux incluem `com.zelchi.techdoro.desktop` e o ícone PNG em
`/usr/share/icons/hicolor/256x256/apps/techdoro.png`. O identificador do atalho
corresponde ao da janela no Wayland e ao `StartupWMClass` no X11.
No Windows, `build.rs` embute o ICO no executável; o NSIS também usa esse ICO
no instalador. Os atalhos criados pelo NSIS usam o ícone do executável instalado.
A release executa `node scripts/verify-package-icons.mjs` para conferir os
arquivos reais dos pacotes Linux e os recursos de ícone dos executáveis Windows.

Para instalar os pacotes Linux, use o gerenciador de pacotes para resolver as
dependências:

```sh
sudo apt install ./target/packages/techdoro_*.deb   # Ubuntu/Debian
sudo dnf install ./target/packages/techdoro-*.rpm   # Fedora
```

## Atualizações e releases

O aplicativo verifica atualizações ao iniciar. O botão Update aparece somente
quando uma versão mais recente está disponível. Pause o timer e clique em Update
para baixar e instalar; a verificação inicial apenas consulta a disponibilidade.
Se a instalação falhar, o botão permite tentar novamente. A bandeja também oferece
uma verificação manual ao selecionar Update. Todos os pacotes são validados por
assinatura Minisign antes da instalação. O download e a instalação acontecem fora
da thread de interface, sem precisar buscar instaladores no GitHub a cada versão.

No Windows, `cargo-packager-updater` instala o `.exe`. No Linux, o app detecta se
foi instalado por `.deb` ou `.rpm`, baixa o mesmo formato e chama APT ou DNF via
Polkit (`pkexec`). O sistema pode solicitar senha administrativa. A aplicação
reinicia após a instalação bem-sucedida. Fechar a solicitação de autenticação ou
uma falha de assinatura cancela a atualização. Nenhum comando de instalação usa
shell ou scripts baixados.

O Linux precisa de Polkit, de um agente de autenticação ativo na sessão desktop e
do gerenciador de pacotes correspondente. Os instaladores declaram essas
dependências. Builds executados por `cargo run` não tentam instalar pacotes no
sistema; a atualização integrada é habilitada para o executável pertencente ao
pacote Techdoro instalado. A versão atual suporta Ubuntu/Debian via APT e Fedora
via DNF. Somente `.deb`, `.rpm` e `.exe` são distribuídos.

O manifesto `native-latest.json` é separado do `latest.json` da versão Tauri.
Não substitua o manifesto antigo por pacotes novos esperando migração automática.

Para publicar, configure o secret `CARGO_PACKAGER_SIGN_PRIVATE_KEY` e, se houver
senha, `CARGO_PACKAGER_SIGN_PRIVATE_KEY_PASSWORD`. O workflow também aceita os
secrets anteriores `TAURI_SIGNING_PRIVATE_KEY` e `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`
para reutilizar a mesma chave Minisign. A chave privada deve corresponder à chave
pública em `assets/updater.pub`, preservada da configuração anterior.

Se preferir gerar uma nova chave:

```sh
cargo packager signer generate --path .keys/techdoro.key
```

Guarde a chave privada no secret e atualize `assets/updater.pub` com o conteúdo do
arquivo `.pub`. Opcionalmente, a variável de repositório
`TECHDORO_UPDATER_PUBLIC_KEY` substitui a chave pública durante o build de release.
Nunca versione a chave privada.

Atualize `Cargo.toml`, crie `.github/release-notes/vX.Y.Z.md` e substitua
`X.Y.Z` pela mesma versão definida em `Cargo.toml` antes de enviar a tag:

```sh
git tag -a vX.Y.Z -m "Techdoro X.Y.Z"
git push origin vX.Y.Z
```

O workflow valida a versão e as notas, executa os checks, gera e assina os três pacotes
Linux/Windows e publica a release somente após reunir os instaladores e o
manifesto. Não há publicação automática apenas por alterar arquivos locais.

## Referências

- [GPUI Kit](https://gpui-kit.com/docs/getting-started/)
- [gpui-tray](https://github.com/domenkozar/gpui-tray)
- [cargo-packager](https://github.com/crabnebula-dev/cargo-packager)
- [cargo-packager-updater](https://docs.rs/cargo-packager-updater/0.2.3/cargo_packager_updater/)
- [cargo-generate-rpm](https://github.com/cat-in-136/cargo-generate-rpm)
