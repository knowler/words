# words

A place to make thoughts into words.

## CLI helper

I built a small CLI helper for creating new posts. This creates
a new file in the `contents` directory with a kebab-cased
filename, a title case `title`, and a RFC-3339 formated `date`.

### Installation

You can use Cargo to install the binary on your machine:

```shell
$ cargo install --path utilities/cli words
```

### Usage

Right now, the new command only takes one argument, so you will
need to either manually kebab/snake case the post title or wrap
it in brackets. The command will take care of creating a
kebab-cased filename and a tile-cased title. Either of the
following will work:

```shell
$ words new test-post
# or
$ words new "Another Test Post"
```
