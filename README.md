# CSS RTL Compiler
Convert CSS files into an RTL-compatible version.

This library aims to produce CSS that is bi-directional, but that doesn't change how the CSS behaves (doesn't affect specificity of selectors).

The idea is simple: move every CSS property declaration that is direction dependent, such as `right`, `left`, `text-align`, etc, into a direction specific selector, and create two variants of each.

This is done without affecting the specificity by making use of [`:where` pseudo-class](https://developer.mozilla.org/en-US/docs/Web/CSS/:where), which is [widely supported](https://caniuse.com/?search=%3Awhere).

E.g.

Input:
```css
.selector {
  margin-left: 10px;
}
```

Output:
```css
.selector:where([dir=ltr], [dir=ltr] *) {
  margin-left: 10px;
}
.selector:where([dir=rtl], [dir=rtl] *) {
  margin-right: 10px;
}
```

The output CSS will work as long as:
- All CSS for a given page is processed through this tool
- The page (or root element in a shadow DOM) where the CSS is used has `dir` attribute set.
  - Note: `:dir` css pseduo-class [doesn't have wide-enough support](https://caniuse.com/css-dir-pseudo) to be used, so we opted for `[dir]` instead.
