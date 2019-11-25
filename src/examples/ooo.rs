struct Foo {}

impl Foo {
    pub fn bar(&self) {
        println!("bar");
    }

    pub fn baz(&self) {
        println!("baz");
    }
}

trait MyTrait {}

struct MyStruct {}

impl MyTrait for MyStruct {}

struct MyOtherStruct<P> {
    inner: P
}

impl<P> MyOtherStruct<P> where P: MyTrait {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thing() {
        let foo = Foo {};
        foo.bar();
        foo.baz();
    }

    #[test]
    fn test_other_thing() {
        let s = MyStruct {};
        let os = MyOtherStruct {
            inner: s
        };

        let some = Some(os);
    }
}
