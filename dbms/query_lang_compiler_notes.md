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
start
:Select;
split
:Mult;
split again
repeat
:AggregationExpression;
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
