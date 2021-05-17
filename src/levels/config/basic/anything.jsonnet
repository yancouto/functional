{
  name: 'anything',
  description: |||
    A variable is a lower case letter from a to z.
    
    If x is a variable, and M and N are terms, then:
    - x is a term
    - (x: M) is a term (called a function)
    - (M N) is a term (called an application)
    
    Examples of terms:
    - x
    - x: x
    - x: y: y x
    - (x: x x) (y: z)
    
    Parenthesis can be removed if order is clear. For example:
    - (x: (y: x)) = x: y: x
    - ((a b) c)   = a b c
    
    Write any valid term.
  |||,
  extra_info: |||
    The goal of each level is to write a term that solves the problem statement. On this level, you simply need to write any valid term.
  |||,
  test_cases: [
    // This should always pass
    ['f: A', 'A'],
  ],
  solutions: ['x'],
}
