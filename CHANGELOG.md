# Change Log

All notable changes to the "gruber fruity" theme pack will be documented here.

## v0.7.1
Language support:
- C support improvements (VSCode)
    - The `->` operator is now highlighted correctly
    - Square brackets are no longer highlighted as keywords
    - Pre-processor directives are now highlighted as macros
- C++ support improvements (VSCode)
    - The `->` operator is now highlighted correctly
    - Pre-processor directives are now highlighted as macros

Editor:
- Remote Icon now has the sky blue accent colour rather than the default
- Warning and Error items in the status bar now have nice colours

## v0.7.0
Language support:
- added Zig support
    - all Zig keywords are now properly highlighted
    - all built-in Zig types are highlighted as types rather than keywords
    - Zig built-in functions are highlighted with italic sky blue
- updated Python highlighting
    - punctuation is now highlighted as with other languages
    - decorators are now highlighted as metaprogramming items (as in Java)

### New theme philosophy
The theme changes in this update are quite significant and include the removal
of some previously existing themes. This signals a change in design philosophy
for the theme pack: 
- Dark themes are now supposed to have stronger, brighter accent colours on a
gray/black background
- Light themes are now supposed to have more muted, darker accent colours on a
light-gray/white background
- Light and Dark themes come in pairs
    - Apple Dark 🍎 & 🍒 Cherry Light
    - Grape Dark 🍇 & 🫐 Blueberry Light
    - Sorbet Dark 🍧 & 🌸 Sakura Light
    - Pear Dark 🍐 & 🫒 Olive Light
    - Peach Dark 🍑 & 🍊 Orange Light

There are already some exceptions to this rule:
- Sakura Light 🌸 and Orange Light 🍊 are light themes with bright accents
- Sorbet Dark 🍧 and Peach Dark 🍑 are dark themes with muted accents
- Lemon Dark 🍋 (the original theme) does not have a light counterpart

With that out of the way, this is the concrete list of changes:

Theme changes:
- Gruber Blueberry Dark 🫐: this theme has been removed. The contrast between
the dark purple/blue of the theme and the black background is too low to be
readable, while still somehow being too bright to be comfortable.
- Gruber Apple Light 🍎: this theme has been removed. The red colour just did
not work well in a light theme, even after the last tweak.

Three new themes:
- Gruber Grape Dark 🍇 (accent: `#bd4dff`): replacement theme for Gruber
Blueberry Dark 🫐. Bright purple colour works better with the dark background.
- Gruber Blueberry Light 🫐 (accent: `#4400b3`). A darker version of Gruber
Blueberry Dark 🫐's fruity accent works well on a light background. The accent
is also bluer this time around.
- Gruber Apple Dark 🍎 (accent: `#ff264e`): Gruber Apple Dark 🍎 (exact same
fruity colour) but as a dark theme.

## v0.6.0
Language support:
- added Go support
    - all Go keywords are now properly highlighted
    - all built-in Go types are highlighted as types rather than keywords

## v0.5.0
Language support:
- added HTML support
- added CSS support
    - all built-in CSS functions are highlighted as macros
    - all built-in CSS constants and W3C standard colour names are highlighted as constants
    - CSS class names and ID's are highlighted with the bold fruity accent colour

## v0.4.0
Two new pink themes:
- Gruber Sakura Light 🌸 (accent: `#dd68b4`)
- Gruber Sorbet Dark 🍧 (accent: `#ffa6c9`)

## v0.3.1
Language support:
- Lua now has proper syntax highlighting
- C/C++ built-in types are no longer marked as keywords

## v0.3.0
Language support:
- Julia now has enhanced highlighting:
    - Julia type annotations/hints are now highlighted as such
    - Julia macros are highlighted properly

Theme changes:
- Gruber Pear Dark 🍐: string color (deadwood) is now a lighter shade of purple (`#938bb7`)
- Gruber Apple Light 🍎: reduced accent color contrast with background. New accent: (`#ff264e`)
- Gruber Blueberry Dark 🫐: reduced accent color contrast with background. New accent: (`#7171e3`)

## v0.2.2
Bugfixes/ general theme changes:
- statusbar now no longer changes color when working on a file without opening a
directory
- remote connection indicator in status bar is now coloured with the secondary
color of the color scheme ("leaf green")

Theme changes:
- Gruber Pear Dark 🍐: new accent color  `#80cf30`, is much less jarring than the
old `#deff0a` one. Also changed the string color to match the olive theme's string
color (which is a dark purple) for better readability

## v0.2.1
Added support for coloured bracket pairs: matching pairs are now coloured in
the standard foreground color and the comment colour, sticking with the minimal
theme.

Language support:
- added TOML support

## v0.2.0
Four new themes:
- Gruber Peach Dark 🍑 (accent: `#fc8d6e`)
- Gruber Olive Light 🫒 (accent: `#6d8c12`)
- Gruber Pear Dark 🍐 (accent: `#deff0a`)
- Gruber Apple Light 🍎 (accent: `#ff0a0a`)

Language support:
- XML tags are now properly rendered
- greatly improved java support (see java support)
    - fixed classes not being underlined when used in type declarations
    - fixed inherited classes not being coloured as classes
    - fixed imports being coloured as keywords
    - fixed package declarations being coloured as keywords
    - fixed "instanceof" keyword not being rendered as keyword
    - fixed java generic storage types being rendered as keywords
    - fixed object arrays rendering as keywords

## v0.1.2
General:
- fixed extension icon not showing
- added keywords to pack for better searchability 

## v0.1.1
General:
- changed logo to now actually be 256x256px
- removed unrendered HTML from README.md
- updated screenshots

Language support:
- JSON identifiers are now given the fruity accent color rather than the leaf green (string) accent color

## v0.1
- Initial release