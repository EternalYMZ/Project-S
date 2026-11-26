# Janson–Knuth 第二条猜想的全规模结果

在 $n$ 个互异键的均匀随机排列上作普通 $(h,g,1)$ 插入排序，令 $M_3$ 为末趟的成功右移数，$D=n\psi(h,g)-\mathbb E M_3$。Janson 与 Knuth 在第 10 节猜测 $D\ge0$。他们关于绝对误差的第一条猜想已在 P01 三趟综合稿证明。[原文第 10 节](https://arxiv.org/pdf/cs/9608105#page=15) · [第一条猜想的来源及范围](janson-knuth.md)

| 参数 | 解析结论 |
|---|---|
| $g=3$，$\gcd(h,3)=1$ | 每个 $n\ge1$ 与 $h\ge2$ 均有 $D>0$，$h=1$ 时取等号。等长源按相位成块，余下四个源的阶乘矩可精确计算。[P02 定理与复核程序](https://github.com/EternalYMZ/Three-pass-Shellsort-inversion-error/tree/5874bd4e3889ccd31ac75ab5ce01b0a0d7a2f84e/research/three-phase) |
| $h\ge2^{21}$，$g\ge2$，$\gcd(h,g)=1$ | 对所有 $n\ge1$，令 $H=\min(1,g^2/(\lfloor n/h\rfloor+1))$，有 $ng\sqrt h H/16384<D<2ng\sqrt h H$。真实的条件弧方差与按长短链比例计算的圆周平均方差之差至多 $25/4$。[P01 定理与精确核验](https://github.com/EternalYMZ/Optimal-mean-costs-of-three-pass-Shellsort/tree/d02645adc1b0fecbd7f5044a4ce37dac732bf5de/audit/signed-deficit) |
| $h=2$，奇数 $g\ge3$ | 所有输入长度均有严格符号。固定 $g$ 时，$D$ 的极限是 $(g^2-1)(g^2-5)/(96g)$。这些表的第二间隔较大。[两源计算](https://github.com/EternalYMZ/Three-pass-Shellsort-inversion-error/blob/5874bd4e3889ccd31ac75ab5ce01b0a0d7a2f84e/research/three-phase/check_small_gap_sign.py) |

当 $h,g\to\infty$ 且 $n/(hg^2)\to\infty$ 时，
$$
 \frac{D}{g^3h^{3/2}}\longrightarrow\mathcal B,\qquad
 0.002804727333<\mathcal B<0.002821272543 .
$$

区间来自带严格尾界的有向舍入积分。固定互素 $h,g$ 时则可解有限泊松方程，精确得到未经归一化的长数组极限。32 个计算实例包含 $(4,3)$ 的 $55/81$ 和 $(5,4)$ 的 $16631/7680$。[常数证书](https://github.com/EternalYMZ/Optimal-mean-costs-of-three-pass-Shellsort/blob/d02645adc1b0fecbd7f5044a4ce37dac732bf5de/audit/signed-deficit/deep_constant.json) · [固定间隔有理证书](https://github.com/EternalYMZ/Optimal-mean-costs-of-three-pass-Shellsort/blob/d02645adc1b0fecbd7f5044a4ce37dac732bf5de/audit/signed-deficit/poisson_witness.json)

对于通常的递减表 $n>h>g>1$，仍待证明的参数落在有限多对互素间隔 $4\le g<h<2^{21}$，其中每对要处理无穷多个输入长度。三相证明也检验了逐个离散秩估计的限制，$(7,4,3)$ 的中间秩给出负的 $-41/1890$。[独立物理卷积与全排列复算](https://github.com/EternalYMZ/Three-pass-Shellsort-inversion-error/blob/5874bd4e3889ccd31ac75ab5ce01b0a0d7a2f84e/research/three-phase/independent/triple_sign_replay.py)

Lean 4.36 新增了真实排列的条件弧积分、异参数 Bernoulli 和的凸序及方差恒等式，以及将局部充分条件转回真实末趟的定理。14 个新声明入口的 46,064 个传递声明已分别通过 Nanoda 和 Lean 默认内核。三相全规模定理、大间隔门槛与深链常数由解析证明给出。[Lean 入口](https://github.com/EternalYMZ/Optimal-mean-costs-of-three-pass-Shellsort/blob/d02645adc1b0fecbd7f5044a4ce37dac732bf5de/JKSign/Review.lean) · [内核记录](https://github.com/EternalYMZ/Optimal-mean-costs-of-three-pass-Shellsort/blob/d02645adc1b0fecbd7f5044a4ce37dac732bf5de/audit/signed-deficit/kernel.json)
