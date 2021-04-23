local bool = ["FALSE", "TRUE"];
local cases = [
    [bool[x] for x in case]
    for case in [[0, 0, 0, 0], [1, 1, 1, 1], [1, 0, 0, 1], [0, 1, 0, 1], [0, 0, 1, 1], [0, 1, 1, 1]]
];

{
    name: "any",
    description: |||
        Write a function that, given a list of three booleans, reduces to TRUE if at least one of the booleans is TRUE, and to FALSE if all are FALSE.
    |||,
    test_cases: [
        ["f: f (PAIR %s (PAIR %s %s))" % [c[0], c[1], c[2]], c[3]]
        for c in cases
    ],
    solutions: ["f: (OR (SND (SND f)) (OR (FST f) (FST (SND f))))", "f: OR (FST f) ((SND f) OR)", "f: f (a:b: (OR a (b OR)))"],
    wrong_solutions: ["AND AND"]
}