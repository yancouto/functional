{
    name: "and",
    description: |||
        We can define booleans as follows:
        - TRUE  = a: b: a
        - FALSE = a: b: b

        That is, TRUE is a two argument function that returns the first element, and FALSE returns the second.

        Write function AND, that is, a function that is given two booleans, and only returns TRUE if both values are TRUE.

        Note that you can assume that the received values are always booleans! Input is always well formed.
    |||,
    test_cases: [
        ["f: f TRUE TRUE", "TRUE"],
        ["f: f TRUE FALSE", "FALSE"],
        ["f: f FALSE TRUE", "FALSE"],
        ["f: f FALSE FALSE", "FALSE"],
    ]
}
