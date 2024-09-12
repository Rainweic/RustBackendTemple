# ID 照片处理 API 文档

## 端点：/process_idphoto

方法：POST

此端点用于处理 ID 照片，提供多种自定义选项。

### 请求参数

1. input_image: UploadFile（必需）
   - 要处理的输入图像文件

2. mode_option: str（默认值："尺寸列表"）
   - 模式选项

3. size_list_option: str（可选，默认值："一寸"）
   - 尺寸列表选项

4. color_option: str（默认值："白色"）
   - 背景颜色选项

5. render_option: str（默认值："纯色"）
   - 渲染选项

6. image_kb_options: str（默认值："不压缩"）
   - 图像压缩选项

7. custom_color_R: int（可选）
   - 自定义背景颜色 R 值

8. custom_color_G: int（可选）
   - 自定义背景颜色 G 值

9. custom_color_B: int（可选）
   - 自定义背景颜色 B 值

10. custom_size_height: int（可选）
    - 自定义尺寸高度

11. custom_size_width: int（可选）
    - 自定义尺寸宽度

12. custom_image_kb: int（可选）
    - 自定义图像大小（KB）

13. language: str（默认值："zh"）
    - 语言选项

14. matting_model_option: str（默认值："hivision_modnet"）
    - 人像抠图模型选项
    - 可用选项：从 HUMAN_MATTING_MODELS_EXIST 中获取

15. watermark_option: str（默认值："不添加水印"）
    - 水印选项

16. watermark_text: str（可选）
    - 水印文本

17. watermark_text_color: str（可选，默认值："#000000"）
    - 水印文本颜色

18. watermark_text_size: int（可选，默认值：20）
    - 水印文本大小

19. watermark_text_opacity: float（可选，默认值：0.5）
    - 水印文本不透明度

20. watermark_text_angle: int（可选，默认值：30）
    - 水印文本角度

21. watermark_text_space: int（可选，默认值：25）
    - 水印文本间距

22. face_detect_option: str（默认值："mtcnn"）
    - 人脸检测模型选项
    - 可用选项：["face++ (联网Online API)", "mtcnn", "retinaface-resnet50"]（如果可用）

23. head_measure_ratio: float（默认值：0.2）
    - 头部测量比例

24. top_distance_max: float（默认值：0.12）
    - 顶部最大距离

25. top_distance_min: float（默认值：0.10）
    - 顶部最小距离

### 响应

返回一个包含处理后图像和相关信息的 JSON 对象：

- status: 处理状态
- standard_image: 标准图像（Base64 编码）
- hd_image: 高清图像（Base64 编码）
- standard_png: 标准 PNG 图像（Base64 编码）
- hd_png: 高清 PNG 图像（Base64 编码）
- layout_image: 布局图像（Base64 编码，如果可用）
- notification: 通知信息（如果可用）
- download_path: 下载路径（如果可用）

### 响应

返回一个包含可用模型列表的 JSON 对象：

- human_matting_models: 可用的人像抠图模型列表
- face_detect_models: 可用的人脸检测模型列表