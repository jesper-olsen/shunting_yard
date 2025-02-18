Shunting Yard Algorithm
=======================

Implementation of Dijkstra's Shunting Yard Algorithm [1,2] for converting expressions from infix to postfix form.

[1] [An ALGOL 60 Translator for the X1, E.W. Dijkstra, 1961](https://www.cs.utexas.edu/~EWD/MCReps/MR35.PDF) <br/>
[2] [Shunting Yard Algorithm](https://en.wikipedia.org/wiki/Shunting_yard_algorithm)<br/>
[3] [Top Down Operator Precedence, Vaughan R. Pratt, 1973](https://tdop.github.io/)<br/>

Run
===

```
% cargo run

> 3-4
Infix input:[Number(3.0), Operator(Minus), Number(4.0)]
Postfix    :[Number(3.0), Number(4.0), Operator(Minus)]
Result: -1

> sin ( max ( 2, 3 ) / 3 * 3.14 )
Infix input:[Function("sin"), LeftParen, Function("max"), LeftParen, Number(2.0), Comma, Number(3.0), RightParen, Operator(Divide), Number(3.0), Operator(Multiply), Number(3.14), RightParen]
Postfix    :[Number(2.0), Number(3.0), Number(3.0), Operator(Divide), Number(3.14), Operator(Multiply), Function("max"), Function("sin")]
Result: 0.0015926529164868282

> 3 + 4 * 2 / ( 1 - 5 ) ^ 2 ^ 3
Infix input:[Number(3.0), Operator(Plus), Number(4.0), Operator(Multiply), Number(2.0), Operator(Divide), LeftParen, Number(1.0), Operator(Minus), Number(5.0), RightParen, Operator(Pow), Number(2.0), Operator(Pow), Number(3.0)]
Postfix    :[Number(3.0), Number(4.0), Number(2.0), Operator(Multiply), Number(1.0), Number(5.0), Operator(Minus), Number(2.0), Number(3.0), Operator(Pow), Operator(Pow), Operator(Divide), Operator(Plus)]
Result: 3.0001220703125
```
