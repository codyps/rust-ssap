#[derive(PartialEq,Debug)]
pub enum Elem<'a> {
    Value(&'a [u8]),
    List(Vec<Elem<'a>>)
}

/// Parse data in the form:
/// [ foo [ bar baz ] [ jump [ joop ] ] ]
pub fn parse<'a>(data: &'a [&'a [u8]]) -> Vec<Elem<'a>>
{
    let mut base : Vec<Elem<'a>> = vec!{};
    let mut list_stack : Vec<Vec<Elem<'a>>> = vec!{};

    macro_rules! push {
        ($e:expr) => (
            match list_stack.pop() {
                Some(mut v) => {
                    v.push($e);
                    list_stack.push(v);
                },
                None => {
                    base.push($e);
                }
            }
        );
    }

    for d in data {
        match d {
            &b"[" => list_stack.push(vec!{}),
            &b"]" => match list_stack.pop() {
                    Some(l) => push!(Elem::List(l)),
                    None => return base,
            },
            v => push!(Elem::Value(v)),
        }
    }

    base
}

#[test]
fn it_works() {
    assert_eq!(vec![Elem::Value(b"foo")], parse(&vec![b"foo"]));
    assert_eq!(vec![Elem::Value(b"foo"),Elem::List(vec![Elem::Value(b"bar")])],
        parse(&vec![b"foo", b"[", b"bar", b"]"]));
}
