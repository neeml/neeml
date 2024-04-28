Ideas for Neeml
===============

# Name?

Ne(w) eml (joke about the .EML file type).

## Design notes:

- Seamless Gmail (IMAP/REST (for context-awareness))/Exchange (EWS/Graph)
  support with OAuth 2.0, once client ID+secret supplied. 

  This should be as simple as a copy and paste, or using a password shell
  command.
- NNTP, JMAP, IMAP, SMTP support.
- Sendmail binary support.
- NO POP3 support - everything should be server-side.
- Auto-complete from 'providers'.
- 'Contexts'. Contexts are modules that provide contexual assistance in the UI,
  allowing for informed emails.
- OpenAI/Ollama/LLM support for email composing.
- CardDAV support - native 'providers' for Gmail and Exchange.
- CalDAV support - Contexts.
- Hooks - could be used for backups upon mail modification. Hooks will pass a
  JSON object over stdin.
- Extensions with WASM runtime.

### Arch notes:

A bit like my experiments with Xt, Neeml will use a client/server design, with different frontends.

- The server and frontends share the 'common' crate.
- Server will communicate with remote servers (I use a local IMAP server
  currently, so this also works), or watches Maildirs/mboxes/NNTP.
- The server will run in the background as a managed service, and listen on
  localhost. RPC is encrypted, and done over a Unix socket/named pipe.
- Communication will be over RPC (Protobuf).
- Frontends such as CLI tooling (stdin, supported in the `neeml`
  binary/server.), TUI (like mutt or alpine), GUI (Tauri).
