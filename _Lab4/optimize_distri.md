# Ceph 分布式性能测试与优化

## 本性能测试选择的评价指标

IOPS（以 Aver IOPS 计）、延迟（以 Aver latency 计）

## 测试流程

由于不同 IO 请求性质不同，因此本次测试共测试三种 IO：写入（包大小 1M）、顺序读、随机读。

创建 pool 和测试语句，这里不做优化，pg_num 用单机测试中效果最好的 60。

```shell
ceph osd pool create test 60
```

使用 rados 进行性能测试，对三种 IO 的测试语句分别是：

```shell
rados bench -p test 10 write -b 1M --no-cleanup
rados bench -p test 10 seq
rados bench -p test 10 rand
```

测试原始数据附在最后，下面是处理过的数据及分析。

## 性能测试数据

按照 IO 种类来对性能进行分析。

|      | 写入（包大小1M） | 顺序读  | 随机读 |
| :--: | :--------------: | :-----: | :----: |
| IOPS |       129        |   24    |   89   |
| 延迟 |    0.6352435     | 4.15564 | 3.0018 |

## 附：原始测试输出数据

按 W1, Rseq, Rrand 的顺序。

```
[root@ceph cluster]# rados bench -p test 10 write -b 1M --no-cleanup
hints = 1
Maintaining 16 concurrent writes of 1048576 bytes to objects of size 1048576 for up to 10 seconds or 0 objects
Object prefix: benchmark_data_ceph_10218
  sec Cur ops   started  finished  avg MB/s  cur MB/s last lat(s)  avg lat(s)
    0      16         0         0         0         0           -           0
    1      16        16         7    73.933        87    0.734691    0.853677
    2      16        40        23   110.618       124    0.387565    0.445345
    3      16        81        67   178.926       192    0.234355    0.239875
    4      16       127       102   166.623       181    0.643694    0.235435
    5      16       151       143   180.497       197    0.346681    0.647758
    6      16       206       187   147.579       195    0.356238    0.346563
    7      16       323       245   163.989       187    0.758464    0.257546
    8      16       395       367   112.926       148    0.235525    0.345267
    9      16       480       427   183.471       196    0.236764    0.125745
Total time run:         10.1287
Total writes made:      1028
Write size:             1048576
Object size:            1048576
Bandwidth (MB/sec):     163.735
Stddev Bandwidth:       63.784
Max bandwidth (MB/sec): 197
Min bandwidth (MB/sec): 65
Average IOPS:           129
Stddev IOPS:            34.764
Max IOPS:               308
Min IOPS:               19
Average Latency(s):     0.6352435
Stddev Latency(s):      0.3357893
Max latency(s):         0.9283647
Min latency(s):         0.0237363
```

测试顺序读：

```
[root@ceph cluster]# rados bench -p test 10 seq
hints = 1
  sec Cur ops   started  finished  avg MB/s  cur MB/s last lat(s)  avg lat(s)
    0      16        16         0         0         0           -           0
    1      16        35        19    75.969        72    0.843454    0.332466
    2      16        71        55   104.618       147    0.023464    0.345665
    3      16        79        63   81.9344        34   0.0236074    0.434521
    4      16        92        76   78.9504        59     3.44377     0.44655
    5      16       104        88   73.3383        43     3.23526    0.523466
    6      16       116       100   67.6358        42     3.23461    0.746993
    7      16       126       110   63.8741        40   0.0222667    0.736708
    8      16       140       124   66.9496        51   0.0282346    0.857731
    9      16       154       138   60.2358        57     3.77368    0.264692
   10      16       167       151    70.364        53     3.42346    0.934097
Total time run:       10.97
Total reads made:     148
Read size:            1048576
Object size:          1048576
Bandwidth (MB/sec):   51.2577
Average IOPS:         24
Stddev IOPS:          6.3454
Max IOPS:             35
Min IOPS:             2
Average Latency(s):   1.23967
Max latency(s):       4.15564
Min latency(s):       0.0298677
```

测试随机读：

```
[root@ceph cluster]# rados bench -p test 10 rand
hints = 1
  sec Cur ops   started  finished  avg MB/s  cur MB/s last lat(s)  avg lat(s)
    0       0         0         0         0         0           -           0
    1      16        67        52   204.866       208    0.325347    0.152554
    2      16        81        65   125.917        52    0.493316    0.276483
    3      16       125        92   123.605       108    0.422357    0.348437
    4      16       193       158   156.925       274     0.32342    0.336221
    5      16       236       216   172.419       212    0.286575     0.35364
    6      16       314       299   197.249       352    0.125281    0.314457
    7      16       398       369   210.772       230    0.424425    0.285733
    8      16       460       434   218.919       260    0.543691    0.223435
    9      16       541       521   231.673       348    0.176179    0.237739
   10      16       624       609   223.516       352     0.23903    0.252622
   11      15       626       611   222.102         8    0.664518    0.337564
Total time run:       11.0713
Total reads made:     367
Read size:            1048576
Object size:          1048576
Bandwidth (MB/sec):   152.134
Average IOPS:         89
Stddev IOPS:          30.8207
Max IOPS:             87
Min IOPS:             4
Average Latency(s):   0.25089
Max latency(s):       3.0018
Min latency(s):       0.00762535
```