use std::borrow::Cow;

fn abs_all(input: &mut Cow<[i32]>) {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            input.to_mut()[i] = -v;
        }
    }

    println!("value: {:?}", input);
}

fn main() {
    // 只读，不写，没有发生复制操作
    let a = [0, 1, 2];
    let mut input = Cow::from(&a[..]);
    abs_all(&mut input);
    assert_eq!(input, Cow::Borrowed(a.as_ref()));

    // 写时复制， 在读到-1的时候发生复制
    let b = [0, -1, -2];
    let mut input = Cow::from(&b[..]);
    abs_all(&mut input);
    assert_eq!(input, Cow::Owned(vec![0, 1, 2]) as Cow<[i32]>);

    // 没有写时复制，因为已经拥有所有权
    let mut input = Cow::from(vec![0, -1, -2]);
    abs_all(&mut input);
    assert_eq!(input, Cow::Owned(vec![0, 1, 2]) as Cow<[i32]>);

    let v = input.into_owned();
    assert_eq!(v, [0, 1, 2]);
}

// 设计函数输入参数的时候，我们会停顿一下，这里，用 &str 好呢，还是 String 好呢？思考一番，从性能上考虑，有如下结论：
//
// 如果使用 String， 则外部在调用此函数的时候，
// 如果外部的字符串是 &str，那么，它需要做一次克隆，才能调用此函数；
// 如果外部的字符串是 String，那么，它不需要做克隆，就可以调用此函数。但是，一旦调用后，外部那个字符串的所有权就被 move 到此函数中了，外部的后续代码将无法再使用原字符串。
// 如果使用 &str，则不存在上述两个问题。但可能会遇到生命周期的问题，需要注意。
// 继续分析上面的例子，我们发现，在函数体内，做了一次新字符串对象的生成和拷贝。
//
// 让我们来仔细分析一下业务需求。最坏的情况下，如果字符串中没有空白字符，那最好是直接原样返回。这种情况做这样一次对象的拷贝，完全就是浪费了。
//
// 于是我们心想改进这个算法。很快，又遇到了另一个问题，返回值是 String 的嘛，我不论怎样，要把 &str 转换成 String 返回，始终都要经历一次复制。于是我们快要放弃了。

fn remove_spaces<'a>(input: &'a str) -> Cow<'a, str> {
    if input.contains(' ') {
        let mut buf = String::with_capacity(input.len());

        for c in input.chars() {
            if c != ' ' {
                buf.push(c);
            }
        }

        return Cow::Owned(buf);
    }

    return Cow::Borrowed(input);
}
