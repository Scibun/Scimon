# Paimon

Tool to download pdfs in batches written in Rust

> **Under in development**

To download files with local list

```shell
cargo run docs/paimon-example.txt
```

To download files with remote list

```shell
cargo run https://raw.githubusercontent.com/kremilly/Paimon/main/docs/paimon-example.txt
```

To download files without skipping any, simply use the `--noignore` flag

```shell
cargo run docs/paimon-example.txt --noignore
```

## Macros and Comments

Macros, in a computing context, are predefined sequences of commands or instructions that are executed when the macro is called. They are used to automate repetitive tasks and simplify complex processes, turning multiple instructions into a single instruction or command.

Paimon supports the following macros:

* Ignore: When utilizing the !ignore macro, a user can specify certain URLs that they wish to bypass during the operation. For instance, if a software tool is tasked with scanning a list of URLs for updates or changes, by adding a specific URL next to the `!ignore` directive, that URL will be omitted from the scanning process.

  ```shell
  https://example.com/file.pdf !ignore
  ```
* debug:  The `!debug` macro displays specific information to the user, making it invaluable when you need to relay details to the tool's user.

  ```shell
  This is a comment that is displayed with the debug !debug
  ```

**Comments:**

The tool recognizes any non-blank lines without identifiable URLs as comments and, by default, these lines are neither processed nor displayed to the user.
