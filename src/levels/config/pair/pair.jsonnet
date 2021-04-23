{
    name: "pair",
    description: |||
        We can
    |||,
    test_cases: [
        ["p: p A B", "f: f A B"],
        ["p: (p A B) (a:b: a)", "A"],
        ["p: (p A B) (a:b: b)", "B"],
    ],
    provides_constant: true,
    solutions: ["a:b: f: f a b"]
}