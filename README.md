> 这个图图我曾存过的。

# ImgStore

#### 主要是为了实现自己的一个小需求。

设想你在t冲浪时看到一张不错的~~h~~图，但是发布者忘记关水印开原图，你cy等待中国式英雄登场的同时决定把这张图先存进你的文件夹。许久之后你在p国旅游时终于偶然发现了正在寻找的原图，但是你已经不记得低清图什么时候有没有存过了，你也不想从你上千张存图里挑出来删掉，你只是一股脑扔进去。久而久之你发现存储空间红了，翻阅你的文件夹发现同一张图的8k4k2k1080p640x480带各种水印包浆不带水印版本前前后后出现了几十次，为了腾空间你就慢慢删把。

因此就有了这个工具。只要把你准备存的图交给它，让它在你的文件夹里检查有没有跟这张图相似的，然后让你决定怎么处理。

#### 同时作为一个现代客户端应用开发的练手项目。

本人对编程一窍不通，但是坚信HTML+CSS+JS是没有未来的。项目使用的Rust+Slint都是现学现卖的。

### 构建和使用

```
cargo tauri build -b none
```

使用：

```
imgstore --store 你的图库文件夹 你要存的图片
```