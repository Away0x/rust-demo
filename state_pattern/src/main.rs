mod oop;
mod rustify;

/// 状态模式 State Pattern
/// - 状态模式是一个面向对象设计模式
/// - 一个值拥有的内部状态由数个状态对象 (state object) 表达而成，而值的行为则随着内部状态的改变而改变
/// - 使用状态模式意味着
///   1. 业务需求变化时，不需要修改持有状态的值的代码，或者使用这个值的代码
///   2. 只需要更新状态对象内部的代码，以便改变其规则。或者增加一些新的状态对象
/// 缺点:
///   - 某些状态之间是相互耦合的
///   - 需要重复实现一些逻辑代码

fn main() {
    // 面向对象风格
    let mut post = oop::Post::new();
    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
    post.request_review();
    assert_eq!("", post.content());
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    // rust 风格
    // 将状态和行为编码为不同的类型
    // - Rust 类型检查系统会通过编译时错误来阻止用户使用无效的状态
    let mut post = rustify::Post::new();
    post.add_text("I ate a salad for lunch today");
    let post = post.request_review();
    let post = post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
