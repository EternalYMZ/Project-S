# P01 的受限最优表与 P04 的带权菜单

2026 年 11 月 26 日。Miaozhen Yu（俞缪臻）。以下模型均在互异键上执行普通链内线性插入。

后续修订确定了奇偶和整除链目录的最优系数与必选间隔，并将三趟最优成本的加性误差收紧到 $O(n^{5/4})$。[定理条件与当前证明版本](periodic-catalogues.md)

## P01

在均匀随机排列上分别最小化三趟的平均移动数和平均比较数。充分大的每个规模都有一张公因数 2 的表，在两项均值上同时严格胜过每一张公因数至少 3 的表。因此，限制前两间隔非互素时，两项指标的每个精确最优表最终都取公因数 2。Lean 证明公因数 2 的一个试点低于 0.6431、所有更大公因数的临界系数至少为 0.646，并把分离转回真实整数表。解析证书把试点上界收紧到 0.63837。[Lean 声明](https://github.com/EternalYMZ/Optimal-mean-costs-of-three-pass-Shellsort/blob/62fb543048a26583db1d69810fae3ddc77469298/Shellsort/F09DivisorTwo.lean) · [标量证书与重放](https://github.com/EternalYMZ/Optimal-mean-costs-of-three-pass-Shellsort/blob/62fb543048a26583db1d69810fae3ddc77469298/audit/divisor-two/README.md)

固定公因数和形状后，前两趟的首项代价在平衡尺度相等。把第一间隔的归一化尺度调成平衡值的 $t$ 倍，并同步调第二间隔保持形状，额外首项恰为 $(t+t^{-1}-2)/(4a_*)$，其中 $a_*$ 为平衡尺度。这同时给出过大与过小两侧的代价。[形式化恒等式](https://github.com/EternalYMZ/Optimal-mean-costs-of-three-pass-Shellsort/blob/62fb543048a26583db1d69810fae3ddc77469298/Shellsort/F09ScalePenalty.lean)

## P04

每个间隔的概率至多为 $L/c$，且支持集至多有 $Hc$ 个间隔时，带权能量至多为均匀能量的 $LH$ 倍。把原来的均匀收缩系数预先取为 $\varepsilon/(LH)$，即可得到目标系数 $\varepsilon$。一般归约和种子中心 $c\le49{,}932$ 的实际带权三趟界均有 Lean 定理。全尺度谱估计有解析证明。若某素数同时整除两个保留间隔，第三间隔是该素数倍数的概率给出收缩系数的下限：沿模该素数的剩余类分别有序的输入，在这些选择下保持原状。[Lean 定理](https://github.com/EternalYMZ/Optimal-mean-comparisons-in-Shellsort/blob/8037c35ffe866415e8157c5d501ede5da9be79a2/Shellsort/WeightedMenu.lean) · [独立内核重放](https://github.com/EternalYMZ/Optimal-mean-comparisons-in-Shellsort/blob/8037c35ffe866415e8157c5d501ede5da9be79a2/certificates/weighted-menus/p04_weighted_kernel.json)

高概率选表时，每次成功右移都能表示成一次交换逆序对，至少消去一个全局逆序。因此一轮完整试排的比较数至多为 $n(n-1)/2+np$，其中 $p=\lfloor\log_2 n\rfloor$ 是趟数上限。对每张候选表分别给出尾界后使用联合界，所有候选可以共用同一批独立均匀排列。在固定置信度下，只需 $O(1+n/p)$ 个验证排列；一般置信度下的表执行次数还要乘以候选数。该开销属于选表预处理。最优均匀平均阶、逐个输入的随机表保证及高概率选表结论与 Du–Wan 同期工作重叠。[执行计数与尾界复核](https://github.com/EternalYMZ/Optimal-mean-comparisons-in-Shellsort/blob/8037c35ffe866415e8157c5d501ede5da9be79a2/certificates/weighted-menus/check_selection.py) · [Du–Wan 原文](https://arxiv.org/abs/2611.27671v1)
