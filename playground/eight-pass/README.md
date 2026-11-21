# Eight fixed passes at one billion keys

Ordinary no-sentinel linear insertion on distinct shuffled ranks. The table was fixed before sampling. From the playground directory use count.exe 1000000000 SEED eight-pass/gaps.txt 8. Each completed trial verified the final value of every slot.

| Seed | Full key comparisons |
|---|---:|
|114731|408,556,901,247|
|114733|404,942,182,145|

The two-run mean is 406,749,541,696. The mean for the final unit-gap pass alone is 236,327,051,036.5. The empirical range is the interval between the two totals. Sampling ended during the third seed (114737) at the author's request. Only completed runs enter completed.csv. The theoretical relaxed-record cost 49,082,168,327 uses a different distribution for passes three through eight.
