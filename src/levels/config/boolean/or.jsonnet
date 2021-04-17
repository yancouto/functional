{
    name: "or",
    description: |||
        Write function OR, that is, a function that is given two booleans, and only reduces to FALSE if both values are FALSE.

        Examples:
        - AND FALSE FALSE -> FALSE
        - AND FALSE TRUE  -> TRUE

        As always, the input is well formed, both arguments are booleans.
    |||,
    test_cases: [
        ["f: f TRUE TRUE", "TRUE"],
        ["f: f TRUE FALSE", "TRUE"],
        ["f: f FALSE TRUE", "TRUE"],
        ["f: f FALSE FALSE", "FALSE"],
    ],
    provides_constant: true,
    solutions: ["a:b: x:y: a x (b x y)", "a:b: NOT (AND (NOT a) (NOT b))"]
}
