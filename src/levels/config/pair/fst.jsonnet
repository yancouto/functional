{
    name: "fst",
    description: |||
        Write function FST, a function that receives a pair and returns the first element.

        For example:
        - FST (A, B) -> A
    |||,
    extra_info: |||
        Notice that we're using (A, B) in the examples, but that is not valid syntax for a term. When we write that, we actually mean (PAIR A B).
    |||,
    test_cases: [
        ["f: f (PAIR A B)", "A"],
        ["f: f (PAIR (x:x) X)", "x:x"],
        ["f: f (PAIR (x:x) X) Z", "Z"],
    ],
    provides_constant: true,
    solutions: ["p: p (a:b: a)", "p: p TRUE"],
    wrong_solutions: ["p: p FALSE", "a:b: a"],
}