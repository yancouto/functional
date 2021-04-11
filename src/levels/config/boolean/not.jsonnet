{
    name: "not",
    description: |||
        We can define booleans as follows:
        - TRUE  = a: b: a
        - FALSE = a: b: b

        That is, TRUE is a two argument function that returns the first element, and FALSE returns the second.

        Write function NOT, that is, a function that if given TRUE returns FALSE, and if given FALSE returns TRUE.

        For example:
        - (NOT TRUE) a b -> FALSE a b -> b

        Note that you can assume that the received values are always booleans! Input is always well formed.
    |||,
    extra_info: |||
        You can use constants from previous levels. See below for the full list of allowed constants (TODO).
    |||,
    test_cases: [
        ["f: f TRUE", "FALSE"],
        ["f: f FALSE", "TRUE"],
        ["f: (f TRUE) Z Y", "Y"],
    ]
}
