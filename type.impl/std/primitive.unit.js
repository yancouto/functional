(function() {var type_impls = {
"functional":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-FromParallelIterator%3C()%3E-for-()\" class=\"impl\"><a class=\"src rightside\" href=\"src/rayon/iter/from_par_iter.rs.html#272\">source</a><a href=\"#impl-FromParallelIterator%3C()%3E-for-()\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"functional/prelude/trait.FromParallelIterator.html\" title=\"trait functional::prelude::FromParallelIterator\">FromParallelIterator</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a></h3></section></summary><div class=\"docblock\"><p>Collapses all unit items from a parallel iterator into one.</p>\n<p>This is more useful when combined with higher-level abstractions, like\ncollecting to a <code>Result&lt;(), E&gt;</code> where you only care about errors:</p>\n\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">use </span>std::io::<span class=\"kw-2\">*</span>;\n<span class=\"kw\">use </span>rayon::prelude::<span class=\"kw-2\">*</span>;\n\n<span class=\"kw\">let </span>data = <span class=\"macro\">vec!</span>[<span class=\"number\">1</span>, <span class=\"number\">2</span>, <span class=\"number\">3</span>, <span class=\"number\">4</span>, <span class=\"number\">5</span>];\n<span class=\"kw\">let </span>res: <span class=\"prelude-ty\">Result</span>&lt;()&gt; = data.par_iter()\n    .map(|x| <span class=\"macro\">writeln!</span>(stdout(), <span class=\"string\">\"{}\"</span>, x))\n    .collect();\n<span class=\"macro\">assert!</span>(res.is_ok());</code></pre></div>\n</div><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_par_iter\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/rayon/iter/from_par_iter.rs.html#273-275\">source</a><a href=\"#method.from_par_iter\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"functional/prelude/trait.FromParallelIterator.html#tymethod.from_par_iter\" class=\"fn\">from_par_iter</a>&lt;I&gt;(par_iter: I)<div class=\"where\">where\n    I: <a class=\"trait\" href=\"functional/prelude/trait.IntoParallelIterator.html\" title=\"trait functional::prelude::IntoParallelIterator\">IntoParallelIterator</a>&lt;Item = <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>&gt;,</div></h4></section></summary><div class='docblock'>Creates an instance of the collection from the parallel iterator <code>par_iter</code>. <a href=\"functional/prelude/trait.FromParallelIterator.html#tymethod.from_par_iter\">Read more</a></div></details></div></details>","FromParallelIterator<()>","functional::drawables::friend_leaderboard::FriendResult","functional::gamestates::base::SteamClient","functional::MainThreadSteamClient"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-ParallelExtend%3C()%3E-for-()\" class=\"impl\"><a class=\"src rightside\" href=\"src/rayon/iter/extend.rs.html#607\">source</a><a href=\"#impl-ParallelExtend%3C()%3E-for-()\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"functional/prelude/trait.ParallelExtend.html\" title=\"trait functional::prelude::ParallelExtend\">ParallelExtend</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a></h3></section></summary><div class=\"docblock\"><p>Collapses all unit items from a parallel iterator into one.</p>\n</div><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.par_extend\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/rayon/iter/extend.rs.html#608-610\">source</a><a href=\"#method.par_extend\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"functional/prelude/trait.ParallelExtend.html#tymethod.par_extend\" class=\"fn\">par_extend</a>&lt;I&gt;(&amp;mut self, par_iter: I)<div class=\"where\">where\n    I: <a class=\"trait\" href=\"functional/prelude/trait.IntoParallelIterator.html\" title=\"trait functional::prelude::IntoParallelIterator\">IntoParallelIterator</a>&lt;Item = <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>&gt;,</div></h4></section></summary><div class='docblock'>Extends an instance of the collection with the elements drawn\nfrom the parallel iterator <code>par_iter</code>. <a href=\"functional/prelude/trait.ParallelExtend.html#tymethod.par_extend\">Read more</a></div></details></div></details>","ParallelExtend<()>","functional::drawables::friend_leaderboard::FriendResult","functional::gamestates::base::SteamClient","functional::MainThreadSteamClient"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()