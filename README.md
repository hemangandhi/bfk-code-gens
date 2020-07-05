# What

A code generator and interpreter for brainfuck.

## Oof, right in the halting problem?

OK, the code generator isn't for "all the possible problems
that can be solved by brainfuck" since that set is impossible
to generate code for (halting problem go brr). The generator
will make a brainfuck program that outputs a given string.
I hope it will output the shortest such program.

Why? Because it's kind of a fun optimization question.

# How

Roughly: an MST on a graph with integer nodes and directed
edges that describe brainfuck programs that go between them.
The specification of the edges is a little tricky. Between
adjacent numbers an edge for `+` and `-` can exist easily.
However, multiplication would be trickier. For doubling
or tripling, this should be fairly clear. Perhaps multiplying
by 2 through 7 could suffice. Another question is about
"register allocation" (where brainfuck specifies infinite
registers). Fewer memory cells used should correlate with a
shorter program since `>` and `<` instructions will also start
to count. The edge-weighting in the graph might lead to some
coloring system that will also help with this. Traversing
`+` and `-` edges would have the same color, probably
(also since copying assures us a whole `[>+>+<<-]>` kind of
thing which might lead to various costs). It is worth noting
that traversing an edge on the graph is destructive, so potentially
costly, depending on the ordering of letters in the string.
(If the string is sorted by ASCII ordinals, the shortest program
would probably use one cell after the first output and increment it
for the rest. Afterall, `.>+[.+]` is a concise (never-ending) alphabet.)

This doesn't cover loop "winding" (ie. the opposite of unwinding).

Hyperoptimization also suggests that a closed search algorithm might
not always find the shortest code that outputs the string. It might
not even be linear (ie. generating the first character optimally would
not generate the rest too).

## Cost Estimate for Multiplying

An alternative way to estimate how expensive multiplying is. Roughly,
`cost(a * b) = 6 + cost(a) + cost(b)`, if register allocation works out
(meaning we'd also have to have 3 registers at all times: one for `a`,
one for `b`, and one for their product.

```
generate a here then[>generate b here<-]>
```

# Motivating example(s)

In which I reveal that this pandemic has led to me having too much time.
(Well also because it was for a reddit comment, only that and nothing more.)

```
+++++[>++>++<<-] makes two 10s next to each other
>+++>++++<       0 13 14, point at 13
[<+++++>-]<      point at 65 in {65 0 14}
[>+>+<<-]>       point at 65 in {0 65 79 (sum of 14 and 65)}
>.<.>------.     print 'yes'
```

This motivates some optimizations:

- The multiplications are pretty short (16 chars to get 2 10s, 26 to get a 65)
- While making the 65, the convenience to get a 14 (= 'y' - 'e') and then the
  copy adding to the 14 to actually make it a 'y'.

This makes me ponder that, in outputting "eeeee eeeee", can the same intuition
lead to:

```
+++++[>+>++<<-]>>+++ sadly 14 chars to get to 13 so a good example for caution in multiplication
                     might be redeemed by the 5 13 arrangement it really is
[>+++++>+++<<-]<     5 0 65 39 pointing at 5
[>+>.<<-]>>>-------. makes a 5 65 32 pattern while printing 'eeeee ' (pointing at 32)
<<[>.<-]             prints the other 5 e's
```

This points out that the ASCII ordinals might not be the only relevant numbers: the
format of the string might give some useful numbers too.

An other intersting thought: 'd' is at 64 and `[>++<-]>[<+>-]<` doubles a cell.
Hence, perhaps a shorter path to 65 is
`>+<++++++[>[>++<-]>[<+>-]<<-]>+`
Apparently not by 5 characters: `+++++[>++<-]>+++[<+++++>-]` which feels strange.
