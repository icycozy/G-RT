# Week 1  Games101

## Lab1

### Task 1

* 注意到angle的计数方法，仅有一个数字代表累计距离初始位置旋转了多少，而无法确定在空间中的旋转，故仅支持在平面上转动，即只按a、d或只按r，但是可以任意固定方向转。

### Task 2

* 使用MSAA抗锯齿化后明显改善

![before](./fig/1.jpg)

![after](./fig/2.jpg)

* MSAA抗锯齿化之后三角形边缘会有黑色，应该是用整点标识pixel时的细节实现问题

### Task 3

* 一片漆黑。调不出来辣！TAT

github repository: git@github.com:icycozy/G-RT.git
commit: e0f3916

## 7.18 update

### Task2

FXAA 抗锯齿化: 效果不明显.

### Task3

渲染各款小牛.

* 小牛方向不对，没有关系，Task1写了旋转物体的函数，手动将小牛转到喜欢的角度:D

![normal](../Games101/normal.png)
![phong](../Games101/phong.png)
![texture](../Games101/texture.png)

* texture直接渲染纹理的图片，比较酷.

![bump](../Games101/bump2.png)
![displacement](../Games101/displacement2.png)

* 可以清晰看出加入凹凸映射之后的立体感增强.

![cool](../Games101/cool.png)

* 失败作，但是酷酷的艺术小牛.
