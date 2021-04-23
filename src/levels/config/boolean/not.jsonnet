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
        You can use constants from previous levels. See below for the full list of allowed constants.

        Notice that you don't NEED to use the constants. And using them as little as possible means solving the problem using the least reductions.
    |||,
    test_cases: [
        ["f: f TRUE", "FALSE"],
        ["f: f FALSE", "TRUE"],
        ["f: (f TRUE) Z Y", "Y"],
    ],
    provides_constant: true,
    solutions: ["b: x:y: b y x", "b: IF b FALSE TRUE"],
    wrong_solutions: ["f: f"]
}
