// The search page's script, loaded on that page only. The index is Zola's
// `search_index.en.js`, which sets `window.searchIndex`; the engine is the
// vendored elasticlunr. Results are built with DOM calls and `textContent`, never
// `innerHTML`: a page's text carries `<` and `&` from its code, and here it is
// data, not markup.
(function () {
  "use strict";
  var input = document.getElementById("search-q");
  var status = document.getElementById("search-status");
  var list = document.getElementById("search-results");
  if (!input || !status || !list) return;
  if (!window.elasticlunr || !window.searchIndex) {
    status.textContent = "The search index did not load. The guide's overview lists every page.";
    return;
  }

  var index = window.elasticlunr.Index.load(window.searchIndex);
  var fields = { title: { boost: 3 }, body: { boost: 1 } };
  var SNIPPET = 180;
  var LEAD = 60;

  // Every word is required first; if that finds nothing, any word will do.
  function search(query) {
    var found = index.search(query, { bool: "AND", expand: true, fields: fields });
    return found.length ? found : index.search(query, { bool: "OR", expand: true, fields: fields });
  }

  // What to highlight: each query word cut to its stem, matched at a word start,
  // so "conditions" marks "condition" and "conditional" as the index matched them.
  function pattern(query) {
    var stems = query.toLowerCase().split(/[\s\-]+/).map(function (word) {
      word = word.replace(/^\W+|\W+$/g, "");
      return word.length > 1 ? window.elasticlunr.stemmer(word) : "";
    }).filter(function (stem) { return stem.length > 1; });
    if (!stems.length) return null;
    var escaped = stems.map(function (s) { return s.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"); });
    return new RegExp("\\b(?:" + escaped.join("|") + ")\\w*", "gi");
  }

  // A window of the page's text around its first match, cut at word boundaries.
  function excerpt(body, re) {
    var at = 0;
    if (re) {
      re.lastIndex = 0;
      var first = re.exec(body);
      if (first) at = Math.max(0, first.index - LEAD);
    }
    if (at > 0) {
      var space = body.indexOf(" ", at);
      at = space === -1 ? at : space + 1;
    }
    var end = Math.min(body.length, at + SNIPPET);
    if (end < body.length) {
      var cut = body.lastIndexOf(" ", end);
      end = cut > at ? cut : end;
    }
    return (at > 0 ? "… " : "") + body.slice(at, end).trim() + (end < body.length ? " …" : "");
  }

  function marked(text, re) {
    var span = document.createElement("span");
    var from = 0;
    if (re) {
      re.lastIndex = 0;
      var m;
      while ((m = re.exec(text)) !== null) {
        if (m.index > from) span.appendChild(document.createTextNode(text.slice(from, m.index)));
        var mark = document.createElement("mark");
        mark.textContent = m[0];
        span.appendChild(mark);
        from = m.index + m[0].length;
      }
    }
    if (from < text.length) span.appendChild(document.createTextNode(text.slice(from)));
    return span;
  }

  function render(query) {
    list.textContent = "";
    query = query.trim();
    if (!query) {
      status.textContent = "";
      return;
    }
    var re = pattern(query);
    var shown = 0;
    search(query).forEach(function (hit) {
      var doc = index.documentStore.getDoc(hit.ref);
      if (!doc || !/^https?:\/\//.test(hit.ref)) return;
      var item = document.createElement("li");
      var link = document.createElement("a");
      link.href = hit.ref;
      link.textContent = doc.title || "happenstance";
      item.appendChild(link);
      if (doc.body) item.appendChild(marked(excerpt(doc.body.replace(/\s+/g, " "), re), re));
      list.appendChild(item);
      shown += 1;
    });
    status.textContent = shown === 0
      ? "No page matches “" + query + "”. Fewer or shorter words may find it."
      : shown + (shown === 1 ? " page matches" : " pages match") + " “" + query + "”.";
  }

  // The address carries the query, so a search can be shared and Back returns to it.
  function remember(query) {
    try {
      var url = new URL(window.location.href);
      if (query) url.searchParams.set("q", query); else url.searchParams.delete("q");
      window.history.replaceState(null, "", url);
    } catch (e) { /* an address that cannot be rewritten: the results still show */ }
  }

  var pending;
  input.addEventListener("input", function () {
    clearTimeout(pending);
    pending = setTimeout(function () { render(input.value); remember(input.value.trim()); }, 120);
  });
  input.form.addEventListener("submit", function (event) {
    event.preventDefault();
    clearTimeout(pending);
    render(input.value);
    remember(input.value.trim());
  });

  // Arrow keys walk from the box into the results and back; Tab works as ever.
  function links() { return Array.prototype.slice.call(list.querySelectorAll("a")); }
  input.addEventListener("keydown", function (event) {
    if (event.key !== "ArrowDown") return;
    var all = links();
    if (all.length) { event.preventDefault(); all[0].focus(); }
  });
  list.addEventListener("keydown", function (event) {
    if (event.key !== "ArrowDown" && event.key !== "ArrowUp") return;
    var all = links();
    var at = all.indexOf(document.activeElement);
    if (at === -1) return;
    event.preventDefault();
    var next = at + (event.key === "ArrowDown" ? 1 : -1);
    if (next < 0) input.focus(); else if (next < all.length) all[next].focus();
  });

  var initial = new URLSearchParams(window.location.search).get("q");
  if (initial) {
    input.value = initial;
    render(initial);
  }
})();
