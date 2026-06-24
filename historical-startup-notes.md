
For my zero external Dependencies goal, I am still missing a `unicode_segmentation` equivalent.
I have now spend over a year (started in mid 2024, now its Jan of 2026), on and off, learning about [Unicode](https://www.unicode.org/versions/Unicode17.0.0/core-spec/chapter-3/#G52443), and now I understand why everybody warns about doing this.
But I am way too far into it to stop now. 

There are however concessions / limits I will have to make to keep my sanity:

1. Grapheme Clusters will be determined by using `Grapheme Cluster Boundary Rules`
2. Only `Extended Grapheme Clusters` are supported.
3. The rule `GB9c` and `GB11` will be ignored.
4. Only `UTF-8` will be supported.
5. Segmenting text into words or sentences will not be supported.

This way I only need to parse one [table](https://www.unicode.org/Public/UCD/latest/ucd/auxiliary/GraphemeBreakProperty.txt) and apply the [rules](https://www.unicode.org/reports/tr29/tr29-47.html#Grapheme_Cluster_Break_Property_Values) using it.

TBD (probably number 2 though):
- Include the .txt and parse it always
- Pre parse the table and include that in the binary

