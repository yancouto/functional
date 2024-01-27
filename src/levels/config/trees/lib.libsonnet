{
    bst(list)::
        if std.length(list) == 0 then
            'FALSE'
        else
            local x = list[0];
            '(NODE %s %d %s)' % [self.bst(std.filter(function(y) y < x, list)), x, self.bst(std.filter(function(y) y > x, list))],
}