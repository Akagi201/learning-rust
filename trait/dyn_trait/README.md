[enum](/enum/methods/) vs [dyn_trait](/trait/dyn_trait/)

这两种不同的实现方法主要区别在于使用枚举类型和 Trait Object 的方式。使用枚举类型的方法在编译时可以进行静态类型检查，但需要手动处理每个具体类型的方法调用。而使用 Trait Object 的方式可以在运行时动态地处理不同类型的对象，但会带来一些性能开销。
