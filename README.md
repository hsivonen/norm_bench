This repo contains `criterion` benchmarks that compare the ICU4X normalizer
with `unicode-normalization` and `unic-normal` (with `&str` source and collecting
to `String`) and with ICU4C (via `rust_icu` with UTF-16 source and ICU4C writing
to `rust_icu_ustring::UChar` and ICU4X writing to `Vec<u16>`).

The test data comes from Wikipedia and, therefore, is subject to CC-by-sa. This
repo is separate from the ICU4X repo in order to avoid adding CC-by-sa content
to the ICU4X repo. See `testdata/wikipedia/sources.txt` and `LICENSE-CC-BY-SA`
for details.
