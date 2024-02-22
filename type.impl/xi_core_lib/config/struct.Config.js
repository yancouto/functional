(function() {var type_impls = {
"xi_core_lib":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-Config%3CT%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/xi_core_lib/config.rs.html#148\">source</a><a href=\"#impl-Clone-for-Config%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"xi_core_lib/config/struct.Config.html\" title=\"struct xi_core_lib::config::Config\">Config</a>&lt;T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xi_core_lib/config.rs.html#148\">source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"struct\" href=\"xi_core_lib/config/struct.Config.html\" title=\"struct xi_core_lib::config::Config\">Config</a>&lt;T&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/clone.rs.html#169\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","xi_core_lib::config::BufferConfig"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Config%3CT%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/xi_core_lib/config.rs.html#623-627\">source</a><a href=\"#impl-Config%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"struct\" href=\"xi_core_lib/config/struct.Config.html\" title=\"struct xi_core_lib::config::Config\">Config</a>&lt;T&gt;</h3></section></summary><div class=\"impl-items\"><section id=\"method.to_table\" class=\"method\"><a class=\"src rightside\" href=\"src/xi_core_lib/config.rs.html#624-626\">source</a><h4 class=\"code-header\">pub fn <a href=\"xi_core_lib/config/struct.Config.html#tymethod.to_table\" class=\"fn\">to_table</a>(&amp;self) -&gt; <a class=\"type\" href=\"xi_core_lib/config/type.Table.html\" title=\"type xi_core_lib::config::Table\">Table</a></h4></section></div></details>",0,"xi_core_lib::config::BufferConfig"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Config%3CT%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/xi_core_lib/config.rs.html#629-638\">source</a><a href=\"#impl-Config%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'de, T: <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt;&gt; <a class=\"struct\" href=\"xi_core_lib/config/struct.Config.html\" title=\"struct xi_core_lib::config::Config\">Config</a>&lt;T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.changes_from\" class=\"method\"><a class=\"src rightside\" href=\"src/xi_core_lib/config.rs.html#632-637\">source</a><h4 class=\"code-header\">pub fn <a href=\"xi_core_lib/config/struct.Config.html#tymethod.changes_from\" class=\"fn\">changes_from</a>(&amp;self, other: <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;&amp;<a class=\"struct\" href=\"xi_core_lib/config/struct.Config.html\" title=\"struct xi_core_lib::config::Config\">Config</a>&lt;T&gt;&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"type\" href=\"xi_core_lib/config/type.Table.html\" title=\"type xi_core_lib::config::Table\">Table</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Returns a <code>Table</code> of all the items in <code>self</code> which have different\nvalues than in <code>other</code>.</p>\n</div></details></div></details>",0,"xi_core_lib::config::BufferConfig"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-Config%3CT%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/xi_core_lib/config.rs.html#148\">source</a><a href=\"#impl-Debug-for-Config%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"xi_core_lib/config/struct.Config.html\" title=\"struct xi_core_lib::config::Config\">Config</a>&lt;T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xi_core_lib/config.rs.html#148\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/nightly/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","xi_core_lib::config::BufferConfig"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Deserialize%3C'de%3E-for-Config%3CT%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/xi_core_lib/config.rs.html#148\">source</a><a href=\"#impl-Deserialize%3C'de%3E-for-Config%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'de, T&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"xi_core_lib/config/struct.Config.html\" title=\"struct xi_core_lib::config::Config\">Config</a>&lt;T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.deserialize\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xi_core_lib/config.rs.html#148\">source</a><a href=\"#method.deserialize\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"serde/de/trait.Deserialize.html#tymethod.deserialize\" class=\"fn\">deserialize</a>&lt;__D&gt;(__deserializer: __D) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;Self, __D::<a class=\"associatedtype\" href=\"serde/de/trait.Deserializer.html#associatedtype.Error\" title=\"type serde::de::Deserializer::Error\">Error</a>&gt;<div class=\"where\">where\n    __D: <a class=\"trait\" href=\"serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt;,</div></h4></section></summary><div class='docblock'>Deserialize this value from the given Serde deserializer. <a href=\"serde/de/trait.Deserialize.html#tymethod.deserialize\">Read more</a></div></details></div></details>","Deserialize<'de>","xi_core_lib::config::BufferConfig"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PartialEq-for-Config%3CT%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/xi_core_lib/config.rs.html#677-681\">source</a><a href=\"#impl-PartialEq-for-Config%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> for <a class=\"struct\" href=\"xi_core_lib/config/struct.Config.html\" title=\"struct xi_core_lib::config::Config\">Config</a>&lt;T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.eq\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xi_core_lib/config.rs.html#678-680\">source</a><a href=\"#method.eq\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq\" class=\"fn\">eq</a>(&amp;self, other: &amp;<a class=\"struct\" href=\"xi_core_lib/config/struct.Config.html\" title=\"struct xi_core_lib::config::Config\">Config</a>&lt;T&gt;) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>This method tests for <code>self</code> and <code>other</code> values to be equal, and is used\nby <code>==</code>.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.ne\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263\">source</a></span><a href=\"#method.ne\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne\" class=\"fn\">ne</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>This method tests for <code>!=</code>. The default implementation is almost always\nsufficient, and should not be overridden without very good reason.</div></details></div></details>","PartialEq","xi_core_lib::config::BufferConfig"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Serialize-for-Config%3CT%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/xi_core_lib/config.rs.html#148\">source</a><a href=\"#impl-Serialize-for-Config%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"xi_core_lib/config/struct.Config.html\" title=\"struct xi_core_lib::config::Config\">Config</a>&lt;T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.serialize\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xi_core_lib/config.rs.html#148\">source</a><a href=\"#method.serialize\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"serde/ser/trait.Serialize.html#tymethod.serialize\" class=\"fn\">serialize</a>&lt;__S&gt;(&amp;self, __serializer: __S) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;__S::<a class=\"associatedtype\" href=\"serde/ser/trait.Serializer.html#associatedtype.Ok\" title=\"type serde::ser::Serializer::Ok\">Ok</a>, __S::<a class=\"associatedtype\" href=\"serde/ser/trait.Serializer.html#associatedtype.Error\" title=\"type serde::ser::Serializer::Error\">Error</a>&gt;<div class=\"where\">where\n    __S: <a class=\"trait\" href=\"serde/ser/trait.Serializer.html\" title=\"trait serde::ser::Serializer\">Serializer</a>,</div></h4></section></summary><div class='docblock'>Serialize this value into the given Serde serializer. <a href=\"serde/ser/trait.Serialize.html#tymethod.serialize\">Read more</a></div></details></div></details>","Serialize","xi_core_lib::config::BufferConfig"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()