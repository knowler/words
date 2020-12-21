+++
title = "Readable HTML starts with the element"
date = 2020-12-05T21:19:00-06:00

[taxonomies] 
tags = ["html", "coding"] 
+++

- There are two levels of semantics: for the human and the for browser.
  - The browser is straightforward: it really only cares what it can derive
    from the element’s tag name or the attributes which have been set on it.
    - For example the following two elements are functionally the same:  
      ```html
      <main>
      <div role="main">
      ```
    - Yes, it does change the way the element is selected, but the information
      about what the element is is the same.
  - The human is more complex: we should never stop caring about what the
    browser is going to understand from our code. On the other hand, the
    semantic choices for what we intend to communicate to the browser are
    limited. In cases where there are no semantics, we tend to revert to the
    `div` element.
- Semantics are best left to the HTML element.
    - Yes, I realize that this statement only makes sense in an HTML5 era.
- `div` elements should be avoided when possible.

- https://mrmrs.cc/writing/scalable-css/
- http://nicolasgallagher.com/about-html-semantics-front-end-architecture/
- https://adamwathan.me/css-utility-classes-and-separation-of-concerns/
