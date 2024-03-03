use std::borrow::Cow;

/*
  `std::borrow::Cow`是Rust标准库中的一个枚举（enum），全称为“Copy on Write”。
  `Cow`是一种智能指针，用于提供对数据的克隆（写时复制）能力。
  它主要用于优化性能，减少不必要的数据复制。`Cow`枚举有两个变体：

- `Borrowed(&'a B)`：这个变体表示数据是借用的，其中`'a`是生命周期，`B`是被借用的数据类型。
- `Owned(B)`：这个变体表示数据是拥有的，`B`是数据的类型。

使用`Cow`可以在需要时自动决定是直接使用借用的数据，还是需要对数据进行克隆以获得所有权。
这在处理可能被多次复制但实际上很少修改的数据时特别有用。
`Cow`允许你在需要修改数据时才进行克隆，从而避免了不必要的数据复制开销。

简单的使用场景包括：

- 当你想要函数既能接受拥有数据的所有权的参数，又能接受借用的数据作为参数时，使用`Cow`可以使得函数接口更加灵活。
- 当你处理的数据通常不需要修改，但在某些情况下需要修改时，使用`Cow`可以避免前期不必要的克隆，只在确实需要修改时才进行克隆，从而提高效率。

使用`Cow`的一个关键点是理解其生命周期和所有权的概念，以及如何根据上下文自动转换`Borrowed`和`Owned`变体。
 */

// 一个处理字符串的函数，接受一个字符串切片或者一个String对象
// 如果字符串不以"hello"开头，则在前面添加"hello, "
// 这个函数展示了Cow的两种使用场景：借用和拥有
fn ensure_hello<'a>(input: Cow<'a, str>) -> Cow<'a, str> {
    if input.starts_with("hello") {
        // 如果已经以"hello"开头，直接返回输入的Cow，不需要修改
        input
    } else {
        // 如果不是以"hello"开头，创建一个新的字符串，并转换为Cow::Owned
        // 这里发生了所有权的转换：从可能的借用到拥有
        Cow::Owned(format!("hello, {}", input))
    }
}

fn main() {
    // 使用借用的数据
    let borrowed_str = "world";
    let result = ensure_hello(Cow::Borrowed(borrowed_str));
    println!("Borrowed result: {}", result);

    // 使用拥有的数据
    let owned_str = String::from("rust");
    let result = ensure_hello(Cow::Owned(owned_str));
    println!("Owned result: {}", result);
}
