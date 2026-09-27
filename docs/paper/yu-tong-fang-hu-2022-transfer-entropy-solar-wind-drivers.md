<!--
  title: Yu, Tong, Fang & Hu 2022 — TE ranking of solar wind drivers
  class: paper
  date: 2026-09-27
  sha256: 68c258354b01f1186e4efa6656d971e36cad412cd6d466e0691ec2f75616531f
  status: full-text (CJSS, open access, CC BY 3.0)
  see-also: docs/paper/gic-causal-driver.md
-->

# Yu, Tong, Fang & Hu 2022 — TE ranking of solar wind drivers

- **Authors:** YU Jiabin (1,2), TONG Jizhou (1), FANG Shaofeng (1), HU Xiaoyan (1).
  1 National Space Science Center, Chinese Academy of Sciences, Beijing 100190;
  2 University of Chinese Academy of Sciences.
- **Journal:** Chinese Journal of Space Science (空间科学学报), 2022, **42**(3): 346–356.
  (English title: *Transfer Entropy Approach to Discovering the Ranking of Solar Wind
  Drivers to Geomagnetic Storm*; Chinese title: 利用转移熵研究引起磁暴扰动的太阳风参数重要性排序.)
- **DOI:** `10.11728/cjss2022.03.210406045` — open access, **CC BY 3.0**.
- **Routes (measured 2026-09-27):** DOI 302 → `www.cjss.ac.cn` (HTTP 200); full text via
  `POST https://www.cjss.ac.cn/article/htmlContent` (JS-rendered `#htmlContent`);
  PDF via `GET https://www.cjss.ac.cn/article/exportPdf?id=597b207a-f197-43e4-ae95-3355b01f66de`
  (after `POST /article/checkArticlePdf`, 1 757 420 B, `%PDF-`), local copy
  `data/cjss.ac.cn/210406045.pdf`. `sciengine.com` mirror is HTTP 403 (WAF), irrelevant.
- **Text note:** inline MathML formulas were stripped on export; the numeric values in
  the tables are complete. Verbatim Chinese prose, sections 0–4, below.

## Abstract (English, verbatim)

> Geomagnetic storm is an important disaster event in space weather, which can affect satellite orbit and ground power system. At present, in the solar wind-magnetosphere system, most studies focus on the linear relationships analyzed by the correlation coefficient. However, transfer entropy can provide powerful model-free directed statistics, which can be used to analyze non-linear relationships that cannot be detected by traditional correlation analysis and model hypothesis. The hourly resolution data of solar activity cycle 23 and 24 were used to analyze the large time scale. The information transmission of solar wind and geomagnetic has a bimodal distribution, which is consistent with the solar activity level. Using the minute resolution data of 93 geomagnetic storms from 2010 to 2018 for small time scale analysis, the results show that E, IMF Bz have strong information transmission to the geomagnetic Sym-H parameter when the time delay is 60 minutes, while v_sw, T_sw, D_sw, B, P_sw are lower. It provides the basis for parameter selection and prediction range determination for solar wind-geomagnetic model construction.

## 0. 引言

太阳风–磁层相互作用形成了一个多尺度耦合的复杂系统，太阳风是引起地磁暴的最主要驱动源，例如当行星际磁场南向时，行星际磁场可与地球磁场发生磁场重联，较易引发磁暴。因而，理解太阳风变化对地磁场的影响机制，是预报地磁暴的关键之一。

有关太阳风与地磁扰动之间的相关性研究，新方法不断出现，这些方法主要基于太阳风和地磁参数的解析关系、相关系数或预报模型，线性回归、统计相关等方法，已被证实是认识磁暴时地磁变化的有效方法，例如Liu等[1]研究了太阳活动上升年的地磁Kp指数与行星际磁场南向分量、太阳风速度、太阳风温度与太阳风数密度Dsw的相关性，相关系数分别为0.66、0.03、0.58和–0.45，发现有一类磁暴与地磁场重联无关。Cane等[2]统计结果表明行星际磁场南向分量与磁暴扰动指数最小值的相关系数为0.74；Wu[3]统计分析了1995－1998年Wind数据，证实与的相关系数为0.87，略高于行星际磁场南向分量最小值与磁暴扰动指数最小值的相关系数0.81。Zhang等[4]发现行星际电场E与指数有很好的相关性。Zhao等[5]的研究表明在诸多参数中，就单一因素来说，行星际电场E对磁暴强度影响最大，行星际磁场南向分量对磁暴强度影响次之；Khabarova等[6]和Ahmed等[7]也得到了相似的结果。然而，地磁系统对太阳风变化的响应是非线性的，线性统计不足以表达其完备的相关性，同时在使用各种预报模型时，不同的输入参数对磁暴的影响和重要性仍不明确，关于哪个太阳风参数对磁暴的发生更重要，还未有统一的认识。

近年来，基于信息论的新方法[11]被提出并成功地应用于揭示地球磁层–电离层系统响应太阳风变化的复杂动力学特征。Michelis等[12]以地磁活动指数AL和Sym-H分别定量化亚暴和磁暴过程，进行了双变量转移熵分析，结果表明亚暴和磁暴之间的信息流动方向取决于全球地磁活动水平。Wing等[13]利用信息论进行太阳风–地球辐射带系统研究，利用互信息、条件互信息和转移熵对能量范围为1.8~3.5 MeV的地球同步轨道高能电子通量的太阳风驱动因素进行了研究，得出了基于信息论分析的重要性排序。转移熵可以提供强大的无模型统计量，在太阳风–地球辐射带系统研究已经得到了验证，但目前还未在太阳风–磁层整体耦合系统中得到应用。本文利用转移熵研究太阳风参数与地磁指数之间的非线性关系，应用转移熵来衡量磁暴时的太阳风驱动因素并进行量化，对不同太阳风参数的重要性进行研究。

## 1. 数据与方法

### 1.1 数据来源

用于研究的太阳风及地磁活动指数的数据均来源于美国国家航空航天局（NASA）的OMNI公开数据，具体包括分钟分辨率数据（HRO）和小时分辨率数据（LRO），时间跨度从1996年到2018年。OMNI整合了来自ACE、WIND、IMP8等航天器的太阳风磁场和等离子体数据集，并根据实际的太阳风速度时移到地球的弓激波鼻尖处。此外，本研究还使用了世界地磁数据中心（WDC）提供的地磁Sym-H指数数据、比利时SILSO中心提供的太阳黑子数据、和加拿大自然资源部的太阳10.7 cm射电流量F10.7数据。

本研究利用美国空间天气研究中心（SWRC）提供的空间天气数据库（DONKI）中提取的磁暴事件列表，共计93个事例，主要分布在第24太阳活动周期间的2010年到2018年。

### 1.2 相关系数与转移熵

相关系数又称皮尔逊积矩相关系数，适用于度量两个变量之间的线性相关性，其值介于1与–1之间。时移相关系数是皮尔逊积矩相关系数向两个时间序列的拓展，其中为时间延迟。当相关函数有多个峰或没有明显的不对称时，这种分析的结果可能不是特别清楚。此外，相关系数是两个变量之间线性关系的一个度量，不能用于描述非线性关系。

信息论中互信息[14]表示一个随机变量中包含的关于另一个随机变量的信息量，代表两个随机变量的依赖程度。互信息对联合概率进行了定量描述，相比于相关系数，更好地表达了两个变量之间的依赖关系。如果两变量服从联合高斯概率分布，互信息与互相关系数等价，例如当满足高斯分布的变量的相关系数=0.5时，等价于=0.14 nats。互信息的取值范围为，经过转化的取值范围位于。

转移熵是一个量化两个系统相关程度或混沌程度的参数，可用于处理有限长度的信号数据。转移熵定义由Schreiber[15]于2000年提出，同时考虑源序列和目标序列的数据长度，通用的转移熵定义为（默认单位为nats）。转移熵可以被视为一种特殊形式的条件互信息[13]。转移熵可以衡量系统中已知目标历史序列的情况下，有多少信息从输入序列传递到目标序列的下一状态。转移熵与相关系数的不同在于方向性。

转移熵算法容易受到有限大小样本效应的影响，也容易受到数值效应的影响，因此建立一个阈值来界定转移熵计算结果是否显著是必要的。为了检验计算结果的统计学意义，将源序列随机打乱重新抽样，这一步骤保证了置换时间序列具有与原始序列相同的均值、方差、自相关函数，但破坏了非线性关系。这个过程的目的是消除两个序列数据之间的所有潜在关联关系，因此计算得到的理论值为零。但是在有限时间序列中，由于有限样本效应很少为零，故而可以得到转移熵高于即可认为是显著的。通过100次随机的零假设抽样计算，若根据置换序列计算出的新值的95%小于原来，则可以认为原始序列计算得来的显著。

运用时移相关系数、转移熵分析行星际磁场南向分量Bz与地磁指数Sym-H之间的相关性。使用1996－2018年小时精度数据对时移相关系数和转移熵进行逐年计算。针对每一年的数据，将时移相关系数的最大时间延迟设置为120 h，以步长1 h逐步计算，即每年共120个数据点，然后取相关系数绝对值最大的数据点作为衡量该年和地磁指数Sym-H之间的相关系数计算结果。同理，计算转移熵时采用同样的时间延迟设置和最大值选取方法，由此可得图1。图1中1996年行星际磁场南向分量与地磁指数的最大时移相关系数=0.39；在太阳活动高年2000年=0.35，同极小年的差异率为11%。另外，1996年和2000年行星际磁场南向分量与地磁Sym-H指数之间的转移熵分别为0.18 nats、0.26 nats，差异率为31%。相关系数在太阳活动高低年的差异性相对较小，转移熵在太阳活动高低年表现出明显的差异。与传统相关系数的对比表明，转移熵可以有效表征太阳风–磁层关系的相关性。

以太阳活动周期的角度看，各太阳风参数向地磁Sym-H指数信息传递的强弱与太阳活动水平的周期性变化一致。另外，太阳10.7 cm射电流量（F10.7）是综合衡量太阳色球、过渡区和日冕极紫外辐射强度的一种常用指数，以F10.7作为衡量太阳活动水平的指标可得与太阳黑子数相同的结果。因此，转移熵可以更好地定量化太阳活动与太阳风–磁层系统的相关性。

## 2. 太阳活动水平对太阳风–磁层相互作用的影响

以往太阳风–磁层耦合关系多参数影响因素的重要性研究，往往仅集中在个别太阳风参数，例如行星际磁场南向分量或行星际电场，而与其他参数，例如行星际磁场大小、太阳风速度、太阳风数密度、太阳风温度、太阳风动压等的综合研究较少涉及。Schwenn等[16]发现，地磁扰动在大小和方向上都与行星际磁场波动密切相关。长期南向的行星际磁场与地球磁场之间的相互连接允许太阳风能量传输到地球磁层[17]。包括Kissinger等[18]在内的多项研究表明了太阳风速度在磁暴产生中的作用。另外，增强的太阳风密度也是一个经常影响磁暴强度的重要参数。Ahmed等[7]的研究表明太阳风密度与其他参数协同具有更好的相关性，而太阳风温度则表现较差。Xie等[19]多篇论文描述了高速太阳风动压与磁暴的关系。Mcpherron等[20]的研究表明磁暴时的环电流积聚原则上可以直接由太阳风电场驱动。Kane[21]表示磁暴期间太阳风速度与行星际磁场南向分量的共同作用效果明显优于；Wang等[22]研究表明，对于磁暴的形成，的重要性强于行星际磁场南向分量增强的持续时间。

### 2.1 太阳风参数与地磁指数的转移熵

实验使用1996－2018年小时精度的太阳风数据，主要分析参数包括行星际磁场、行星际磁场南向分量、太阳风速度、太阳风等离子体数密度、太阳风温度、太阳风动压、行星际电场，以及小时精度的地磁Sym-H指数数据。图2展示了利用转移熵方法分析各太阳风参数与地磁Sym-H指数的相关性结果，即：1996－2018年中，行星际电场与地磁指数之间的信息传递均最高，随后是和，最低是；、和位于中部且相互差异较小，排名在各年中略有不同。各太阳风参数与地磁Sym-H指数转移熵的变化趋势表现类似于图1中行星际磁场南向分量与地磁Sym-H指数的转移熵相似的特性：（1）在太阳活动高年，更为显著；（2）对于任一太阳风参数，在第24太阳活动周的峰值比23太阳活动周要低。

### 2.2 转移熵与太阳活动水平相关性分析

如图2所示，除行星际磁场南向分量外，行星际磁场、太阳风速度、太阳风等离子体数密度、行星际电场及地磁Sym-H指数转移熵表现出与太阳活动周类似的双峰结构。同时将太阳活动指数太阳黑子数、F10.7进行相关性分析，结果列于表1。由表1可知，相关系数位于[0.61, 0.83]之间、位于[0.58, 0.82]之间，均表现出较强的相关性。同时转移熵位于[0.15, 0.36]之间、位于[0.12, 0.33]之间，即太阳活动指数与存在明显的信息传递。由此可知，以小时精度的数据分析，行星际磁场南向分量与太阳活动之间的强相关并非偶然，其他各太阳风参数与太阳活动水平也存在较强的关联，佐证了Liu等[23]对于地球近地空间环境变化对太阳活动的依赖性。

## 3. 引起磁暴扰动的太阳风参数重要性排序

### 3.1 数据选择

磁暴是指整个地球磁层发生的持续十几个小时到几十个小时的一种剧烈地磁扰动，中低纬度地磁台站水平分量的显著减小为磁暴的主要特征。在磁暴初相和主相期间，一般会出现太阳风–磁层发电机效应，将导致磁层对流电场快速渗透到中低纬度地区。由于不同的物理成因，应分别研究磁暴主相和恢复相，目前本文将研究范围限定于磁暴主相阶段。

根据DONKI提供的磁暴列表，其中小磁暴共计6个（约占7%），中等强度的磁暴共计58个（约占62%），大磁暴及特大磁暴共计29个（约占31%）。磁暴峰值的选取从开始时间向后搜索48 h，搜索期间指数的最小值作为该磁暴的峰值，将峰值所对应的时间定为该磁暴的峰值时间。由表2可知，磁暴主相的平均持续时间不足10 h，因此分钟精度的Sym-H指数可以更细致地刻画磁暴主相的变化过程，实验采用了2010－2018年分钟精度的太阳风数据，以及分钟精度的地磁Sym-H指数数据。

### 3.2 重要性排序方法

以往研究结果表明，地磁指数对太阳风变化的响应时间普遍小于3 h[11,28]，本研究将响应时间范围设定在0至500 min，与Stumpo等[11]研究磁层电离层对太阳风的响应时间范围一致。以往的转移熵往往集中在单个时间序列，而磁暴期间的重要性系数为磁暴集合（见表2）上的太阳风参数对地磁Sym-H指数的转移熵平均值。相应地由转移熵定义的重要性系数以及信噪比（S/N）、显著性检验指标由式（12）–（17）给出。转移熵算法通过置换时间序列[15]来创建零假设下的概率分布来计算噪声，噪声的均值和标准偏差是由100次随机的零假设抽样计算平均而来。

### 3.3 重要性排序

对行星际磁场南向分量和地磁Sym-H指数之间的转移熵进行计算，首先对每个磁暴事件进行独立的转移熵分析，而后以相同的时间延迟计算多个磁暴事件转移熵的平均值。由图3可以看出，转移熵在60 min处达到最大值=0.244 nats、=0.196 nats，信噪比=4.15和显著性=36.83σ。转移熵明显大于噪声，即信息传递显著。而基本都处于背景噪声的内部或边缘，不显示任何明显的时间延迟特征，信息传递不显著。

经比较，上述结果与Runge等[28]的结论基本一致，即行星际磁场南向分量扰动是导致行星际条件变化的重要部分；Stumpo等[11]的研究也表明，在时间延迟时行星际磁场南向分量与Sym-H指数之间有较强的信息传递。但是已有研究并未涉及太阳风温度、行星际电场等太阳风参数的综合比较。进一步利用转移熵分析磁暴期间其他太阳风参数，包括行星际磁场、太阳风速度、太阳风等离子体数密度、太阳风温度、太阳风动压和行星际电场，对Sym-H指数的影响，结果如图4所示。各太阳风参数对地磁Sym-H指数的转移熵均高于噪声；而则处于噪声边缘，缺乏显著性。最大值出现的时间普遍小于3 h，反应出磁层对太阳风变异性的响应是突变式的激增，表现为短时间内一个强烈的信息传递。

**表3 太阳风参数对地磁Sym-H指数重要性系数排序（T(E), nats）**

| 序号 | 太阳风参数 | T_E (nats) | S/N | 显著性 σ | 相关系数 | D_M |
|---|---|---|---|---|---|---|
| 1 | 行星际电场 (E) | 0.200 | 3.92 | 37.43 | 0.473 | 0.101 |
| 2 | 行星际磁场南向分量 (Bz) | 0.196 | 4.15 | 36.83 | 0.462 | 0.107 |
| 3 | 太阳风速度 (v_sw) | 0.159 | 3.15 | 29.92 | 0.322 | 0.200 |
| 4 | 太阳风温度 (T_sw) | 0.157 | 3.08 | 29.17 | 0.224 | 0.285 |
| 5 | 太阳风数密度 (D_sw) | 0.148 | 2.93 | 28.00 | 0.244 | 0.262 |
| 6 | 行星际磁场 (B) | 0.146 | 3.10 | 27.45 | 0.290 | 0.213 |
| 7 | 太阳风动压 (P_sw) | 0.135 | 2.68 | 25.58 | 0.209 | 0.277 |

由表3可知，排在第一位的是行星际电场，=0.200 nats。第二位是行星际磁场南向分量，=0.196 nats，略低于。然后排名依次是太阳风速度、太阳风温度、太阳风密度、行星际磁场。最低的是太阳风动压，=0.135 nats。

值得注意的是，行星际电场对地磁Sym-H指数的转移熵在60 min处达到最大值=0.252 nats、=0.200 nats，信噪比=3.92和显著性=37.43σ，也有明显的信息传递，甚至比更强烈。以相关系数为基础进行排序，前三位仍是行星际电场、行星际磁场南向分量和太阳风速度，但是行星际磁场的相对重要性提升至第四位，太阳风温度下降至第六位。转移熵和相关系数排序的差异，表明了太阳风参数与地磁Sym-H指数之间关系的复杂性。

## 4. 结论

应用转移熵研究了地磁扰动的太阳风驱动因素，可以得出如下结论。

1. 从太阳活动周期的角度分析，各太阳风参数向地磁Sym-H指数信息传递的强弱与太阳活动水平的周期性变化一致。
2. 磁暴期间各太阳风参数向地磁Sym-H指数信息传递，最相关的变量是行星际电场，行星际磁场南向分量次之，然后排名依次是太阳风速度、太阳风温度、太阳风等离子体数密度、行星际磁场，最弱的是太阳风动压。
3. 行星际电场与地磁指数之间的信息传递在60 min处达到峰值，比略高，佐证了Zhao等[5]的研究。

本文将转移熵方法推广至磁暴事件集合上的平均转移熵，定义了磁暴期间太阳风参数与地磁Sym-H指数的重要性系数，并综合考虑了7种太阳风参数，以重要性系数为依据获得了引发磁暴扰动的太阳风参数重要性排序。

## Figure captions

- 图 1 行星际磁场南向分量Bz与地磁Sym-H指数的转移熵和相关系数 (Transfer entropy and correlation coefficient between IMF Bz and Sym-H).
- 图 2 各太阳风参数对地磁Sym-H指数的转移熵 (Transfer entropy of multiple solar wind parameters to Sym-H).
- 图 3 行星际磁层南向分量Bz和地磁Sym-H指数的转移熵 (Transfer entropy of Bz and Sym-H).
- 图 4 行星际磁场、太阳风速度、太阳风等离子体数密度、太阳风温度、太阳风动压、行星际电场对地磁Sym-H指数的转移熵 (Transfer entropy of B, v_sw, D_sw, T_sw, P_sw, E to Sym-H).

## References

1. 刘绍亮, 陈剑利. 空间科学学报, 1988, 8(1): 35-38.
2. CANE H V, RICHARDSON I G, ST CYR O C. Geophysical Research Letters, 2000, 27(21): 3591-3594. doi:10.1029/2000GL000111.
3. WU C C. Journal of Geophysical Research, 2002, 107(A10): 1314. doi:10.1029/2001JA000161.
4. 张继春, 田剑华, 濮祖荫. 空间科学学报, 2001, 21(4): 297-304. doi:10.3969/j.issn.0254-6124.2001.04.002.
5. 赵明现, 乐贵明, 刘玉洁. 空间科学学报, 2006, 26(6): 421-426. doi:10.3969/j.issn.0254-6124.2006.06.003.
6. KHABAROVA O, PILIPENKO V, ENGEBRETSON M J, et al. ICS-8, University of Calgary Press, 2006: 127-132.
7. AHMED L, EL-ERAKI M A, SAMY A, et al. Space Weather, 2018, 16(9): 1277-1290. doi:10.1029/2018SW001863.
8. IYEMORI T, MAEDA H. Solar-Terrestrial Predictions Proceedings, Vol. 4, Boulder, 1980.
9. JI E Y, MOON Y J, GOPALSWAMY N, et al. JGR: Space Physics, 2012, 117(A3): A03209.
10. RASTÄTTER L, KUZNETSOVA M M, GLOCER A, et al. Space Weather, 2013, 11(4): 187-205. doi:10.1002/swe.20036.
11. STUMPO M, CONSOLINI G, ALBERTI T, et al. Entropy, 2020, 22(3): 276. doi:10.3390/e22030276.
12. DE MICHELIS P, CONSOLINI G, MATERASSI M, et al. JGR: Space Physics, 2011, 116(A8): A08225.
13. WING S, JOHNSON J R, CAMPOREALE E, et al. JGR: Space Physics, 2016, 121(10): 9378-9399. doi:10.1002/2016JA022711.
14. TSONIS A A. Nonlinear Processes in Geophysics, 2001, 8(6): 341-345. doi:10.5194/npg-8-341-2001.
15. SCHREIBER T. Physical Review Letters, 2000, 85(2): 461-464. doi:10.1103/PhysRevLett.85.461.
16. SCHWENN R, DAL LAGO A, HUTTUNEN E, et al. Annales Geophysicae, 2005, 23(3): 1033-1059. doi:10.5194/angeo-23-1033-2005.
17. GONZALEZ W D, JOSELYN J A, KAMIDE Y, et al. JGR: Space Physics, 1994, 99(A4): 5771-5792. doi:10.1029/93JA02867.
18. KISSINGER J, MCPHERRON R L, HSU T S, et al. JGR: Space Physics, 2011, 116(A5): A00I19. doi:10.1029/2010JA015763.
19. XIE H, GOPALSWAMY N, ST CYR O C, et al. Geophysical Research Letters, 2008, 35(6): L06S08. doi:10.1029/2007GL032298.
20. MCPHERRON R L, BAKER D N, BARGATZE L F, et al. Advances in Space Research, 1988, 8(9/10): 71-86.
21. KANE R P. JGR: Space Physics, 2005, 110(A2): A02213. doi:10.1029/2004JA010799.
22. WANG Y M, SHEN C L, WANG S, et al. Geophysical Research Letters, 2003, 30(20): 2039. doi:10.1029/2003GL017901.
23. 刘立波, 万卫星, 陈一定, 等. 科学通报, 2011, 56(12): 1202-1211. doi:10.1007/s11434-010-4226-9.
24. 吴迎燕, 徐文耀, 陈耿雄, 等. 地球物理学报, 2007, 50(1): 1-9. doi:10.3321/j.issn:0001-5733.2007.01.001.
25. BLANC M, RICHMOND A D. JGR: Space Physics, 1980, 85(A4): 1669-1686. doi:10.1029/JA085iA04p01669.
26. KOZYRA J U, JORDANOVA V K, HOME R B, et al. Magnetic Storms, Vol. 98, AGU, 1997: 187-202.
27. 史良文, 申成龙, 汪毓明. 地球物理学报, 2014, 57(11): 3822-3833. doi:10.6038/cjg20141136.
28. RUNGE J, BALASIS G, DAGLIS I A, et al. Scientific Reports, 2018, 8(1): 16987. doi:10.1038/s41598-018-35250-5.

## Relevance to `docs/paper/gic-causal-driver.md`

Yu et al. rank solar-wind drivers to the **Sym-H storm index** by transfer entropy over 93
storms (2010–2018), with a **source-shuffled permutation null** (100 resamples, 95 %
threshold) and **no family-wise correction**; they report E and Bz dominant at a
**60-minute delay** (E 0.200 nats, Bz 0.196 nats; E's peak 0.252 nats at 60 min). Their
target is the magnetospheric index, **not ground dB/dt**, and the null is a shuffle, not
phase-randomized — so our chain (L1 Bz → ground dB/dt at an auroral-zone station, strict
phase-randomized null, family-wise bound) is **not preempted**; the 60-minute Bz/E
dominance is **corroborated**. Their Table 3 shows the TE ranking diverging from the
correlation ranking — the same method argument our paper makes for TE over correlation.
