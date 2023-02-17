# Change Log

All notable changes to the "gruber fruity" theme pack will be documented here.

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
- Gruber Apple Light 🍎: reduced accent color contrast with background. New accent: `#ff264e`
- Gruber Blueberry Dark 🫐: reduced accent color contrast with background. New accent: `#7171e3`

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