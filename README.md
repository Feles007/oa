# The Oppressed Archives

A GMod addon archiving project.

Visit at: *ryff3kenu5u7qijckmatsids5ur4h6g335qg3bqq5s55z2db3buhkzad (dot) onion*

The site is static, so this is only the source for tools used to generate it.

This repo isn't really meant for others to use, it's mostly to keep track of development.

## File Structure

#### /generator/

A rust program which generates modified HTML, CSS, and images. Run like any other Cargo project.

On my machine, running it in release mode provides a ~30x increase in speed.

#### /gen-data/

Where the files needed by the generator are stored. This folder should be laid out as follows:

```
gen-data
│  data.json // This holds the creator, category, and addon data
│  robots.txt
│  favicon.ico
│
│─ templates
│   │─ index.html
│   │─ creator.html
│   │─ all.html
│   └─ error.html
│   
│─ static
│  │ no_image.jpg // Default thumbnail
│  └─ *.css // These files will be minified
│
└─ thumbnails
   └─ <creator name>
      └─ <addon name>.<file extension> // These should be 16:9
```

#### /public/

The directory to point your web server to. Contains the final product.
The generator doesn't deal with the actual addon files, so those must be placed manually.
(Though the directories can be created from the data.json file by running the generator with the --dir argument.)

## Setup

1. Install rustup and nginx-extras
2. Clone this repo into /srv/www/
3. *cd oa/generator/*
4. *cargo run --release*
5. *cd ../*
6. Copy or link oa.conf into /etc/nginx/sites-enabled/
7. Start, restart, or reload Nginx