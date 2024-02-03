# Query language compiler notes

## Grammar primitives

- Select
- Insert
- Delete
- From
- Into
- Values
- Where
- Comma
- Semicolon
- Dot
- Identifier
- Value
  - Boolean
  - String
  - Integer
  - Float
- Parenthesis
  - Opening
  - Closing
- LogicalOperator
  - Or
  - And
  - Not
- ComparisonOperator
  - ==
  - !=
  - <
  - <=
  - \>
  - \>=
- NumericalOperator
  - Add
  - Sub
  - Mult
  - Div

## Grammar rules (sentences)

### Select statement

```puml
@startuml Select statement
title Select statement
start
:Select;
split
:Mult;
split again
repeat
:Identifier;
split
split again
:Dot;
:Identifier;
end split
repeat while (Comma?)
end split
:From;
repeat
:Identifier;
repeat while (Comma?)
split
split again
:Where;
repeat
:BooleanExpression;
repeat while (Comma?)
end split
:Semicolon;
stop
@enduml
```

### Boolean expression

```puml
@startuml boolean expression
title BooleanExpression (with left recursion and without parenthesis-handling)
start
split
:BooleanExpression;
split
:And;
split again
:Or;
end split
:BooleanExpression;
split again
:Not;
:BooleanExpression;
split again
split
:Identifier;
split
split again
:Dot;
:Identifier;
end split
split again 
:Value;
end split
split
:Equals;
split again
:NotEquals;
split again
:Less;
split again
:LessEquals;
split again
:More;
split again
:MoreEquals;
end split
split
:Identifier;
split
split again
:Dot;
:Identifier;
end split
split again 
:Value;
end split
end split
end
@enduml
```

#### After removing left recursion and adding parenthesis-handling:

```puml
@startuml
title BooleanExpression
start
split
:BooleanExpressionBody;
split again
:OpenParenthesis;
:BooleanExpressionBody;
:ClosedParenthesis;
end split
:BooleanExpressionPrime;
end
@enduml
```

```puml
@startuml boolean expression
title BooleanExpressionBody
start
split
:Not;
:BooleanExpression;
split again
split
:Identifier;
split
split again
:Dot;
:Identifier;
end split
split again 
:Value;
end split
split
:Equals;
split again
:NotEquals;
split again
:Less;
split again
:LessEquals;
split again
:More;
split again
:MoreEquals;
end split
split
:Identifier;
split
split again
:Dot;
:Identifier;
end split
split again 
:Value;
end split
end split
end
@enduml
```

```puml
@startuml boolean expression prime
title BooleanExpressionPrime
start
split
split again
split
:And;
split again
:Or;
end split
:BooleanExpression;
end split
end
@enduml
```

