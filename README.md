
<p align="center"><img src="art/logo.png" width="320px"/></p>

<h1 align="center">gipfeli</h1>

<p align="center">Quickly share small files via text messages</p>


## Usage

~~~
Quickly share small files via text messages

Usage: gipfeli [PATTERNS]...

Arguments:
  [PATTERNS]...  List of file patterns to match for packing

Options:
  -h, --help     Print help
  -V, --version  Print version
~~~

### Examples

To generate a shareable text from all the files with the `sh` and `py` extension from the current folder, run `gipfeli` as

~~~console
gipfeli *.sh *.py
~~~

The result is either copied to the clipboard or printed to the terminal.

To unpack a string, run `gipfeli` with no arguments with the string already
copied in the clipboard, or paste into the terminal, and then hit
<kbd>Enter</kbd>.
