- [ ] Add emoji support ::
    * Easiest way would be to apply these rules: https://www.unicode.org/reports/tr51/#EBNF_and_Regex
    * Check against https://unicode.org/emoji/charts/full-emoji-list.html
    * Probably better to construct my own from: https://www.unicode.org/reports/tr51/#emoji_data
- [ ] Add support for `GB9c` :: Do not break within certain combinations with Indic_Conjunct_Break (InCB)=Linker.
- [ ] Rework parser :: From the current double pass to a single pass.
- [ ] Rework return value :: To a vec of small strings dep=software_ideas:"Small String"
