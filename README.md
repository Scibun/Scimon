# Paimon

Tool to download pdfs in batches written in Rust

> **Under in development**

To build, run

```shell
cargo build
```

> The directory of output is `target/release`

To download files with local list

```shell
paimon -r docs/paimon-example.txt
```

To download files with remote list

```shell
paimon -r https://raw.githubusercontent.com/AgiosLux/Paimon/main/docs/paimon-example.txt
```

To download files at Monlib list

```shell
paimon -r @kremilly/titulo-de-exemplo
```

To download files without skipping any, simply use the `--noignore` flag

```shell
paimon -r docs/paimon-example.txt --noignore
```

To skipping all comments, simply use `--no-comments` flag

```shell
paimon -r docs/paimon-example.txt --no-comments
```

To send e-book to Kindle, simply use the `--kindle` flag

```shell
paimon -r docs/paimon-example.txt --kindle <YOUR_KINDLE_EMAIL>
```

> *P.S.: A maximum file size limit for sending to Kindle is 25 MB.*

> *[Check out](https://www.lifewire.com/find-kindle-email-address-5271915) this tutorial to learn how to find your Kindle email address.*

To publish new Monlib (*`under in development`*) list

```shell
paimon --file docs/paimon-example.txt --title "Example list" --privacy public --publish
```

Use `--options` flag for manage your settings

* *api_listsopen-env*: Use `open-env` for open the .env file

  ```shell
  paimon --options open-env
  ```
* *force-download-env*: Use `force-download-env` for reset your  .env file

  ```shell
  paimon --options force-download-env
  ```

## Configuration's

| System  | Location                                                           |
| ------- | ------------------------------------------------------------------ |
| Linux   | `home/<YOUR_USERNAME>/.config/Paimon/.env`                       |
| MacOS   | `/Users/<YOUR_USERNAME>/Library/Application Support/Paimon/.env` |
| Windows | `C:\Users\<YOUR_USERNAME>\AppData\Roaming\Paimon\.env`           |

Environments of system

| Name           | Description                                                    |
| -------------- | -------------------------------------------------------------- |
| SMTP_SERVER    | Your SMTP server address                                       |
| SMTP_USERNAME  | Your username of SMTP server                                   |
| SMTP_PASSWORD  | Your password of SMTP server                                   |
| MONLIB_API_KEY | Your API key for access Monlib (*`Under in development`*) |

## Macros and Comments

Macros, in a computing context, are predefined sequences of commands or instructions that are executed when the macro is called. They are used to automate repetitive tasks and simplify complex processes, turning multiple instructions into a single instruction or command.

Paimon supports the following macros:

* *ignore*: When utilizing the `!ignore` macro, a user can specify certain URLs that they wish to bypass during the operation. For instance, if a software tool is tasked with scanning a list of URLs for updates or changes, by adding a specific URL next to the `!ignore` directive, that URL will be omitted from the scanning process.

  ```shell
  https://example.com/file.pdf !ignore
  ```
* *debug*:  The `!debug` macro displays specific information to the user, making it invaluable when you need to relay details to the tool's user.

  ```shell
  This is a comment that is displayed with the debug !debug
  ```

**Comments:**

The tool recognizes any non-blank lines without identifiable URLs as comments and, by default, these lines are neither processed nor displayed to the user.
