{
    name: "xor",
    description: |||
        Write function XOR, that is, a function that is given two booleans, reduces to TRUE when exactly one of them is true.

        Examples:
        - XOR FALSE FALSE -> FALSE
        - XOR TRUE TRUE   -> FALSE
        - XOR TRUE FALSE  -> TRUE

        As always, the input is well formed, both arguments are booleans.
    |||,
    test_cases: [
        ["f: f TRUE TRUE", "FALSE"],
        ["f: f TRUE FALSE", "TRUE"],
        ["f: f FALSE TRUE", "TRUE"],
        ["f: f FALSE FALSE", "FALSE"],
    ],
    solutions: ["a:b: x:y: a (b y x) (b x y)"]
}
