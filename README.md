Shunting Yard Algorithm
=======================

Implementation of Dijkstra's Shunting Yard Algorithm [1,2] for converting expressions from infix to postfix form.

[1] [An ALGOL 60 Translator for the X1, E.W. Dijkstra, 1961](https://www.cs.utexas.edu/~EWD/MCReps/MR35.PDF) <br/>
[2] [Shunting Yard Algorithm](https://en.wikipedia.org/wiki/Shunting_yard_algorithm)<br/>
[3] [Top Down Operator Precedence, Vaughan R. Pratt, 1973](https://tdop.github.io/)<br/>

Run
===

```% cargo run

> 3-4
Input Token: Number(3.0)
Input Token: Operator(Minus)
Input Token: Number(4.0)
Result: -1

> sin ( max ( 2, 3 ) / 3 * 3.14 )
Input Token: Function("sin")
Input Token: LeftParen
Input Token: Function("max")
Input Token: LeftParen
Input Token: Number(2.0)
Input Token: Comma
Input Token: Number(3.0)
Input Token: RightParen
Input Token: Operator(Divide)
Input Token: Number(3.0)
Input Token: Operator(Multiply)
Input Token: Number(3.14)
Input Token: RightParen
Result: 0.0015926529164868282

> 3 + 4 * 2 / ( 1 - 5 ) ^ 2 ^ 3
Input Token: Number(3.0)
Input Token: Operator(Plus)
Input Token: Number(4.0)
Input Token: Operator(Multiply)
Input Token: Number(2.0)
Input Token: Operator(Divide)
Input Token: LeftParen
Input Token: Number(1.0)
Input Token: Operator(Minus)
Input Token: Number(5.0)
Input Token: RightParen
Input Token: Operator(Exp)
Input Token: Number(2.0)
Input Token: Operator(Exp)
Input Token: Number(3.0)
Result: 3.0001220703125
>
```
