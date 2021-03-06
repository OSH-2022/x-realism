<img src="./realism.svg" style="zoom: 67%;" />

*Yet Another OS Group's project repository.*

## ð¨âð¨âð¦âð¦ æå Members

- [é»çè½©](https://github.com/SproutNan)ï¼PB20111686ï¼
- [åè¯å®](https://github.com/liuly0322)ï¼PB20000180ï¼
- [è®¸å¤é](https://github.com/xkz0777)ï¼PB20111714ï¼
- [å¶åå®](https://github.com/ysy-phoenix)ï¼PB20111701ï¼

## â¿ é¡¹ç®ç®ä» Project Introduction

æ¬é¡¹ç®ä¸ºä½¿ç¨ Rust ç¼ç¨è¯­è¨ç»ç»ä¸ä¸ªæ³¨éæ§è½ãå¹¶ååå®å¨çå¾®åæ ¸æä½ç³»ç»ã

## ð é¡¹ç®è¿å± Process Managment

|    Date    |         Title         |                            Result                            |    Notes     |
| :--------: | :-------------------: | :----------------------------------------------------------: | :----------: |
| 2022.3.13ð |     éä½è°ç éé¢      | [preliminary research](./reports/2022.3.13&#32;preliminary&#32;research.pdf) |  èå¸å·²åå¤  |
| 2022.3.20ð |     éä½è®¨è®ºå®é¢      | [research](./reports/2022.3.20&#32;research.md) and [plan](./reports/2022.3.20&#32;plan.md) |  èå¸å·²åå¤  |
| 2022.3.27ð |   è°ç æ¥åä»»å¡å®æ    |     [discussion](./reports/2022.3.27&#32;discussion.md)      |              |
| 2022.4.03ð |    å®æè°ç æ¥åv0     |           [research_v0](./reports/research-v0.md)            |              |
| 2022.4.04ð |  è°ç æ¥å - å¼ä¼è®¨è®º  |     [meeting summary](./reports/2022.4.4&#32;meeting.md)     | èå¸åä¼æå¯¼ |
| 2022.4.09ð |    å®æè°ç æ¥åv1     |           [research_v1](./reports/research-v1.md)            |              |
| 2022.4.16ð |    è®¨è®ºå¯è¡æ§æ¥å     |                                                              |              |
| 2022.4.19ð |   å®æå¯è¡æ§æ¥åv0    |        [feasibility_v0](./reports/feasibility-v0.md)         |              |
| 2022.4.23ð | å®æä¸­ææ±æ¥åå®¹åPPT |                                                              |              |
| 2022.4.25ð |     è¿è¡ä¸­ææ±æ¥      |                                                                                                    |              |
| 2022.5.09ð |      ä¸èå¸è®¨è®º       |                                                             |              |

## ðºï¸ æä»¶å¤¹è¯´æ Folder Description

- `reports`: åç±»æ¥å
- `docs`: é¡¹ç®ç Github Pages
- `src`: é¡¹ç®çä»£ç ç­èµæº

## ð§ æå»ºæ¹å¼ Build

Rust éè¦å®è£ nightly çï¼

```bash
$ rustup default nightly
```

å®è£éè¦çä¾èµï¼

```bash
$ cargo install bootimage
```

å®è£ qemuï¼åèè¯¥[ææ¡£](https://www.qemu.org/download/)å³å¯ã

ä¹åï¼

```bash
$ git clone git@github.com:OSH-2022/x-realism.git
$ cd srcs
$ cargo run
```

å³å¯å®ææ¬é¡¹ç®æå»º

## â è®¸å¯è¯ License

`x-realism` é¡¹ç®éç¨ `MIT` åè®®å¼æºï¼è¯¦ç»ä¿¡æ¯è¯·åè `LICENCE` è®¸å¯è¯ã

## ð åè References

1. [âå¨å½å¤§å­¦çæä½ç³»ç»æ¯èµ2022"ç¸å³ä¿¡æ¯](https://github.com/oscomp)ï¼[2022.3.13]
2. [x-DisGraFSçåæè°ç æ¥å](https://github.com/OSH-2021/x-DisGraFS/blob/main/docs/%E5%89%8D%E6%9C%9F%E8%B0%83%E7%A0%94%E5%86%85%E5%AE%B9/%E5%BE%80%E5%B1%8AOSH%E8%AF%BE%E9%A2%98%E8%B0%83%E7%A0%94%E6%8A%A5%E5%91%8A.md)ï¼[2022.3.13]
3. [rCore-Tutorial-Book ç¬¬ä¸ç](https://rcore-os.github.io/rCore-Tutorial-Book-v3/)ï¼[2022.3.20]
