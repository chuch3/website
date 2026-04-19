### (WIP) personal website

#### TODO :

start time : 2026-04-18 14:15

- [x] setup http/tcp suite 
- [x] response with html
- [x] multi-threaded server | 2026-04-18 19:46 > 2026-04-19 04:13
- [x] cleanup | 2026-04-19 10:29 > 2026-04-19 11:33
- [x] error handling 

- [ ] HTTP reponse handler with parser and designated html as response body 
    - [x] basic frame of response builder | 2026-04-19 20:06 > 
    - [x] creating jinja environment and context | 2026-04-19 22:51 > 2026-04-20 01:28
    - [ ] Add jinja and template bytes convertion on html body 
    - [ ] Create response and write into tcp stream as bytes
    - [ ] Add mock template to test response and minijinja 
- [ ] implement HTTPS protocol (TLS and SSL certificates) with `rustls`
- [ ] html/css templating  
    - neocities guide & template
    - collage with card based template
- [ ] host on github
    - [ ] Base URL and path on open graph base.html and global variable for jinja environment


> Compiler-Driven developement with errors to guide funcionality, focusing on client API design over implementation (top-to-down approach).

#### Future Features

- OpenSSL web security 
- Async
- Experiment with `Futures` over closures

> "Other options you might explore are the fork/join model, the single-threaded async I/O model, and the multithreaded async I/O model."

### Reference 

[rust book](https://doc.rust-lang.org/book/ch21-01-single-threaded.html)

