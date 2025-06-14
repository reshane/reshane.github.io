# Golang & strings

A quick post about golang and its strings.

## The Background

I was working on a function that grabs the first n characters of a string in golang and came across something interesing.

Golang strings are utf-8 encoded by default, and support things like umlauts and accents out of the box.

_However..._

indexing a string like so...
```go
"Hello, World!"[0:2]
```
...is done using a *byte* offset.

This means that ASCII is going to work great, and all of your tests will pass in the standard english problem space.

## The Problem

As soon as your program encounters anything above single byte ASCII, it will truncate halfway through characters.

For example, the output of the following...
```go
package main

import "fmt"

func main() {
	fmt.Println("Müller"[0:2])
}
```
...is this abomination...
```
M�
```

## The Solution

The golang utf-8 charcter is called a `rune`, and is its own datatype.

Thankfully, as string can be trivially converted into a `[]rune`, which is then indexed at the utf-8 character boundary.

So something like this...
```go
package main

import "fmt"

func main() {
	fmt.Println(string([]rune("Müller")[0:2]))
}
```
...Will actually produce the expected output...
```
Mü
```

## Thoughts

I am struggling to see the benefit of defaulting to byte indexed strings when they are automatically utf-8 encoded.

Is there a significant enough performance overhead in the alternative? (rune indexing by default)

It would be interesting to try optimizing rune indexing of strings.

