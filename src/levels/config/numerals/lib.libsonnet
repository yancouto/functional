local bool_lib = import '../boolean/lib.libsonnet';
{
  test_num(term, x)::
    bool_lib.test_true('%s%s TRUE' % [term, std.repeat(' FALSE', x)]),
}
