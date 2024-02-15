local bool = import '../boolean/lib.libsonnet';
{
    bst(list)::
        if std.length(list) == 0 then
            'FALSE'
        else
            local x = list[0];
            '(NODE %s %d %s)' % [self.bst(std.filter(function(y) y < x, list)), x, self.bst(std.filter(function(y) y > x, list))],
    test_bst(term, list)::
        bool.test_true(
            # This is awkward because we always need to put the term in the beginning for it to work during levels
            '%s (h:j:k:t: Y (f: a:b: a (l:v:r:t: b (x:y:z:t: (AND (EQ v y) (AND (f l x) (f r z)))) FALSE) (b (l:v:r:t: FALSE) TRUE)) (NODE h j k) (%s)) %s' % [term, self.bst(list), if std.length(list) == 0 then 'TRUE' else 'FALSE']
        )
}