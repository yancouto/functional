{
    name: "fst",
    description: |||
        We can
    |||,
    test_cases: [
        ["f: (PAIR A B) f", "A"],
        ["f: (PAIR (x:x) X) f", "x:x"],
        ["f: (PAIR (x:x) X) f Z", "Z"],
    ],
    provides_constant: true,
    solutions: ["a:b: a", "TRUE"],
    wrong_solutions: ["FALSE"]
}