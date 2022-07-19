<!-- markdownlint-disable no-inline-html -->
<!-- markdownlint-disable no-space-in-emphasis -->
<!-- cSpell:disable -->

# Identifiers

<div class="admonition summary">
<p class="admonition-title">Grammar</p>

<pre style="white-space: pre-wrap;"><code id="IdentifierPathProduction"><span style="color: var(--md-code-hl-keyword-color);">IdentifierPath</span><span style="color: var(--md-code-hl-operator-color);"> = </span><span style="color: var(--md-code-hl-constant-color);">1</span><span style="color: var(--md-code-hl-operator-color);">…</span><span style="color: var(--md-code-hl-operator-color);">{</span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../05-expressions/07-identifiers#IdentifierProduction">«Identifier»</a></span><span style="color: var(--md-code-hl-operator-color);"> / </span><span style="color: var(--md-code-hl-string-color);">'.'</span><span style="color: var(--md-code-hl-operator-color);">}</span><span style="color: var(--md-code-hl-operator-color);">;</span><br/></code></pre>

<pre style="white-space: pre-wrap;"><code id="IdentifierProduction"><span style="color: var(--md-code-hl-keyword-color);">«Identifier»</span><span style="color: var(--md-code-hl-operator-color);"> = </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../05-expressions/07-identifiers#RawIdentifierProduction">«RawIdentifier»</a></span><span style="color: var(--md-code-hl-operator-color);"> - </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../05-expressions/07-identifiers#KeywordProduction">«Keyword»</a></span><span style="color: var(--md-code-hl-operator-color);">;</span><br/></code></pre>

<pre style="white-space: pre-wrap;"><code id="KeywordProduction"><span style="color: var(--md-code-hl-keyword-color);">«Keyword»</span><span style="color: var(--md-code-hl-operator-color);"> = </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../05-expressions/04-primary-expressions#BooleanLiteralProduction">«BooleanLiteral»</a></span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../03-types/04-elementary-types#FixedBytesTypeProduction">«FixedBytesType»</a></span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../05-expressions/05-numbers#NumberUnitProduction">«NumberUnit»</a></span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../05-expressions/07-identifiers#ReservedKeywordProduction">«ReservedKeyword»</a></span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../03-types/04-elementary-types#SignedIntegerTypeProduction">«SignedIntegerType»</a></span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../03-types/04-elementary-types#UnsignedIntegerTypeProduction">«UnsignedIntegerType»</a></span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"abstract"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"address"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"anonymous"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"as"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"assembly"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"bool"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"break"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"calldata"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"catch"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"constant"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"constructor"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"continue"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"contract"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"delete"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"do"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"else"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"emit"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"enum"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"event"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"external"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"fallback"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"false"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"fixed"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"for"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"function"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"hex"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"if"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"immutable"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"import"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"indexed"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"interface"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"internal"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"is"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"library"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"mapping"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"memory"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"modifier"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"new"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"override"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"payable"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"pragma"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"private"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"public"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"pure"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"receive"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"return"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"returns"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"storage"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"string"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"struct"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"true"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"try"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"type"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"ufixed"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"unchecked"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"using"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"view"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"virtual"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"while"</span><span style="color: var(--md-code-hl-operator-color);">;</span><br/></code></pre>

<pre style="white-space: pre-wrap;"><code id="ReservedKeywordProduction"><span style="color: var(--md-code-hl-keyword-color);">«ReservedKeyword»</span><span style="color: var(--md-code-hl-operator-color);"> = </span><span style="color: var(--md-code-hl-string-color);">"after"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"alias"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"apply"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"auto"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"byte"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"case"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"copyof"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"default"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"define"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"final"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"implements"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"in"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"inline"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"let"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"macro"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"match"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"mutable"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"null"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"of"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"partial"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"promise"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"reference"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"relocatable"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"sealed"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"sizeof"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"static"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"supports"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"switch"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"typedef"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"typeof"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"var"</span><span style="color: var(--md-code-hl-operator-color);">;</span><br/></code></pre>

<pre style="white-space: pre-wrap;"><code id="RawIdentifierProduction"><span style="color: var(--md-code-hl-keyword-color);">«RawIdentifier»</span><span style="color: var(--md-code-hl-operator-color);"> = </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../05-expressions/07-identifiers#IdentifierStartProduction">«IdentifierStart»</a></span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-operator-color);">{</span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../05-expressions/07-identifiers#IdentifierPartProduction">«IdentifierPart»</a></span><span style="color: var(--md-code-hl-operator-color);">}</span><span style="color: var(--md-code-hl-operator-color);">;</span><br/></code></pre>

<pre style="white-space: pre-wrap;"><code id="IdentifierStartProduction"><span style="color: var(--md-code-hl-keyword-color);">«IdentifierStart»</span><span style="color: var(--md-code-hl-operator-color);"> = </span><span style="color: var(--md-code-hl-string-color);">'_'</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">'$'</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">'a'</span><span style="color: var(--md-code-hl-operator-color);">…</span><span style="color: var(--md-code-hl-string-color);">'z'</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">'A'</span><span style="color: var(--md-code-hl-operator-color);">…</span><span style="color: var(--md-code-hl-string-color);">'Z'</span><span style="color: var(--md-code-hl-operator-color);">;</span><br/></code></pre>

<pre style="white-space: pre-wrap;"><code id="IdentifierPartProduction"><span style="color: var(--md-code-hl-keyword-color);">«IdentifierPart»</span><span style="color: var(--md-code-hl-operator-color);"> = </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../05-expressions/07-identifiers#IdentifierStartProduction">«IdentifierStart»</a></span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">'0'</span><span style="color: var(--md-code-hl-operator-color);">…</span><span style="color: var(--md-code-hl-string-color);">'9'</span><span style="color: var(--md-code-hl-operator-color);">;</span><br/></code></pre>

</div>

--8<-- "specification/notes/05-expressions/07-identifiers/index.md"