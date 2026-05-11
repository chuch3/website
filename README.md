## poly's web 🚧

personal domain/library/website to dropoff projects in this microworld of an internet

### TODO

> Currently enduring front end hell! :)

start project : 
- 2026-04-18 14:15 (backend) 
- 2026-04-25 15:32 (frontend)  
- 2026-05-12 02:45 (unfinished, but first hosting! yaaaay)

- [ ] Scroll bar within masonry
- [ ] custom fonts
- [ ] custom cursor 
- [ ] implement HTTPS protocol (TLS and SSL certificates) with `rustls`
    - OpenSSL web security 


- masonry layout on projects, check breitnw and neocities 
    - extend and modularize base html for other indexes

- html/css templating | 2026-04-22 01:09 > 

    - base template to hold base jinja template
    - Web structure
        - Guestbooks
        - Blog post for progress
        - Social links
        - collage with card based template

> Compiler-Driven developement with errors to guide funcionality, focusing on client API design over implementation (top-to-down approach).

### Bugs 

- [ ] Fix corner friend (cow gif) position in mobile phones or adjusted screens
- [ ] Set base url to take on environment var host url
- [ ] Base URL and path on open graph base.html and global variable for jinja environment

### Future Features

- better home page and project card designs (abstract deadline)

- Async
- Experiment with `Futures` over closures

> "Other options you might explore are the fork/join model, the single-threaded async I/O model, and the multithreaded async I/O model."

### Done!

- [x] setup http/tcp suite 
- [x] response with html
- [x] multi-threaded server | 2026-04-18 19:46 > 2026-04-19 04:13
- [x] cleanup | 2026-04-19 10:29 > 2026-04-19 11:33
- [x] error handling 

- [x] HTTP reponse handler with parser and designated html as response body 
    - [x] basic frame of response builder | 2026-04-19 20:06 > 2026-04-20 01:59
    - [x] creating jinja environment and context | 2026-04-19 22:51 > 2026-04-20 01:28
    - [x] Add jinja and template bytes convertion on html body | 2026-04-20 14:09
    - [x] Create response and write into tcp stream as bytes | 2026-04-20 18:12
    - [x] Add mock template to test response and minijinja 
- [x] Submit mime header and static resource into response when requested

- [x] Jinja page descriptors
- [x] jinja toml read 

### Reference 

[rust book](https://doc.rust-lang.org/book/ch21-01-single-threaded.html)

