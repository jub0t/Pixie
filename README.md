# Interpreter

## Info

* Built-in functions are called `Methods`
* Types: `I32`, `String`, `F64`, `Bool`

## Example

```js
let name = "John Doe"
print(name)

const name = "Jane Doe"
print(name)

const cool = true
const age = 18
print(age, cool)

echo("Newlines Work Too\n")

function my_func(t1: String, t2: I32){
    print(t1)
    print(t2)
}

my_func("Function Params Work", 100)
```
