// pub trait Monad<T> {
//     fn ret(t: T) -> Self;
//     fn bind<F>(&self, f: F) -> Self
//         where F: Fn(T) -> Self;
//         // where F: Fn(T) -> Self;
// }

// pub struct Return<T> {
//     value: T,
// }

// // impl<T> Monad<T> for Return<T> {
// //     fn ret(v: T) -> Self { Return { value: v } }
// //     fn bind(&self, f: (Fn(&T) -> Self)) -> Self {
// //         f(&self.value)
// //     }
// // }

// // impl<T> Monad<T> for Option<T> {
// //     fn value(&self) -> &T { &
// // }

