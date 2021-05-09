local bool_lib = import '../boolean/lib.libsonnet';
local lib = import 'lib.libsonnet';
{
  name: 'empty',
  description: |||
    Write function EMPTY, that given a list L, reduces to TRUE if it's empty, and FALSE if it's not.
    
    For example:
    - POP [A] = POP (A, FALSE) -> FALSE
    - POP [] = POP FALSE -> TRUE
  |||,
  // TODO: hint
  test_cases: [
    bool_lib.test_false('f: f %s' % [lib.list(['A'])]),
    bool_lib.test_true('f: f %s' % [lib.list([])]),
    bool_lib.test_true('f: f (POP %s)' % [lib.list(['A'])]),
    bool_lib.test_false('f: f (PUSH A FALSE)'),
  ],
  provides_constant: true,
  solutions: ['l: l (h:t:x: FALSE) TRUE'],
}
