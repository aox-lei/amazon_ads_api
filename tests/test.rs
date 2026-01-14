
#[derive(Debug, PartialEq, Eq)]
pub enum A {
    A,
    B,
}

#[tokio::test]
async fn test() {
    let a = A::A;
    dbg!(a == A::A);
}
