# 三趟反演误差、最优间隔与四趟构造

在均匀随机排列上按预定表 $(h,g,1)$ 作普通线性插入，$M_3$ 是进入末趟时的反演数。Janson 与 Knuth 在 1996 年预印本第 10 节先后提出统一绝对误差界和全规模单侧符号。他们在第 9 节已证明一族间隔的总平均成本为 $O(n^{23/15})$。[原文第 10 节](https://arxiv.org/pdf/cs/9608105#page=15)

P02 原工作稿给出了第一条猜想的解析证明。综合后的 P01 对任意正互素 $h,g$ 和所有 $n\ge1$ 给出 $|\mathbb E M_3-n\psi(h,g)|\le\pi^2g^3h^{3/2}/(2\sqrt3)$。真实弧和参考二项分布给出两个不同的临界极限。因此，在 $h/\sqrt n,g/n^{1/4}$ 落入正紧矩形的全部互素表上，充分大时 $n\psi-\mathbb E M_3=\Theta(n^{3/2})$。该窗内的符号和误差阶均由解析式确定。Lean 证明了真实成本恒等式、参考二项和、有限耦合和给定积分弧界后的全局估计。该积分弧界来自稿内的条件化和 Fourier 证明。[定理、18,790 项声明的内核重放与历史实验的精确重算](https://github.com/EternalYMZ/Optimal-mean-costs-of-three-pass-Shellsort/tree/86868d7f2b2ed2271235e357698544dbff7cbea8/audit/inversion-error)

右移和键比较分别最小化，均有最优值 $\kappa n^{3/2}+O(n^{5/4})$，其中 $0.60822153\le\kappa<0.60827485406$。经典试表 $(q^2+1,q,1)$ 的系数大于 $0.62525495$。若改取 $q=\lfloor(n/2)^{1/4}\rfloor$，显式四趟表 $(2q^2+2,2q,2,1)$ 的系数小于 $0.59880$。对整族 $(dH,dG,d,1)$ 优化时，首项最优族最终取 $d=2$，系数位于 $(0.5867418,0.5867796)$。一般四趟的指数问题仍由 P03 单列。[四趟精确分解、长短链边界与系数检查](https://github.com/EternalYMZ/Optimal-mean-costs-of-three-pass-Shellsort/blob/86868d7f2b2ed2271235e357698544dbff7cbea8/audit/inversion-error/check_lift.py)

P02 专项推进了 $g=2$ 的全规模符号和 $n/h\ge1$ 的统一误差剖面。该剖面在 $\rho=1$ 达到最小值，在每个整数 $\rho\ge2$ 处有严格局部极小。一般互素间隔、所有规模的单侧猜想仍然开放。[剖面、精确值与重算](https://github.com/EternalYMZ/Three-pass-Shellsort-inversion-error/tree/b98b0c9a71d7ca4a5e72e122b058a00a21f0fcf1/research/parity-profile)
