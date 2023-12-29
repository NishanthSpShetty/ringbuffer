## RingBuffer

Circular/Ring buffer implementation 


### Usage

create a new instance of RingBuffer
```
 let buffer = ring::RingBuffer::<i32>::new(5);

```
This will create a buffer with capacity 5

### Offer and poll

Write and read from buffer using offer and poll API
```
let res = buffer.offer(10);
if res.is_err(){
    //failed to insert, may be full
}


//read from the buffer
let val = buffer.poll();
if val.is_none(){
    //poll returned None, may be empty?
}
```

### Implementations in other languages
Go implementation  [here](https://github.com/NishanthSpShetty/goring)

Zig implementation [here](https://github.com/NishanthSpShetty/zigring)

