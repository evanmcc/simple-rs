# notes on working through the simple repo

# chapter 1

this was all pretty simple, but I didn't do it in java so my perspective of the whole thing might be subtly warped.  I suppose to lay out my motivations, I find object oriented style borderline impossible to follow sometimes, and prefer explicitly procedural style for educational materials.  As such, while I do use some struct methods in the rust implementation I'm doing, I attempt to only use them to make things a little cleaner for the pseudo-coding stuff that I am doing to avoid having to write a parser.

Since I'm not doing it in Java, most of the work here was set up and getting things to work the way I wanted to do it.

commit: f72e693

# chapter 2

I suspect from an educational standpoint, that jumping right into aggressive peephole optimizations for nodes as they're created is not actually the best approach, and I think that introducing a lattice right away is going to annoy a lot of people.

Anyway, I skipped all the peephole stuff, and will probably continue to do so until the IR is more built up, but we'll see how doable that is.
