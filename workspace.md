<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
**Table of Contents**  *generated with [DocToc](https://github.com/thlorenz/doctoc)*

- [workspace](#workspace)
  - [依赖管理](#%E4%BE%9D%E8%B5%96%E7%AE%A1%E7%90%86)
  - [参考](#%E5%8F%82%E8%80%83)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

# workspace



> 若一个 Cargo.toml 有 [workspace] 但是没有 [package] 部分，则它是虚拟清单类型的工作空间。



## 依赖管理

![img.png](images/share-dependency1.png)
![img.png](images/share-dependency2.png)

my_app 依赖 my_lib, 这时候有个共同依赖 serde, 如果我们将my_lib的serde升级为一个新的版本，那么我们需要将my_app下的serde库也升级为新的版本。



## 参考

- [workspace共享依赖](https://www.cnblogs.com/w4ngzhen/p/18183529)