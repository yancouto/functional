{
    name: "simple list",
    description: |||
        A simple list can be create using pairs. It can be defined recursively as:
        - A list with a single term M is simply M.
        - A list with two or more terms, is represented as (M, rest), where rest is the rest of the list.

        So, for example:
        - (a, (b, c)) = f: f a (g: g b c) is a list of three elements.

        Write a function that, given a list of five elements, outputs the third.
    |||,
    test_cases: [
        ["f: f (PAIR A (PAIR B (PAIR C (PAIR D E))))", "C"],
        ["f: f (PAIR X (PAIR Y (PAIR Z (PAIR W A))))", "Z"],
    ],
    solutions: ["f: f FALSE FALSE TRUE"],
}