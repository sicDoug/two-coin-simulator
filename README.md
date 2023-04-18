# two-coin-simulator

Simple CLI simulator of the infamous Two-Coin Problem written in Rust.

## The Problem.

Two coins will be flipped.
You know that at least one of them will land Heads up.

*What his the probablility of both coins landing Heads up?*

## Solution.

People's intuition often tricks them into thinking the snswer is 50%.
The reason for this is the human tendency to simplify.
Sometime this can lead to *over*simplification.

>Well, if one of them is heads, the other could only be heads or tails.
This much is true.
>Since it can only be heads or tails, the answer is fifty-fifty.
This, however, is a fallacy.

When two coins are flipped, there are four possible outcomes:

1. Heads-heads.
2. Heads-tails.
3. Tails-heads.
4. Tails-tails.

The fallacy stems from thinking that *heads-tails* and *tails-heads* are the same.
In reality, these are to completely different outcomes.

The conditions of the problem states that at least one coin will land heads up.
This means that any outcome must contain at least one heads.
Now, if we list the possibe outcomes:

1. Heads-heads.
2. Heads-tails.
3. Tails-heads.
~~4. Tails-tails.~~

*Tails-tails* is the only non-valid outcome according to the contitions.
This leave three possible outcomes, of which one is the target: *heads-heads*.
Thus, the probablility of both coins landing heads up is 1 in 3 (~33%).

