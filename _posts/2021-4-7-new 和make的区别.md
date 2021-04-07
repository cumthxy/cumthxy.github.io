---
layout: post
title: Go new和make的区别
categories: golang
description: Go new和make的区别
keywords: golang new make
---

## 1

对于引用类型变量，不光要声明，同时也需要为他分配内存空间。

对于值类型的，则不需要，默认分配好了。





### new



```golang
// The new built-in function allocates memory. The first argument is a type,
// not a value, and the value returned is a pointer to a newly
// allocated zero value of that type.
func new(Type) *Type
```



只接受一个参数，这个参数是一个类型，new(T) 为一个 T 类型新值分配空间并将此空间初始化为 T 的零值，返回的是新值的地址，也就是 T 类型的指针 *T，该指针指向 T 的新分配的零值。

### make



make也是用于内存的分配，只用于chan ,map,以及切片的内存创建。返回的类型就是这三个类型的本身。而不是他们的指针类型，因为这三种类型就是引用类型。

```golang
 slice, map, or chan (only)
```

### 联系

二者都是内存的分配（堆上），但是`make`只用于slice、map以及channel的初始化（非零值）；而`new`用于类型的内存分配，并且内存置为零。所以在我们编写程序的时候，就可以根据自己的需要很好的选择了。

`make`返回的还是这三个引用类型本身；而`new`返回的是指向类型的指针。









