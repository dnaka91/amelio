(function() {var implementors = {};
implementors["bitvec"] = [{"text":"impl&lt;T&gt; FusedIterator for Domain&lt;'_, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;O, T&gt; FusedIterator for Iter&lt;'_, O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;O, T&gt; FusedIterator for IterMut&lt;'_, O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;O, T&gt; FusedIterator for Windows&lt;'_, O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;O, T&gt; FusedIterator for Chunks&lt;'_, O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;O, T&gt; FusedIterator for ChunksMut&lt;'_, O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;O, T&gt; FusedIterator for ChunksExact&lt;'_, O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;O, T&gt; FusedIterator for ChunksExactMut&lt;'_, O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;O, T&gt; FusedIterator for RChunks&lt;'_, O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;O, T&gt; FusedIterator for RChunksMut&lt;'_, O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;O, T&gt; FusedIterator for RChunksExact&lt;'_, O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;O, T&gt; FusedIterator for RChunksExactMut&lt;'_, O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, O, T, P&gt; FusedIterator for Split&lt;'a, O, T, P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: 'a + BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: 'a + BitStore,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: FnMut(usize, &amp;bool) -&gt; bool,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, O, T, P&gt; FusedIterator for SplitMut&lt;'a, O, T, P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: 'a + BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: 'a + BitStore,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: FnMut(usize, &amp;bool) -&gt; bool,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, O, T, P&gt; FusedIterator for RSplit&lt;'a, O, T, P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: 'a + BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: 'a + BitStore,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: FnMut(usize, &amp;bool) -&gt; bool,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, O, T, P&gt; FusedIterator for RSplitMut&lt;'a, O, T, P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: 'a + BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: 'a + BitStore,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: FnMut(usize, &amp;bool) -&gt; bool,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;O, T, P&gt; FusedIterator for SplitN&lt;'_, O, T, P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: FnMut(usize, &amp;bool) -&gt; bool,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;O, T, P&gt; FusedIterator for SplitNMut&lt;'_, O, T, P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: FnMut(usize, &amp;bool) -&gt; bool,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;O, T, P&gt; FusedIterator for RSplitN&lt;'_, O, T, P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: FnMut(usize, &amp;bool) -&gt; bool,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;O, T, P&gt; FusedIterator for RSplitNMut&lt;'_, O, T, P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: FnMut(usize, &amp;bool) -&gt; bool,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;O, T&gt; FusedIterator for IntoIter&lt;O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;O, T&gt; FusedIterator for Drain&lt;'_, O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;O, T, I&gt; FusedIterator for Splice&lt;'_, O, T, I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Iterator&lt;Item = bool&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["generic_array"] = [{"text":"impl&lt;T, N&gt; FusedIterator for GenericArrayIter&lt;T, N&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: ArrayLength&lt;T&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["hashbrown"] = [{"text":"impl&lt;T&gt; FusedIterator for RawIter&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; FusedIterator for RawIntoIter&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; FusedIterator for RawDrain&lt;'_, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;K, V, F&gt; FusedIterator for DrainFilter&lt;'_, K, V, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: FnMut(&amp;K, &amp;mut V) -&gt; bool,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;K, V&gt; FusedIterator for Iter&lt;'_, K, V&gt;","synthetic":false,"types":[]},{"text":"impl&lt;K, V&gt; FusedIterator for IterMut&lt;'_, K, V&gt;","synthetic":false,"types":[]},{"text":"impl&lt;K, V&gt; FusedIterator for IntoIter&lt;K, V&gt;","synthetic":false,"types":[]},{"text":"impl&lt;K, V&gt; FusedIterator for Keys&lt;'_, K, V&gt;","synthetic":false,"types":[]},{"text":"impl&lt;K, V&gt; FusedIterator for Values&lt;'_, K, V&gt;","synthetic":false,"types":[]},{"text":"impl&lt;K, V&gt; FusedIterator for ValuesMut&lt;'_, K, V&gt;","synthetic":false,"types":[]},{"text":"impl&lt;K, V&gt; FusedIterator for Drain&lt;'_, K, V&gt;","synthetic":false,"types":[]},{"text":"impl&lt;K&gt; FusedIterator for Iter&lt;'_, K&gt;","synthetic":false,"types":[]},{"text":"impl&lt;K&gt; FusedIterator for IntoIter&lt;K&gt;","synthetic":false,"types":[]},{"text":"impl&lt;K&gt; FusedIterator for Drain&lt;'_, K&gt;","synthetic":false,"types":[]},{"text":"impl&lt;K, F&gt; FusedIterator for DrainFilter&lt;'_, K, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: FnMut(&amp;K) -&gt; bool,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T, S&gt; FusedIterator for Intersection&lt;'_, T, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Eq + Hash,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T, S&gt; FusedIterator for Difference&lt;'_, T, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Eq + Hash,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T, S&gt; FusedIterator for SymmetricDifference&lt;'_, T, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Eq + Hash,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T, S&gt; FusedIterator for Union&lt;'_, T, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Eq + Hash,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["mime_guess"] = [{"text":"impl FusedIterator for Iter","synthetic":false,"types":[]},{"text":"impl FusedIterator for IterRaw","synthetic":false,"types":[]}];
implementors["nix"] = [{"text":"impl&lt;'a&gt; FusedIterator for Fds&lt;'a&gt;","synthetic":false,"types":[]}];
implementors["rand"] = [{"text":"impl&lt;D, R, T&gt; FusedIterator for DistIter&lt;D, R, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;D: Distribution&lt;T&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Rng,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["serde_json"] = [{"text":"impl&lt;'de, R, T&gt; FusedIterator for StreamDeserializer&lt;'de, R, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Read&lt;'de&gt; + Fused,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Deserialize&lt;'de&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; FusedIterator for Iter&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; FusedIterator for IterMut&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl FusedIterator for IntoIter","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; FusedIterator for Keys&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; FusedIterator for Values&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; FusedIterator for ValuesMut&lt;'a&gt;","synthetic":false,"types":[]}];
implementors["smallvec"] = [{"text":"impl&lt;'a, T:&nbsp;Array&gt; FusedIterator for Drain&lt;'a, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;A:&nbsp;Array&gt; FusedIterator for IntoIter&lt;A&gt;","synthetic":false,"types":[]}];
implementors["tinyvec"] = [{"text":"impl&lt;'p, A, I&gt; FusedIterator for ArrayVecSplice&lt;'p, A, I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Array,<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Iterator&lt;Item = A::Item&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;A:&nbsp;Array&gt; FusedIterator for ArrayVecIterator&lt;A&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T:&nbsp;'a + Default&gt; FusedIterator for ArrayVecDrain&lt;'a, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p, 's, T:&nbsp;Default&gt; FusedIterator for SliceVecDrain&lt;'p, 's, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p, A, I&gt; FusedIterator for TinyVecSplice&lt;'p, A, I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Array,<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Iterator&lt;Item = A::Item&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;A:&nbsp;Array&gt; FusedIterator for TinyVecIterator&lt;A&gt;","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()