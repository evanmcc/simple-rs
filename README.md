# simple-rs

A loose rust implementation of Cliff Click's [Simple](https://github.com/SeaOfNodes/Simple/) compiler/book.

I say loose because what I am mostly interested in is comparing optimization approaches: the eager peephole optimization in the book, [e-graphs](https://egraphs-good.github.io/), and Chris Fallin's [acyclic e-graph](https://cfallin.org/blog/2026/04/09/aegraph/) approach.

## chapter 1

this was all pretty simple, but I didn't do it in java so my perspective of the whole thing might be subtly warped.  I suppose to lay out my motivations, I find object oriented style borderline impossible to follow sometimes, and prefer explicitly procedural style for educational materials.  As such, while I do use some struct methods in the rust implementation I'm doing, I attempt to only use them to make things a little cleaner for the pseudo-coding stuff that I am doing to avoid having to write a parser.

Since I'm not doing it in Java, most of the work here was set up and getting things to work the way I wanted to do it.

commit: f72e693

## chapter 2

I suspect from an educational standpoint, that jumping right into aggressive peephole optimizations for nodes as they're created is not actually the best approach, and I think that introducing a lattice right away is going to annoy a lot of people.

Anyway, I skipped all the peephole stuff, and will probably continue to do so until the IR is more built up, but we'll see how doable that is.

## chapter 3

Scopes!  This is pretty simple.  That said, I've been sloppy about how stuff was organized up to now, so I'm going to start reorganizing stuff so that `prog` and the nodes stuff is more separated.

*days later* Well, that was a giant pain in the ass.  Still feeling my way around various rust things.  My major commentary about this chapter is that while it does introduce an important concept, it's still stepping on itself with respect to construction due to all of the peephole optimization rules that it's applying, which are acting like a ersatz interpreter.  While they dramatically reduce the complexity of the node graph, they leave up in the air what the final graph is actually going to look like, and my current code is peppered with TODOs talking about rough edges because I don't know what the end code is going to look like, and potentially incorrect assumptions leading to weird corners of the code.

I *also* realize that some of this is me doing it in rust in a semi-procedural way, rather than java, where I'd have all the code, or oop-y rust, where it would look more or less like a direct translation, but I am pretty set in my ways, I will maintain till my dying day the stance that strongly OO Java code is pretty hard to read.

The other thing that isn't super clear at this point is how various versions of the stack nodes are retained, which makes the correctness conditions for the node pool construction kind of hard to check (especially if you're not doing it right like I am).

