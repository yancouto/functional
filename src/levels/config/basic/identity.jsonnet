{
  name: 'identity',
  description: |||
    We can reduce the term (x: ...) B by replacing all free ocurrences of x in ... with B.
    
    Some examples:
    - (x: y) z -> y
    - (x: x) z -> z
    - (x: x y) (z: z) -> (z: z) y -> y
    - (x: (x: x) z) y -> (x: x) z -> z
    - (x: x x) (x: x x) -> (x: x x) (x: x x) -> (x: x x) (x: x x) -> ...
    
    Each step is called a reduction. More formally, if A and B are terms, then (x: A) B -> A[x=B], where A[x=B] means replacing all ocurrences of variable x in A with the term B.
    
    We say that the function (x: A) is applied to the term B.
    
    Write an identity function, that is, a function that when applied to any term, reduces to that same term.
  |||,
  extra_info: |||
    We say that (x: A) B reduces to A[x=B].
    
    Formally:
    - If A = x, then A[x=B] = B
    - If A = y, then A[x=B] = y
    - If A = y: C, then A[x=B] = y: C[x=B]
    - If A = x: C, then A[x=B] = x: C (this is called shadowing, as we have an inner ocurrence of x)
    - If A = C D, then A[x=B] = C[x=B] D[x=B]
  |||,
  test_cases: [
    ['f: f A', 'A'],
    ['f: f B', 'B'],
    ['f: f (x: x)', 'x:x'],
    ['f: f (x: x) Z', 'Z'],
    ['f: f (a:b:c: c b b a)', 'a:b:c: c b b a'],
  ],
  solutions: ['x: x'],
}
