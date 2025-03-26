
20240116，大学倒数第二次英语考试，即兴有了想法做一个词典；
剑桥词典一直是我最喜欢的词典，没有找到网上大神做的本地词典，所以想做个更方便一点的命令行版；
rust 入门项目，代码质量不佳，欢迎PR

英文版说明：

### 安装

更新也是这个
```bash
bash -c "$(curl -fsSL https://github.com/xwxb/camdict-cli/raw/main/install.sh)"
```

卸载 
```bash
sudo rm /usr/local/bin/fcd
```


### 使用
```bash
fcd [word] | [phrase]
```
（fcd: Find by Cambridge Dict)

默认展示三条释义，`-a` 参数全部展开，`-n` 参数指定具体数目

### 其他
灵感和参考致谢
- https://github.com/ChestnutHeng/Wudao-dict

- https://github.com/chengqing97/jc-dict
- https://github.com/Tontuu/diglish


TODO
- [ ] 处理 `-` 符号的情况，和一般标点不一样
- [ ] 多词性处理（默认有一部分处理），调整显示三次的逻辑
- [ ] 优化执行速度
- [ ] 增强鲁棒性，尽量不报 rust 内部的错误：网络错误处理、单词没找到情况的处理
- [ ] 中文反查
- [ ] 模糊搜索
- [ ] 最近搜索
- [ ] 添加英文文档和开源许可证
- [ ] LRU 缓存最近查询，默认50，支持自定义数量

暂时感觉没必要做，有需求可以提一下具体使用场景
- [ ] 词型派生处理？暂时没想到规则是什么，目前情况是存在对应词型，有些是确实查不到需要处理
- [ ] 查询缓存
- [ ] 交互模式
- [ ] 相似单词建议？
- [ ] 支持根据 LRU 词频生成单词本
