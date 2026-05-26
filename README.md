It's `tealang`

> Sip a tea and work on Teal :)

```teal
let five = 5;
let ten = 10;

add <- fn(x, y) {
    x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;

Calculator <- struct {
    name: str
}

Calculator <- new(name) {
    return Calculator {name: name};
}

Calculator <- extend(self) {
    add <- fn(a, b) {return a + b}
    sub <- fn(a, b) {return a - b}
}
```
