{
    name: "not",
    description: |||
        We can define booleans as follows:
        - TRUE  = a: b: a
        - FALSE = a: b: b

        That is, TRUE is a two argument function that returns the first element, and FALSE returns the second.

        Write function NOT, that is, a function that if given TRUE returns FALSE, and if given FALSE returns TRUE.

        Note that you can assume that the received values are always booleans! Input is always well formed.
    |||,
    extra_info: |||
        Explain something about constants.
    |||,
    test_cases: [
        ["f: f TRUE", "FALSE"],
        ["f: f FALSE", "TRUE"],
    ]
}
