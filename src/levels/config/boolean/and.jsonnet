{
    name: "and",
    description: |||
        Write function AND, that is, a function that is given two booleans, and only reduces to TRUE if both values are TRUE.

        Examples:
        - AND TRUE TRUE  -> TRUE
        - AND FALSE TRUE -> FALSE

        As always, the input is well formed, both arguments are booleans.
    |||,
    test_cases: [
        ["f: f TRUE TRUE", "TRUE"],
        ["f: f TRUE FALSE", "FALSE"],
        ["f: f FALSE TRUE", "FALSE"],
        ["f: f FALSE FALSE", "FALSE"],
    ],
    provides_constant: true,
    solutions: ["a:b: x:y: a (b x y) y"]
}
