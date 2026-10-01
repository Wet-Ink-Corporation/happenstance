// The site's only script: the theme toggle, copy buttons, the mobile drawer and
// the on-this-page indicator. Everything degrades to a working page without it.
(function () {
  "use strict";
  var root = document.documentElement;

  function currentTheme() {
    if (root.dataset.theme === "light" || root.dataset.theme === "dark") return root.dataset.theme;
    return window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
  }
  document.querySelectorAll(".theme-toggle").forEach(function (button) {
    button.addEventListener("click", function () {
      var next = currentTheme() === "dark" ? "light" : "dark";
      root.dataset.theme = next;
      try { localStorage.setItem("hs-theme", next); } catch (e) { /* storage blocked: the choice lasts this page view */ }
    });
  });

  function flash(button) {
    button.classList.add("is-copied");
    button.setAttribute("aria-label", "Copied");
    setTimeout(function () { button.classList.remove("is-copied"); button.setAttribute("aria-label", "Copy"); }, 1600);
  }
  function copy(text, button) {
    if (!navigator.clipboard) return;
    navigator.clipboard.writeText(text).then(function () { flash(button); }, function () { /* denied: nothing to show */ });
  }
  document.querySelectorAll("[data-copy]").forEach(function (button) {
    button.addEventListener("click", function () { copy(button.getAttribute("data-copy"), button); });
  });
  document.querySelectorAll("[data-copy-from]").forEach(function (button) {
    button.addEventListener("click", function () {
      var source = document.getElementById(button.getAttribute("data-copy-from"));
      if (source) copy(source.innerText, button);
    });
  });

  var menu = document.querySelector(".hs-header__menu");
  var drawer = document.getElementById("drawer");
  if (menu && drawer) {
    menu.addEventListener("click", function () {
      var open = menu.getAttribute("aria-expanded") === "true";
      menu.setAttribute("aria-expanded", String(!open));
      menu.setAttribute("aria-label", open ? "Open the menu" : "Close the menu");
      drawer.hidden = open;
    });
  }

  var links = Array.prototype.slice.call(document.querySelectorAll(".hs-toc__link"));
  if (links.length && "IntersectionObserver" in window) {
    var byId = {};
    links.forEach(function (a) { byId[decodeURIComponent(a.hash.slice(1))] = a; });
    var observer = new IntersectionObserver(function (entries) {
      entries.forEach(function (entry) {
        if (!entry.isIntersecting) return;
        links.forEach(function (a) { a.removeAttribute("aria-current"); });
        var link = byId[entry.target.id];
        if (link) link.setAttribute("aria-current", "true");
      });
    }, { rootMargin: "-80px 0px -70% 0px" });
    Object.keys(byId).forEach(function (id) {
      var heading = document.getElementById(id);
      if (heading) observer.observe(heading);
    });
  }
})();
