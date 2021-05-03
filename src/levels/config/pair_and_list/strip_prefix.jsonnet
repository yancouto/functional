local bool = import '../boolean/lib.libsonnet';
local lib = import 'lib.libsonnet';
{
  name: 'strip prefix',
  description: |||
    Using the Y combinator learned in previous problem, it's possible to use recursion. Recursion is the act of writing a self referential function.
    
    Suppose you want to write a function that receives two arguments and applies *itself* to both arguments in inverted order. You can write:
    F = Y (f: x:y: f y x)
    
    That is, you can write a function that receives itself as the first argument and apply the Y combinator to it. That can be applied in fact to any function.
    
    With that knowledge solve the following problem: you're given a list of booleans. Write a function F that strips all the contiguous FALSE at the beginning of the list and returns the rest of the list.
    
    It is guaranteed that the list has at least one TRUE, meaning the answer will be a non-empty list.
  |||,
  extra_info: |||
    Examples for the problem:
    - F [FALSE, FALSE, TRUE, TRUE, FALSE] -> [TRUE, TRUE, FALSE]
    - F [TRUE, FALSE] -> [TRUE, FALSE]
    - F [FALSE, TRUE] -> [TRUE]

    Let's reason why you can use Y in the way we said.
    
    Let F a self-referential function. Let's write it as
    F = (f: M) F
    
    where f in M are the locations in F that reference itself.
    
    If we define F = Y (f: M), from the definition of Y, we have:
    F = Y (f: M) = (f: M) (Y (f: M)) = (f: M) F = M[f=F]
  |||,
  before_level_constants: [
    ['Y', 'f: (x: f (x x)) (x: f (x x))'],
  ],
  test_cases: [
    ['f: f %s' % [lib.list(['FALSE', 'FALSE', 'TRUE', 'TRUE', 'FALSE'])], lib.list(['TRUE', 'TRUE', 'FALSE'])],
    ['f: f %s' % [lib.list(['TRUE', 'FALSE'])], lib.list(['TRUE', 'FALSE'])],
    ['f: f %s' % [lib.list(['FALSE', 'TRUE'])], lib.list(['TRUE'])],
  ],
  solutions: ['Y (f: p: (p TRUE) p (f (POP p)) )'],
}
