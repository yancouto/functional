{
    name: "if",
    description: |||
        Write function IF, that is, a three-parameter function that is given a boolean, and two other values (say, A and B). If the boolean is TRUE, then it reduces to A, otherwise, it reduces to B. 

        For example:
        - IF TRUE a b -> a
        - IF FALSE a b -> b

        As always, input is well formed, so the first argument is always a boolean.
    |||,
    test_cases: [
        ["f: f TRUE A B", "A"],
        ["f: f FALSE A B", "B"],
        ["f: f FALSE X Y", "Y"],
        ["f: f TRUE  X Y", "X"],
    ],
    show_constants: false,
    provides_constant: true,
    solutions: ["b:x:y: b x y"]
}
