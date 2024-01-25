{
  name: 'node',
  description: |||
    Let L, V and R be terms, we define a tree node as follows:
    - (L V R) = f: f L V R
    
    That is, a function that receives one argument and applies it to L, V and R.
    
    Write NODE, a function that receives three arguments L, V and R, and returns a tree node made of the three elements.
  |||,
  extra_info: |||
    You can see a tree node is very similar to a pair, but made of three elements. We will use it to build the definition of a tree in the following problems.
  |||,
  test_cases: [
      ['n: n L V R', 'f: f L V R'],
      ['n: (n L V R) (a:b:c: a)', 'L'],
      ['n: (n L V R) (a:b:c: b)', 'V'],
      ['n: (n L V R) (a:b:c: c)', 'R'],
      ['n: (n (x: x x) (x: B) A) (a:b:c: a c)', 'A A'],
  ],
  provides_constant: true,
  solutions: ['l:v:r: f: f l v r'],
  wrong_solutions: ['PAIR', 'l:r:v: f: f l v r', 'l:v:r: l v r'],
}