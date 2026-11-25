"""Exact exponent certificate for the explicit all-four-pass constant.
The analytic input log(16n) <= n^(1/128) for n>=2^4096 is
certified by: ln(2)<=1, ln(2)>=1/2, exp(x)>=1+x, and the two
integer inequalities recorded below.
"""
from fractions import Fraction as F
from pathlib import Path
import json

if not __debug__:
    raise RuntimeError("Run without -O so every exponent bound is checked")

s0=4096
ell=49
# Every listed exponent bounds the base-2 logarithm at n=2^4096
# after replacing L_n by n^(1/128); all relevant powers decrease.
upper={
    "small_row_mode_term": F(8)+ell*F(15,4)-s0*(F(1,14)-F(1,256)),
    "small_row_start_term": F(1)+ell*F(15,2)-s0*F(1,7),
    "ideal_term": ell*F(7,2)-s0*(F(1,6)-F(1,128)),
    "large_row_near_term": F(21)+ell*F(7,2)-s0*(F(1,6)-F(3,256)),
    "large_row_far_term": F(4)+ell*F(7,2)-s0*F(1,6),
}
lower={
    "h_over_g": -ell*F(7,2)+s0*F(1,6),
    "p_over_2304Ln_using_2304_lt_2pow12":
        -F(2)-ell*F(7,2)-12+s0*(F(1,6)-F(1,128)),
    "band_over_32pi_using_102400_lt_2pow17":
        -17-ell*F(11,4)+s0*F(1,14),
}
assert upper["small_row_mode_term"]<=-1
assert upper["small_row_start_term"]<=-1
assert upper["ideal_term"]<=0
assert upper["large_row_near_term"]<=-1
assert upper["large_row_far_term"]<=-1
assert lower["h_over_g"]>=2
assert lower["p_over_2304Ln_using_2304_lt_2pow12"]>=0
assert lower["band_over_32pi_using_102400_lt_2pow17"]>=0
assert 2304<2**12 and 102400<2**17
assert 2**32>=4100 and 2**32>=256
weighted=F(5*3+4*11+4*19+8*57,21)
small=2+s0*F(8,21)
assert weighted<29<2048
assert small<2048
assert F(29,21)-F(11,8)==F(1,168)

result={
    "all_checks_passed":True,
    "arithmetic":"exact integer and Fraction",
    "lambda_log2":ell,
    "N0_log2":s0,
    "upper_log2_exponents":{k:str(v) for k,v in upper.items()},
    "lower_log2_exponents":{k:str(v) for k,v in lower.items()},
    "large_weighted_constant_negative_log2_bound":str(weighted),
    "finite_n_constant_negative_log2_bound":str(small),
    "chosen_uniform_constant_negative_log2":2048,
    "log_majorant_integer_checks":{
        "2pow32_ge_4100":True,
        "2pow32_ge_256":True,
    },
}
expected = json.loads(Path(__file__).with_suffix(".json").read_text(encoding="utf-8"))
if result != expected:
    raise ValueError("The recomputed R10 exponent certificate differs from the archived result")
print(json.dumps(result, ensure_ascii=False, indent=2))
