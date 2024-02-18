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

## Grammar rules 
(sentences)

```plantuml
title DML query
start
split
:SelectStatement;
split again
:InsertStatement;
split again
:DeleteStatement;
split again
:UpdateStatement;
end split
:DMLQueryPrime;
end
```

```plantuml
title DML Query Prime
start
split
split again
:Semicolon;
:DMLQuery;
:DMLQueryPrime;
end split
end
```

### Select statement

```plantuml
@startuml Select statement
title Select statement
start
:Select;
partition Selection {
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
}
:From;
partition SourceTables {
repeat
:Identifier;
repeat while (Comma?)
}
split
split again
:Where;
:BooleanExpression;
end split
end
@enduml
```

### Boolean expression

```plantuml
@startuml boolean expression
title BooleanExpression (with left recursion and without parenthesis-handling)
start
split
:OpeningParenthesis;
:BooleanExpression;
:ClosingParenthesis;
split again
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

#### After removing left recursion and adding parenthesis-handling

```plantuml
@startuml boolean expression
title BooleanExpression
start
split
:Not;
:BooleanExpression;
split again
:OpeningParenthesis;
:BooleanExpression;
:ClosingParenthesis;
split again
:ComparedValue;
partition ComparisonOperator {
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
}
:ComparedValue;
end split
:BooleanExpressionPrime;
end
@enduml
```

```plantuml
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
:BooleanExpressionPrime;
end split
end
@enduml
```

---
#### Compared value

```plantuml
title ComparedValue
start
split
:Identifier;
split
:Dot;
:Identifier;
split again
end split
split again
:Value;
split again
:Sub;
:ComparedValue;
split again
:OpeningParenthesis;
:ComparedValue;
:ClosingParenthesis;
end split
:ComparedValuePrime;
end
```

```plantuml
title ComparedValuePrime
start
split
split again
:NumericalOperator;
:ComparedValue;
:ComparedValuePrime;
end split
end
```

### Insert statement

```plantuml
title Insert statement
start
:Insert;
:Into;
:Identifier;
:OpeningParenthesis;
repeat
:Identifier;
repeat while (Comma?)
:ClosingParenthesis;
:Values;
:OpeningParenthesis;
repeat
:ConstantCalculatedExpression;
repeat while (Comma?)
:ClosingParenthesis;
end
```

#### Constant calculated expression

```plantuml
title Constant calculated expression
start
split
:Value;
split again
:Not;
:ConstantCalculatedExpression;
split again
:Sub;
:ConstantCalculatedExpression;
split again
:OpeningParenthesis;
:ConstantCalculatedExpression;
:ClosingParenthesis;
end split
:ConstantCalculatedExpressionPrime;
end
```

```plantuml
title Constant calculated expression prime
start
split
split again
split
:NumericalOperator;
split again
:ComparisonOperator;
split again
:And;
split again
:Or;
end split
:ConstantCalculatedExpression;
:ConstantCalculatedExpressionPrime;
end split
end
```

#### Calculated expression

```plantuml
title Calculated expression
start
split
:Identifier;
split
split again
:Dot;
:Identifier;
end split
split again
:Value;
split again
:Sub;
:CalculatedExpression;
split again
:Not;
:CalculatedExpression;
split again
:OpeningParenthesis;
:CalculatedExpression;
:ClosingParenthesis;
end split
:CalculatedExpressionPrime;
end
```

```plantuml
title Calculated expression prime
start
split
split again
split
:NumericalOperator;
split again
:ComparisonOperator;
split again
:And;
split again
:Or;
end split
:CalculatedExpression;
:CalculatedExpressionPrime;
end split
end
```

### Update statement

```plantuml
title Update statement
start
:Update;
:Identifier;
:OpeningParenthesis;
repeat
:Identifier;
repeat while (Comma?)
:ClosingParenthesis;
:Values;
:OpeningParenthesis;
repeat
:CalculatedTuple;
repeat while (Comma?)
:ClosingParenthesis;
:Where;
:BooleanExpression;
end
```

### Delete statement

```plantuml
title Delete statement
start
:Delete;
:From;
:Identifier;
:Where;
:BooleanExpression;
end
```
