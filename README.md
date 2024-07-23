<div align='center'>
    <img src="https://i.imgur.com/ZZ9a1DU.png"/>
</div>

<p align='center'><b>Unleash your knowledge.</b></p>

<p align='center'>
	<a href='https://github.com/Scibun/Scibun/actions/workflows/rust.yml'><img src='https://img.shields.io/github/actions/workflow/status/scibun/scimon/rust.yml?style=flat-square'/></a>
	<img src='https://img.shields.io/github/license/Scibun/Scimon?style=flat-square'/>
</p>

<p align='center'>
    <img src='https://i.imgur.com/RRPMQ2j.png' />
</p>

## What is Scimon?

Scimon is a tool designed for batch downloading PDF files using its own dedicated language, Monset (.mon). Monset features a very simple and quick-to-write syntax, making it easy to use. The Scimon interpreter is both fast and secure, as it is written in Rust, leveraging the language's best practices.

## What is Monset?

Monset is a language designed specifically for downloading files. It offers a streamlined syntax that makes the process of retrieving files from the internet straightforward and efficient. By focusing on simplicity, Monset ensures that users can quickly grasp its fundamentals and start downloading files with minimal effort.

The key strength of Monset lies in its user-friendly design. The syntax is intuitive, reducing the learning curve typically associated with programming languages. This makes it accessible to both beginners and experienced developers, allowing them to integrate file downloading capabilities into their projects seamlessly. Monset abstracts the complexities involved in file transfers, providing a clear and concise way to handle downloads.

## Documentation

For more help and document, see our documentation:

- [How to build](https://scibun.github.io/ScimonDocs/build.html)
- [Basic usage](https://scibun.github.io/ScimonDocs/basic-usage.html)
- [Flags](https://scibun.github.io/ScimonDocs/flags.html)
- [Scrape](https://scibun.github.io/ScimonDocs/scrape.html)
- [Providers](https://scibun.github.io/ScimonDocs/providers.html)
- [Monset](https://scibun.github.io/ScimonDocs/monset/what-is.html)
  - [Downloads Block](https://scibun.github.io/ScimonDocs/monset/download-block.html)
  - [Readme Block](https://scibun.github.io/ScimonDocs/monset/readme-block.html)
  - [Commands Block](https://scibun.github.io/ScimonDocs/monset/commands-block.html)
  - [Compress folder](https://scibun.github.io/ScimonDocs/monset/compress.html)
  - [Open links](https://scibun.github.io/ScimonDocs/monset/open-links.html)
  - [Markdown render](https://scibun.github.io/ScimonDocs/monset/markdown-render.html)
  - [Style](https://scibun.github.io/ScimonDocs/monset/style.html)
  - [Print](https://scibun.github.io/ScimonDocs/monset/prints.html)
- [Configs](https://scibun.github.io/ScimonDocs/configs/index.html)
  - [Scimon.yml file](https://scibun.github.io/ScimonDocs/configs/scimon.yml-file.html)
  - [.env file](https://scibun.github.io/ScimonDocs/configs/env-file.html)
- [External Resources Usage](https://scibun.github.io/ScimonDocs/external-resources.html)

## Example of code and execute

```monset
path = "downloads/"
open = "https://github.com/kremilly"

compress = "folder.zip"

print = "Hello, World!"

style = "https://raw.githubusercontent.com/sindresorhus/github-markdown-css/main/github-markdown.css"

readme = "https://gist.githubusercontent.com/Kremilly/5fd360d994bb0fe108b648d0e4c9e92f/raw/ac524eba2112bf0bdbac1ad27e24f78f678589ec/readme-example.html"

downloads {
    https://arxiv.org/pdf/2405.01513 !ignore
    https://olacesar.com/e-books/protegido.pdf !ignore
    https://raw.githubusercontent.com/facebook/react/main/README.html
    https://pt.wikisource.org/wiki/Manifesto_da_Guerrilha_do_Livre_Acesso !ignore
}

commands {
    https://gist.githubusercontent.com/Kremilly/e0e0db11e43269da179adab610f38bb1/raw/6820be26a936a54bac713d03deb49edf804d0b6b/index.py
}
```

> [!note]
>
> Save as `scimon.mon`

Run the command:

```bash
scimon -r scimon.mon
```
