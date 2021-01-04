(function() {var implementors = {};
implementors["base64"] = [{"text":"impl&lt;'a, R:&nbsp;Read&gt; Read for DecoderReader&lt;'a, R&gt;","synthetic":false,"types":[]}];
implementors["bitvec"] = [{"text":"impl&lt;'a, O, T&gt; Read for &amp;'a BitSlice&lt;O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,<br>&nbsp;&nbsp;&nbsp;&nbsp;BitSlice&lt;O, T&gt;: BitField,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["bufstream"] = [{"text":"impl&lt;S:&nbsp;Read + Write&gt; Read for BufStream&lt;S&gt;","synthetic":false,"types":[]}];
implementors["hyper"] = [{"text":"impl&lt;S:&nbsp;NetworkStream&gt; Read for PooledStream&lt;S&gt;","synthetic":false,"types":[]},{"text":"impl Read for Response","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Read for Body&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl Read for Http11Message","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;Read&gt; Read for HttpReader&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl Read for HttpStream","synthetic":false,"types":[]},{"text":"impl&lt;S:&nbsp;NetworkStream&gt; Read for HttpsStream&lt;S&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Read for Request&lt;'a, 'b&gt;","synthetic":false,"types":[]}];
implementors["lettre"] = [{"text":"impl Read for MockStream","synthetic":false,"types":[]},{"text":"impl Read for NetworkStream","synthetic":false,"types":[]},{"text":"impl Read for Message","synthetic":false,"types":[]}];
implementors["native_tls"] = [{"text":"impl&lt;S:&nbsp;Read + Write&gt; Read for TlsStream&lt;S&gt;","synthetic":false,"types":[]}];
implementors["nix"] = [{"text":"impl Read for PtyMaster","synthetic":false,"types":[]}];
implementors["openssl"] = [{"text":"impl&lt;S:&nbsp;Read + Write&gt; Read for SslStream&lt;S&gt;","synthetic":false,"types":[]}];
implementors["rand_core"] = [{"text":"impl Read for dyn RngCore","synthetic":false,"types":[]}];
implementors["rocket"] = [{"text":"impl Read for NamedFile","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Read for &amp;'a NamedFile","synthetic":false,"types":[]},{"text":"impl Read for DataStream","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()