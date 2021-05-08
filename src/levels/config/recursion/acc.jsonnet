local bool = import '../boolean/lib.libsonnet';
local pl = import '../pair_and_list/lib.libsonnet';
{
  name: 'acc',
  description: |||
    Write a function ACC that takes as arguments a list L, a function F and a value I.
    
    F is a two argument function that reduces to a single argument.
    
    Starting the accumulated value X as I, replace X with F X Y for every element Y of the list L. Your function must then resolve to the last value of X.
    
    Examples:
    - ACC [TRUE, FALSE] OR FALSE -> TRUE
    - ACC [] OR FALSE -> FALSE
  |||,
  extra_info: |||
    We call this process accumulation, and it is used to condense the values of a list to a single value.
    
    In the examples, we're taking the OR of a list of booleans, but this can be much generic than that.
  |||,
  test_cases: [
    bool.test_true('f: f %s OR FALSE' % [pl.list(['TRUE', 'FALSE'])]),
    bool.test_false('f: f %s OR FALSE' % [pl.list(['FALSE', 'FALSE', 'FALSE'])]),
    bool.test_false('f: f %s OR FALSE' % [pl.list([])]),
    bool.test_false('f: f %s XOR FALSE' % [pl.list(['TRUE', 'FALSE', 'TRUE'])]),
    bool.test_true('f: f %s XOR FALSE' % [pl.list(['TRUE', 'FALSE'])]),
  ],
  provides_constant: true,
  solutions: ['l:f:i: Y (g: l:x: l (h:t:d: g t (f x h)) x) l i', 'Y (g: l:f:i: l (h:t:d: g t f (f i h)) i)'],
}
