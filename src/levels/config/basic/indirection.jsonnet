{
    name: "indirection",
    description: |||
        Write a function SWAP_ARGS that receives a two argument function and will swap the values of the arguments it receives.

        Examples:
        - SWAP_ARGS (a:b: a b) x y -> y x
        - SWAP_ARGS (a:b: a a) x y -> y y
    |||,
    test_cases: [
        ["f: f (a:b: a b) X Y", "Y X"],
        ["f: f (a:b: a a) A B", "B B"],
        ["f: f (a:b: a b) A (x: x)", "A"],
    ],
    solutions: ["f:a:b: f b a"]
}