Maxilla
=======

An experimental tool to explain the parse trees of Rust programs.

Named after the appendages crustaceans use to move food into their mouths:
  https://en.wikipedia.org/wiki/Maxilla_(arthropod_mouthpart)#Crustaceans


Current example
---------------

```
fn main() { let greeting = greet("Dave"); print!(greeing); }
fn greet(name: &str) -> &str { "Hello, " + name }
A module declaration containing:

    fn main() { let greeting = greet("Dave"); print!(greeing); }
    A safe inherited-visibility declaration of a function named main containing:

        { let greeting = greet("Dave"); print!(greeing); }
        A block containing:

            let greeting = greet("Dave");
            A declaration of a local assigning the symbol greeting of type "undeclared" to "ExprCall(expr(4294967295: greet), [expr(4294967295: \"Dave\")])"


            print!(greeing);
            some other statement


    fn greet(name: &str) -> &str { "Hello, " + name }
    A safe inherited-visibility declaration of a function named greet containing:

        name: &str
        An argument containing:

            &str
            TyRptr(None, MutTy { ty: type(str), mutbl: MutImmutable })


            name
            the symbol name


        { "Hello, " + name }
        A block containing:

            "Hello, " + name
            ExprBinary(Spanned { node: BiAdd, span: Span { lo: BytePos(102), hi: BytePos(103), expn_id: ExpnId(4294967295) } }, expr(4294967295: "Hello, "), expr(4294967295: name))
```
