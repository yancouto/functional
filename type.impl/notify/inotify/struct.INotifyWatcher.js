(function() {var type_impls = {
"notify":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Drop-for-INotifyWatcher\" class=\"impl\"><a class=\"src rightside\" href=\"src/notify/inotify.rs.html#494-499\">source</a><a href=\"#impl-Drop-for-INotifyWatcher\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"notify/inotify/struct.INotifyWatcher.html\" title=\"struct notify::inotify::INotifyWatcher\">INotifyWatcher</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.drop\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/notify/inotify.rs.html#495-498\">source</a><a href=\"#method.drop\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop\" class=\"fn\">drop</a>(&amp;mut self)</h4></section></summary><div class='docblock'>Executes the destructor for this type. <a href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop\">Read more</a></div></details></div></details>","Drop","notify::RecommendedWatcher"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Watcher-for-INotifyWatcher\" class=\"impl\"><a class=\"src rightside\" href=\"src/notify/inotify.rs.html#441-492\">source</a><a href=\"#impl-Watcher-for-INotifyWatcher\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"notify/trait.Watcher.html\" title=\"trait notify::Watcher\">Watcher</a> for <a class=\"struct\" href=\"notify/inotify/struct.INotifyWatcher.html\" title=\"struct notify::inotify::INotifyWatcher\">INotifyWatcher</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.new_raw\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/notify/inotify.rs.html#442-449\">source</a><a href=\"#method.new_raw\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"notify/trait.Watcher.html#tymethod.new_raw\" class=\"fn\">new_raw</a>(tx: <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/sync/mpsc/struct.Sender.html\" title=\"struct std::sync::mpsc::Sender\">Sender</a>&lt;<a class=\"struct\" href=\"notify/struct.RawEvent.html\" title=\"struct notify::RawEvent\">RawEvent</a>&gt;) -&gt; <a class=\"type\" href=\"notify/type.Result.html\" title=\"type notify::Result\">Result</a>&lt;<a class=\"struct\" href=\"notify/inotify/struct.INotifyWatcher.html\" title=\"struct notify::inotify::INotifyWatcher\">INotifyWatcher</a>&gt;</h4></section></summary><div class='docblock'>Create a new watcher in <em>raw</em> mode. <a href=\"notify/trait.Watcher.html#tymethod.new_raw\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.new\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/notify/inotify.rs.html#451-461\">source</a><a href=\"#method.new\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"notify/trait.Watcher.html#tymethod.new\" class=\"fn\">new</a>(tx: <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/sync/mpsc/struct.Sender.html\" title=\"struct std::sync::mpsc::Sender\">Sender</a>&lt;<a class=\"enum\" href=\"notify/enum.DebouncedEvent.html\" title=\"enum notify::DebouncedEvent\">DebouncedEvent</a>&gt;, delay: <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/time/struct.Duration.html\" title=\"struct core::time::Duration\">Duration</a>) -&gt; <a class=\"type\" href=\"notify/type.Result.html\" title=\"type notify::Result\">Result</a>&lt;<a class=\"struct\" href=\"notify/inotify/struct.INotifyWatcher.html\" title=\"struct notify::inotify::INotifyWatcher\">INotifyWatcher</a>&gt;</h4></section></summary><div class='docblock'>Create a new <em>debounced</em> watcher with a <code>delay</code>. <a href=\"notify/trait.Watcher.html#tymethod.new\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.watch\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/notify/inotify.rs.html#463-476\">source</a><a href=\"#method.watch\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"notify/trait.Watcher.html#tymethod.watch\" class=\"fn\">watch</a>&lt;P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/path/struct.Path.html\" title=\"struct std::path::Path\">Path</a>&gt;&gt;(\n    &amp;mut self,\n    path: P,\n    recursive_mode: <a class=\"enum\" href=\"notify/enum.RecursiveMode.html\" title=\"enum notify::RecursiveMode\">RecursiveMode</a>\n) -&gt; <a class=\"type\" href=\"notify/type.Result.html\" title=\"type notify::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>&gt;</h4></section></summary><div class='docblock'>Begin watching a new path. <a href=\"notify/trait.Watcher.html#tymethod.watch\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.unwatch\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/notify/inotify.rs.html#478-491\">source</a><a href=\"#method.unwatch\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"notify/trait.Watcher.html#tymethod.unwatch\" class=\"fn\">unwatch</a>&lt;P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/path/struct.Path.html\" title=\"struct std::path::Path\">Path</a>&gt;&gt;(&amp;mut self, path: P) -&gt; <a class=\"type\" href=\"notify/type.Result.html\" title=\"type notify::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>&gt;</h4></section></summary><div class='docblock'>Stop watching a path. <a href=\"notify/trait.Watcher.html#tymethod.unwatch\">Read more</a></div></details></div></details>","Watcher","notify::RecommendedWatcher"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()