{
  name: 'creating recursion',
  description: |||
    Write a term Y such that, for any term F, we have
    Y F = F (Y F)
    
    This is called the Y-combinator. It is used to create recursion, because notice that:
    Y F = F (Y F) = F (F (Y F)) = F (F (F (F ... (Y F))))
    
    This is always infite, but just because of that, it doesn't mean that a reduction doesn't exist. If A is an infinite term (doesn't have a reduction) but B isn't, then "(a:b: b) A B" is finite too.

    This level is very hard. It's mind twisting to create this recursive structure, feel free to skip it and go to the next level.
    
    The most interesting part is actually using this result, not coming up with it.
  |||,
  extra_info: |||
    If you consider W = (x: x x), then W W -> W W, the term reduces to itself.
    
    For Y, you want to modify W in such a way that (f: W W) F -> F ((f: W W) F).
  |||,
  extra_info_is_hint: true,
  test_cases: [
    ['y: y (f: x: x) A', 'A'],
    ['y: y (f: l: (l TRUE) (f (l FALSE))) (PAIR (x:Z) Y)', 'Z'],
    ['y: y (f: l: (l TRUE) (f (l FALSE))) (PAIR (x: x C) (PAIR (x:A) B))', 'A C'],
  ],
  solutions: ['f: (x: f (x x)) (x: f (x x))'],
}
