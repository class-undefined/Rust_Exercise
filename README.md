# Rust_Exercise
[toc]
## 特性

### 所有权

在Rust中，若声明有类似于Java或C++中的引用传递类型概念的变量，存在相同作用域下将地址赋值给另一个变量，则该变量的所有权将发生转移，原先的变量将**不可访问**。

#### 直接转移

```rust
/* Error: 进行类似于其他语言中的引用赋值，地址被新的变量获取，则会丧失所有权。 */
fn test2() {
    let a:Vec<i32> = Vec::new();
    a.push(1);
    let b = a; //将a的所有权转移给b
    println!("{}", a[0]); // 试图访问已丧失所有权的变量a
}

```

#### 间接转移

```rust
/* Error: _v变量的地址被赋值给了change函数实参v，所有权已转移 */
fn test3() {
    let _v:Vec<i32> = Vec::new();
    _v.push(1);
    /* change函数得到了传递过来的Vec实例的所有权 */
    let change = |v:Vec<i32>| -> () {
        return;
    };
    change(_v);
    println!("{}", _v[0]); // 试图访问已丧失所有权的变量_v
}
```



---



### 引用、借用

在Rust中，由于有**所有权**的特性，若想用另一个变量去读取其值进行一些操作，而又不会丢失其所有权，可以使用引用特性，使用方式是将`&`加在变量名前。

事实上这样的场景十分常见。

如，现在有一个`let nums:Vec<i32> = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10].to_vec();`，我们需要分别获取它们累加、累乘的结果。为了不丢失所有权，直接使用`nums`变量进行迭代获取结果显然是可行的，但并不优雅。我们可以分别实现累加、累成的函数进行操作。

这样的函数签名大概是这样的形式`fn sum(arr: Vec<i32>)->i32`、`fn mult(arr: Vec<i32>)->i32`。

但这会引发一个问题：**间接转移**`test3`代码段即存在这样的问题。当`_v`变量作为实参传入`change`函数时，`_v`的所有权就已经被转移到了`change`函数的形参`v`中，这意味着`change`一旦执行，`_v`则已丧失所有权，不可访问。

而这样的情况函数签名若是以`fn sum(arr: Vec<i32>)`、`fn mult(arr: Vec<i32>)`的形式，那么意味着我们至多只能执行其中的一个`sum`或是`mult`，因为`nums`的所有权会在执行他们的其中一个时就已经转移丢失了。

所以为了保持其原有的所有权，我们需要有一种借用它的方式，这样的方式就是引用。针对累加、累乘，我们就可以写出这样的代码。

```rust
fn sum(arr: &Vec<i32>) ->i32 {
    let mut ans = 0;
    for n in arr.iter() {
        ans += n;
    }
    ans
}
fn mult(arr: &Vec<i32>) ->i32 {
    let mut ans = 1;
    for n in arr.iter() {
        ans *= n;
    }
    ans
}
let nums:Vec<i32> = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10].to_vec();
let ans1 = sum(&nums); // 通过引用传递nums，保证nums不会丢失所有权，下同
let ans2 = mult(&nums);
println!("sum:{} mult:{}", ans1, ans2);
}
```





### 可变性与不可变性 

Rust的`let`关键字自带`const`（不可变）性质，但与JavaScript、C++的`const`关键字带来的效果略有差异。在JavaScript、C++以修饰的`const`的变量，只为了确保其变量所指向堆区的地址是否发生变化，而不关心其地址指向堆中的内存区域的数据是否有所改变。

在`TypeScript`中，以下代码段是合法的。

```typescript
const array: Array<int> = []; // 声明一个名为array，int类型的数组
array.push(1); // 为array添加一个元素1
```

在`C++`中，以下代码段是合法的。

```c++
const vector<int> array = vector<int>(); // 声明一个名为array，int类型的数组
array.push_back(1); // 为array添加一个元素1
```

而Rust不是这样的，任何变量，没有经过`mut`关键字的修饰，**无论是栈中的内存数据还是堆区的内存数据，都是不可改变的。**

```rust
/* Error: cannot borrow `array` as mutable, as it is not declared as mutable */
let array:Vec<i32> = Vec::new(); // 声明一个名为array，int类型的数组
array.push(1); // 尝试为array添加一个元素1，不可行
```

而如果想要为其添加元素，则必须使用`mut`关键字修饰。

```rust
/* 与javascript的const不同，Rust若不指定mut，则无法更改容器内的数据，这里的数据修改权不仅限于栈空间，更像是变量的可写权限？ */
fn test5() {
    let mut _v:Vec<i32> = Vec::new();
    _v.push(1); // 直接添加
    /* change函数得到了传递过来的Vec实例的借用权 */
    let change = |v:& mut Vec<i32>| -> () {
        v.push(3);
        return;
    };
    change(&mut _v);
    println!("{}", _v[1]); // 可以成功获取change函数添加的元素3
}
```



