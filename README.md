[rust-os](https://os.phil-opp.com/freestanding-rust-binary/)

[post 3](https://github.com/phil-opp/blog_os/tree/post-03)

` Writing an OS in Rust Philipp Oppermann's blog`

> This blog series creates a small operating system in the Rust programming language. Each post is a small tutorial and includes all needed code, so you can follow along if you like. The source code is also available in the corresponding Github repository.

> Latest post: Async/Await `

`This Operating System tutorial gives valuable insight with so much depth about computer architecture.`

`As Phillipp Oppermann introduces, "To write an operating system kernel, we need code that does not depend on any operating system features. This means that we can't use threads, files, heap memory, the network, random numbers, standard output, or any other features requiring OS abstractions or specific hardware. Which makes sense, since we’re trying to write our own OS and our own drivers".`

`We learn a lot by writing bare this bare metal, great practice for Rust. Not using GRUB or the Multiboot standard, amongst other challenges and limitations, allows full conspicuous understanding about the multitudes of operating systems.`
