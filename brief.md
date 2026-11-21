# Project S 的证明从哪里开始？

把长度为五的数组 $[2,0,4,1,3]$ 按间隔 $2$ 排一遍。偶数槽组成的链从 $[2,4,3]$ 变成 $[2,3,4]$，奇数槽里的 $[0,1]$ 已有序。程序做了一次成功右移，两个键各改变了一个链内位置，也就是各跨过两个物理槽。完成这一趟后，键 $0$ 仍在键 $2$ 的右边。一次右移、四格物理位移和剩余的跨链逆序，分别回答了三个具体问题。

**给键标上最终秩，可以追踪下一趟从什么状态开始。** 固定一个秩阈值，把比它小的键涂色。一条链排完后，着色键恰好占据链首的一段，长度等于这一条链原有的着色键数。这样，只记录各链前缀的长度，就能计算下一组槽位会读到多少着色键。证明可以使用最终秩；普通 Shellsort 仍逐链做线性插入，每一次键比较和成功右移都按它实际走过的步骤计算。P00 给出单趟链内位移的基本换算。[链内位移的 Lean 定义与恒等式](https://github.com/EternalYMZ/Chain-displacement-in-Shellsort/blob/51421b8d6708f3de0b4b5b6c41e38db775b95f9c/Shellsort/ChainTransport.lean)

**固定三遍，和允许趟数增长，会给出不同的最优值。** 在均匀随机的互异键上，间隔表在读取输入前由 $n$ 决定。P01 构造一张三遍表给出上界，并逐类处理每一张合法三遍表给出下界；分别优化平均成功右移数、平均键比较数，二者都为 $\Theta(n^{3/2})$。P04 允许随规模增加趟数。在每一组相近尺度的间隔中，证明选出一个对这一规模所有输入共用的间隔，控制该组结束时的平均平方位移。尺度逐步缩小，累积的平均键比较数为 $O(n\log n)$。对每张预定表，比较记录又必须区分 $n!$ 个等可能输入，因此最佳平均键比较数为 $\Theta(n\log n)$。这项存在性结论没有给出廉价生成精确最优表的算法。[P01 三遍最小值](https://github.com/EternalYMZ/Optimal-mean-costs-of-three-pass-Shellsort/blob/f83ba5cca0512e1406e71107487a9034cdf5bda1/Shellsort/ThreePassOptimalMean.lean) · [P04 预定表最小值](https://github.com/EternalYMZ/Optimal-mean-comparisons-in-Shellsort/blob/fe3fbebaa0bada514b2fe3e6c2cd61cd99a858f4/Shellsort/VerifiedOptimum.lean)

**同一串中间状态还可以提出另一个执行问题。** 假设每一趟应该得到的数组已经给出，允许直接把所需记录写进任意槽。P08 证明，这时必须且只需为每对相邻状态中内容不同的槽各写一次。知道目标改变了允许的操作；这笔写入账与普通插入的右移账各有自己的执行定理。[检查点之间的最少主数组写入](https://github.com/EternalYMZ/Shellsort-checkpoint-writes/blob/7920dab6af0fbe89b72d2389582556253a89189f/Shellsort/PhysicalSlotWrites.lean)

按问题选读时，可以从 P03 的四遍下界、P06 的任意有序分区、P09 的选表成本或 P10 的精确赢家进入。[总目录](https://github.com/EternalYMZ/Project-S/blob/697ed43421b4ef767c877093834b360fdc916b75/README.md) · [公开 Lean 源码索引](https://github.com/EternalYMZ/Project-S/blob/697ed43421b4ef767c877093834b360fdc916b75/MODULES.md)
