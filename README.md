izzet
=====

[![CircleCI](https://circleci.com/gh/pjhades/izzet/tree/master.svg?style=svg)](https://circleci.com/gh/pjhades/izzet/tree/master)

A simple static blog generator, simple enough that nobody wants to use it at all.

Build and Test
==============

Simple enough, just do:

```bash
$ make
$ make test
```

Create a New Site
=================

Find a directory you like and run:

```bash
$ izzet -n        # Initialize a site in the current directory
$ izzet -n site   # Initialize a site in the specified directory
```

After that, in your site directory you will find things arranged like this:

```bash
$ tree site/
site/
├── izzet.toml
├── src
└── theme
    ├── archive.html
    ├── index.html
    └── post.html

2 directories, 4 files
```

Here's a brief description of what you find there:
- `izzet.toml` is the configuration file in TOML.
- `src` is the source directory, where you put all your source files for the articles, pages and so on.
  Only source files in this directory will be scanned by izzet when generating the site.
- `theme` is the theme directory, where there are templates for the pages. By default izzet will only
  create a very simple (or ugly if you like) theme. You can customize them in whatever way you wish.
  (See the [Customizing Themes](#customizing-themes) section.)
  Don't forget to commit them through your version control system.

Write Posts
===========
Izzet allows you to write articles or pages. An article is simply a blog post or something, while a page
is something like the "about page". Internally izzet treats them all as posts.

To write a post you may do:

```bash
$ cd site
$ izzet -a src/article.md   # Create an article
$ izzet -p src/page.md      # Create a page
```

Then in the created post source file, you'll find:

```bash
$ cat src/article.md
title = "Default Title"
link = "article"
url = "/{{ year }}/{{ month }}/{{ day }}/{{ link }}.html"
ts = "2017-12-06T19:19:25.897192-05:00"
kind = "Article"
%%%
```

This is a header containing the metadata of the post:

- `title` is the title that will be displayed on the page for that post.
- `link` is the link, which may be used in the URL of the post.
- `url` is the URL pattern, which determines at what URL this post will be accessed.
- `ts` is the timestamp of the creation of this post.
- `kind` is the kind of this post: article or page.
- `%%%` marks the end of the metadata, following which is the actual content of the post.
  So you should start writing things after that mark.

See the [post metadata](#post-metadata) section for a reference of all metadata options.

Generate Site
=============
Izzet generates a site in two steps:

1. Collect source files and translate the markup language (only supports Markdown now) to HTML.
2. Generate output files, creating the site file layout based on the configuration and the metadata of posts.

To generate the site, just do:

```bash
$ izzet -g
```

This is the simplest form, which asks izzet to:

- use the default configuration file under current directory `./izzet.toml`,
- generate the site with source files in `./src/`, and
- write output files to the current directory.

So accordingly you can change each of these behaviors by specifying certain options like:

```bash
# Use configuration file /path/to/conf
$ izzet -g -c /path/to/conf
# Find source files under /path/to/indir/src
$ izzet -g -i /path/to/indir
# In addition, write output files to /path/to/outdir
$ izzet -g -i /path/to/indir -o /path/to/outdir
```

So our sample site above will be generated like:

```bash
$ ./izzet -g
$ tree .
.
├── 2017
│   └── 12
│       └── 06
│           └── article.html
├── archive.html
├── index.html
├── izzet.toml
├── page.html
├── src
│   ├── article.md
│   └── page.md
└── theme
    ├── archive.html
    ├── index.html
    └── post.html

5 directories, 10 files
```

You will find some new files:
- `index.html` is the index page.
- `archive.html` is the archive page containing the list of all articles.
- `2017/...` containing the rendered articles.

Preview the Site
================
You can start a local HTTP server to preview the generated site:

```bash
$ izzet -s           # Serve the current directory on the default port
$ izzet -s -l 9999   # Listen on port 9999
```

The default port of the local server is 10950;

Configuration
=============
Izzet reads configuration written in TOML.
Here's a list of all supported configuration options.
Some options have a corresponding command-line option or flag.
Note that the options specified in the command has the highest priority.

- `force` (boolean, optional):
  Overwrite existing files. This affects all possible writes like creating new site and creating post source.

- `in_dir` (string, optional):
  Site directory where source files will be looked for.

- `out_dir` (string, optional):
  Output directory where generated site files will be written to.

- `port` (integer, optional):
  Port number for the local server.

- `title` (string, optional):
  Site title.

Post Metadata
=============
Post metadata is, in most situations, automatically generated by izzet when
creating a new post source file. So you may rarely need to change it manually.

- `title` (string, mandatory):
  Title of the post.

- `link` (string, mandatory):
  Link of the post, which may be used by the URL pattern.

- `url` (string, mandatory):
  URL pattern. This determines the output path of the post.
The following variables can help:

  - `{{ year }}`: year from the timestamp of the post.
  - `{{ month }}`: month from the timestamp of the post.
  - `{{ day }}`: day from the timestamp of the post.
  - `{{ link }}`: link of the post as set by `link` above.

For example, the default URL pattern is
`"/{{ year }}/{{ month }}/{{ day }}/{{ link }}.html"`.
If a post with link `xyz` is created on Jun 1 2000, then according to this pattern,
the generated post will be written to `/2000/06/01/xyz.html`.

- `ts` (timestamp, mandatory):
  Creation timestamp of the post.

- `kind` (string, mandatory):
  Kind of the post. Currently this option can only be set to `"Article"` or `"Page"`.

Customizing Themes
==================
Izzet uses [Tera](https://crates.io/crates/tera) as the templating system.
A newly created site comes with a theme which only has a set of nearly-empty templates.
To customize them or create your own themes, pay attention to the following rules.

First of all your theme needs to include at least these templates:

- `index.html` which will act as the index page.
- `archive.html` which will display a list of your articles.
- `post.html` which will be used to render your posts.

All these files follow the syntax of Tera, namely a Django/Flask-like templating syntax.

Within the templates, you can use the following variables:

- `post`, which refers to the post being rendered.
- `conf`, which refers to the site configuration.
- `pages`, a list of pages collected in your site.
- `articles`, a list of articles collected in your site.
- `latest_article`, refers to the most recently created article.
