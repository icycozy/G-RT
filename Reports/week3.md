# Week 3  Ray Tracing: The Next Week

* 根据时间渲染动态物体的模糊.

![moving balls](../RayTracer/output/moving_balls.jpg)  

* 增加AABB和bvh，运行速度显著提升.
* 新建texture.rs，渲染纹理.

![checkered sphere](../RayTracer/output/checkered_spheres.jpg)

* 新建rtw.rs，导入图片. 连接stb库，包装stb_image.h，成功让rust使用c头文件.
  * fix: 直接用opencv::imgcodecs::imread.
* 加入perlin noise. Hermitian Smoothing.

![perlin noise](../RayTracer/output/perlin.jpg)
![marble texture](../RayTracer/output/perlin2.jpg)

* terbulence.
* 新增quad类.
* 新增light emitting material.

![we have lights!](../RayTracer/output/diffuse_light.jpg)

* 渲染cornell box.

![cornell box](../RayTracer/output/cornell_box.jpg)

* move and rotate box.
* add volumes.

![smoke boxes](../RayTracer/output/smoke_cornell.jpg)

* final picture.

![All objects in one picture](../RayTracer/output/FinalImage.jpg)

* 实现多线程.

commit: e77d4ecb1b
