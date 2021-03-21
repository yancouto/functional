use super::{Level, TestCase};

lazy_static! {
    pub static ref LEVELS: Vec<Level> = vec![Level {
        name: "anything".to_string(),
        description: "
        A variable is a lower case letter from a to z.

        If x is a variable, and M and N are terms, then:
        - x is a term
        - x: M is a term (called a function)
        - M N is a term (called an application)

        Examples of terms:
        - x
        - x: x
        - x: y: y x
        - (x: x x) (y: z)

        Write any valid term.
        ".to_string(),
        extra_info: None,
        test_cases: vec![
            // This should always pass
            TestCase::from("f: A", "A"),
        ]
    },Level {
        name: "identity".to_string(),
        description: "
        If A and B are terms, with A being the function A = x: M, then A B -> M[x=B], where M[x=B] means replacing all ocurrences of variable x in M with the term B.

        Examples:
        - (x: y) z -> y
        - (x: x) z -> z
        - (x: x y) (z: z) -> (z: z) y -> y
        - (x: (x: x) x) y -> (x: x) y -> y
        - (x: x x) (x: x x) -> (x: x x) (x: x x) -> (x: x x) (x: x x) -> ...

        Write an identity function, that is, a function that when applied to any term, reduces to that same term.
        
        ".to_string(),
        extra_info: Some("
            We say that A B reduces to M[x=B].

            Formally:
            - If M = x, then M[x=B] = B
            - If M = y, then M[x=B] = y
            - If M = y: N, then M[x=B] = y: N[x=B]
            - If M = x: N, then M[x=B] = x: N
            - If M = N O, then M[x=B] = N[x=B] O[x=B]
        ".to_string()
        ),
        test_cases: vec![
            TestCase::from("f: f A", "A"),
            TestCase::from("f: f B", "B"),
            TestCase::from("f: f (x: x)", "x:x"),
        ]
    }
    ];
}
