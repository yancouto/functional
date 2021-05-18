local bool_lib = import '../boolean/lib.libsonnet';
local lib = import 'lib.libsonnet';
{
  name: 'empty',
  description: |||
    Write function EMPTY, that given a list L, reduces to TRUE if it's empty, and FALSE if it's not.
    
    For example:
    - EMPTY [A] = EMPTY (A, FALSE) -> FALSE
    - EMPTY [] = EMPTY FALSE -> TRUE
  |||,
  extra_info: |||
    If you have a list L, then "L A B" will resolve to B if L is empty, otherwise, it will resolve to "A h t B", where h is the first element of the list and t is the rest of the list.
    
    You can set A as a three argument function that receives the first element, the rest, and a dummy argument (which will always be B).
    
    This is a generic way to deal with lists, and applicable in many other list problems.
  |||,
  extra_info_is_hint: true,
  test_cases: [
    bool_lib.test_false('f: f %s' % [lib.list(['A'])]),
    bool_lib.test_true('f: f %s' % [lib.list([])]),
    bool_lib.test_true('f: f (POP %s)' % [lib.list(['A'])]),
    bool_lib.test_false('f: f (PUSH A FALSE)'),
  ],
  provides_constant: true,
  solutions: ['l: l (h:t:x: FALSE) TRUE'],
}
