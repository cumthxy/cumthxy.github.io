---
layout: post
title: Go new和make的区别
categories: golang
description: Go new和make的区别
keywords: golang new make
---

## 1



channe源码位于`runtime`中`的chan.go` 源码如下：

```go
type hchan struct {
	qcount   uint           // channel中元素的个数
	dataqsiz uint           // 底层循数组的长度
	buf      unsafe.Pointer // 指向底层循环数组的指针，而且只针对有缓冲的channel
	elemsize uint16   //channel中元素大小
	closed   uint32  // channel是否被关闭的标志
	elemtype *_type // channel中元素类型
	sendx    uint   // channel的发送操作处理到的位置
	recvx    uint   // channel的接受操作处理到的位置
	recvq    waitq  // 等待接受groutine队列
	sendq    waitq  // 等待发送的groutine队列
	lock mutex
}
```

其中，`recvq`，`sendq` 存储了当前的channel由于缓冲区不足而阻塞的`groutine`，这些等待的队列使用双向链表 [`runtime.waitq`](https://draveness.me/golang/tree/runtime.waitq) 表示，链表中所有的元素都是 [`runtime.sudog`](https://draveness.me/golang/tree/runtime.sudog) 结构.

```go
type waitq struct {
	first *sudog
	last  *sudog
}
```

[`runtime.sudog`](https://draveness.me/golang/tree/runtime.sudog) 表示一个在等待列表中的 `Goroutine`，该结构中存储了两个分别指向前后 [`runtime.sudog`](https://draveness.me/golang/tree/runtime.sudog) 的指针以构成链表。

`lock` 用来保证每个读 channel 或写 channel 的操作都是原子的。

```go
func makechan(t *chantype, size int) *hchan {
	elem := t.elem

	// compiler checks this but be safe.
	if elem.size >= 1<<16 {
		throw("makechan: invalid channel element type")
	}
	if hchanSize%maxAlign != 0 || elem.align > maxAlign {
		throw("makechan: bad alignment")
	}

	mem, overflow := math.MulUintptr(elem.size, uintptr(size))
	if overflow || mem > maxAlloc-hchanSize || size < 0 {
		panic(plainError("makechan: size out of range"))
	}

	// Hchan does not contain pointers interesting for GC when elements stored in buf do not contain pointers.
	// buf points into the same allocation, elemtype is persistent.
	// SudoG's are referenced from their owning thread so they can't be collected.
	// TODO(dvyukov,rlh): Rethink when collector can move allocated objects.
	var c *hchan
	switch {
	case mem == 0:
		// Queue or element size is zero.
		c = (*hchan)(mallocgc(hchanSize, nil, true))
		// Race detector uses this location for synchronization.
		c.buf = c.raceaddr()
	case elem.ptrdata == 0:
		// Elements do not contain pointers.
		// Allocate hchan and buf in one call.
		c = (*hchan)(mallocgc(hchanSize+mem, nil, true))
		c.buf = add(unsafe.Pointer(c), hchanSize)
	default:
		// Elements contain pointers.
		c = new(hchan)
		c.buf = mallocgc(mem, elem, true)
	}

	c.elemsize = uint16(elem.size)
	c.elemtype = elem
	c.dataqsiz = uint(size)
	lockInit(&c.lock, lockRankHchan)

	if debugChan {
		print("makechan: chan=", c, "; elemsize=", elem.size, "; dataqsiz=", size, "\n")
	}
	return c
}
```

