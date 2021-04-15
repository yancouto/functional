
{
    name: "identity",
    description: |||
        If A and B are terms, with A being the function A = x: M, then A B -> M[x=B], where M[x=B] means replacing all ocurrences of variable x in M with the term B.

        Examples:
        - (x: y) z -> y
        - (x: x) z -> z
        - (x: x y) (z: z) -> (z: z) y -> y
        - (x: (x: x) x) y -> (x: x) y -> y
        - (x: x x) (x: x x) -> (x: x x) (x: x x) -> (x: x x) (x: x x) -> ...

        Write an identity function, that is, a function that when applied to any term, reduces to that same term.
    |||,
    extra_info: |||
        We say that A B reduces to M[x=B].

        Formally:
        - If M = x, then M[x=B] = B
        - If M = y, then M[x=B] = y
        - If M = y: N, then M[x=B] = y: N[x=B]
        - If M = x: N, then M[x=B] = x: N
        - If M = N O, then M[x=B] = N[x=B] O[x=B]
    |||,
    test_cases: [
        ["f: f A", "A"],
        ["f: f B", "B"],
        ["f: f (x: x)", "x:x"],
    ],
    solutions: ["x: x"]
}