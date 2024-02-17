local bool = import '../boolean/lib.libsonnet';
local num = import '../numerals/lib.libsonnet';
{
  bst(list)::
    if std.length(list) == 0 then
      'FALSE'
    else
      local x = list[0];
      '(NODE %s %d %s)' % [self.bst(std.filter(function(y) y < x, list)), x, self.bst(std.filter(function(y) y > x, list))],
  test_bst_shape(term, list)::
    bool.test_true(
      // This is awkward because we always need to put the term in the beginning for it to work during levels
      '%s (h:j:k:t: Y (f: a:b: a (l:v:r:t: b (x:y:z:t: (AND (EQ v y) (AND (f l x) (f r z)))) FALSE) (b (l:v:r:t: FALSE) TRUE)) (NODE h j k) (%s)) %s' % [term, self.bst(list), if std.length(list) == 0 then 'TRUE' else 'FALSE']
    ),
  // post called
  inner_test_bst_shape_func(list):::
    local rest =
      if std.length(list) == 1 then
        'l FALSE (NOT r)'
      else
        local l = [x for x in list[1:] if x < list[0]];
        local r = [x for x in list[1:] if x > list[0]];
        'AND (%s) (%s)' % [
          if std.length(l) == 0 then 'BEMPTY l' else 'l %s' % [self.inner_test_bst_shape_func(l)],
          if std.length(r) == 0 then 'BEMPTY r' else 'r %s' % [self.inner_test_bst_shape_func(r)],
        ];
    '(l:v:r: AND (%s v) (%s))' % [num.num_is_eq_func(list[0]), rest],
  test_bst_shape_func(list)::
    if std.length(list) == 0 then
      'BEMPTY'
    else
      '(b: b ' + self.inner_test_bst_shape_func(list) + ')',
  bst_to_preorder()::
    '(b: (Y (f: b:p: b (l:v:r:p: f l (PUSH v (f r p))) p)) b FALSE)',
  bst_to_inorder()::
    '(b: (Y (f: b:p: b (l:v:r:p: PUSH v (f l (f r p))) p)) b FALSE)',
}
