<div align='center'>
    <img src="https://i.imgur.com/ZZ9a1DU.png"/>
</div>

<p align='center'><b>Unleash your knowledge.</b></p>

<p align='center'>
	<a href='https://github.com/Scibun/Scibun/actions/workflows/rust.yml'><img src='https://img.shields.io/github/actions/workflow/status/scibun/scimon/rust.yml?style=flat-square'/></a>
	<img src='https://img.shields.io/github/license/Scibun/Scimon?style=flat-square'/>
</p>

## Documentation

For more help and document, see our documentation:

- [How to build](https://scibun.github.io/ScimonDocs/build.html)
- [Basic usage](https://scibun.github.io/ScimonDocs/basic-usage.html)
- [Flags](https://scibun.github.io/ScimonDocs/flags.html)
- [Downloads Block](https://scibun.github.io/ScimonDocs/download-block.html)
- [Readme Block](https://scibun.github.io/ScimonDocs/readme-block.html)
- [Commands Block](https://scibun.github.io/ScimonDocs/commands-block.html) (Experimental)
- [Open links](https://scibun.github.io/ScimonDocs/open-links.html)
- [Checksum and Checksum Validate](https://scibun.github.io/ScimonDocs/checsum.html)
- [Directives and Comments](https://scibun.github.io/ScimonDocs/directives.html)
- [Markdown render](https://scibun.github.io/ScimonDocs/markdown-render.html)
- [Scrape](https://scibun.github.io/ScimonDocs/scrape.html)
- [Providers](https://scibun.github.io/ScimonDocs/providers.html)
- [Scimon.yml file](https://scibun.github.io/ScimonDocs/scimon.yml-file.html)
- [.env file](https://scibun.github.io/ScimonDocs/env-file.html)
- [External Resources Usage](https://scibun.github.io/ScimonDocs/external-resources.html)

## Example of code and execute

```monset
path = "downloads/"
open = "https://github.com/kremilly"

readme = "https://gist.githubusercontent.com/Kremilly/5fd360d994bb0fe108b648d0e4c9e92f/raw/ac524eba2112bf0bdbac1ad27e24f78f678589ec/readme-example.md"
checksum = "https://gist.githubusercontent.com/kremilly/499d6d51d096c1813cea0eade8eb0bc4/raw/d7c5965aeaf005cf0b612e3468ab47c30480083b/scibun.sha256"
checksum.unmatch = "keep"

commands {
    index.py
}

downloads {
    https://arxiv.org/pdf/2405.01513 !ignore
    https://www.scielo.br/j/rdbci/a/fwDKj9FMX7YTRDgkJGG4gnR?format=pdf&lang=pt !ignore
    https://en.wikipedia.org/wiki/Rust_(programming_language) !ignore
    https://en.wikipedia.org/wiki/Google !ignore
    https://www.nasa.gov/wp-content/uploads/static/history/alsj/a17/A17_FlightPlan.pdf !ignore
    https://sci-hub.se/10.1080/0025570x.2002.11953151
    https://olacesar.com/e-books/protegido.pdf
    https://github.com/huyubing/books-pdf/blob/master/slime.pdf !ignore
    https://raw.githubusercontent.com/facebook/react/main/README.md !ignore
    https://pt.wikisource.org/wiki/Manifesto_da_Guerrilha_do_Livre_Acesso !ignore
}
```

> [!note]
>
> Save as `scimon.mon`

Run the command:

```bash
scimon -r scimon.mon
```
