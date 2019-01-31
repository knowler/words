+++
title = "Rust For A Rusty Game Developer — Part I"
date = 2019-01-02T21:18:00-07:00

[taxomonies]
tags = ["rust", "coding"]
+++

Rust is my first foray into a systems programming language since
highschool and even then I never went in past my toes. This is
the introduction of a series of posts I’ll make as I document my
experience of learning Rust.
<!-- more -->

## The Aspiring Game Developer

I remember wanting to learn C++, because that's what the
professional game developers were using. I was an impatient
teenager though — I just wanted to make games — and I ended up
settling with an ActionScript 3 library called [FlashPunk]. I
used AS3 a lot and did end up learning a lot of foundational
programming concepts through it (it takes a lot from Java; AS2
was more JavaScript-y). When I went to college I jumped back to
the web (my stomping ground), because there weren’t a lot of
ways I knew how to use my game development skills. Everyone
needed a website, so I embraced my new role as everyone’s web
developer.

## Why a Systems Programming Language?

Over the last 4-5 years, I have become obsessed with speed and
performance. How did I become this way? By living in a rural
town in the middle of nowhere Saskatchewan for 4 years. When I
got there I didn’t have any cell reception and the WiFi was
horrible and though it did improve during my time there, I
became keenly aware of the frustrations of living with a
sub-sub-par Internet connection. This experience has been
guiding for me as I dive deeper into programming. **I want to
build products that everyone, anywhere can access — no matter
what difficulties may befall them.**

## Why Not C++?

When I was deciding to into a deeper programming language, I
first jumped to C++ because I had some familarity with it and it
is so popular. It was hard to feel like I was getting anywhere
after a week though. I had become accustomed to the ecosystems
that I knew from Node and PHP, but there seemed to be no such
thing for C++. I realized that the language was pre-Internet and
it didn’t have the opportunity for a centralized ecosystem to
catch on — by the time the Internet came around everyone had
developed their own ways of managing dependencies. **Attempting
to learn C++ was a lonely experience.** I needed something that
had a thriving ecosystem if I was going to catch any sort of
momentum in my learning.

## Why Rust?

I had heard of Rust through following [Yoshua Wuyts on
Twitter](https://twitter.com/yoshuawuyts). I had discovered him
through being a fly on the wall in the communities that
surrounded Tachyons and functional CSS. Since I trusted his
fascination with performance, I became interested in Rust and it
was the first language that came to mind when I started feeling
lonely learning C++. Rust was very much the opposite of what I
found with C++: the community is thriving, there are plenty of
official, well maintained resources for learning the language,
the ecosystem is well established — despite being a relatively
young language — and the tooling is fantastic. **While Rust can
feel like a decently steep learning curve, the community and
ecosystem cushion that, make it easy to gain momentum when
learning.**

## Resources I’ve Found Helpful

The following are some resources I’ve found helpful as I’ve
learnt Rust. Let it be a guide to the Rust ecosystem if you’re
brand new.

### Books

I began reading _[The Rust Programming Language]_ in August.
Though, as I read, I did find it difficult to figure out what I
could use Rust for that wasn’t too far fetched for a beginner
(i.e. making a game), the book did include some great exercises
which helped me get the underlying theories and values which are
core to Rust as a language. The final project in the book was a
multi-threaded web server and it was my motivation. Overall, I
would recommend the book to anyone wanting to begin with Rust.
Sure you might want something that is more practical and has
less theory, but if you take the time to read the book, you’ll
understand why Rust is the way it is. After reading the book, I
discovered _[Rust By Example]_ which satisfies the desire for a
resource that is more practical and straightforward; a great
reference to keep while programming in Rust.

### Documentation

[Docs.rs] is amazing. They have done such a good job of
centralizing all of the documentation, information, and links
regarding any crate you pick up.

{{ resize_image(path="./images/docs-rs-crate-overview.png", width=800, height=800, op="fit", quality=90) }}

## Projects

So far, the most immediate need that I’ve discovered that I can
use Rust for is commandline-interfaces. I have more than a few
Bash scripts lying around which I’ve been wanting to refactor
and I’ve begun using Rust for that. So far I’ve been using
[StructOpt] as a library for building [quick little CLIs for
everyday tasks](https://github.com/knowler/roots-cli-rust).

This blog project in itself has been a fun chance to learn Rust.
I chose [Zola] as the static site generator and it's been a
pretty smooth experience so far. It's able to deploy to Netlify
which is always great. As a little fun project I built [a CLI for
scaffolding new posts](https://github.com/knowler/words#cli-helper),
which should serve me as I continue to blog. I've found [Tera]
— a Jinja inspired templating language to be a great asset for
that type of project.

## Future Goals

I think as a big goal, I’d love to build a 2d platformer. I’ve
always loved 2d games and that's much of what I built when I
used to make games.

- - -

**Coming Soon — Part II**

[Docs.rs]: https://docs.rs
[StructOpt]: https://github.com/TeXitoi/structopt
[Tera]: https://tera.netlify.com
[Zola]: https://getzola.org
[The Rust Programming Language]: https://doc.rust-lang.org/book
[Rust By Example]: https://doc.rust-lang.org/rust-by-example
[FlashPunk]: http://useflashpunk.net