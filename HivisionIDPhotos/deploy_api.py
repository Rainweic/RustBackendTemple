from fastapi import FastAPI, UploadFile, Form, HTTPException
from pydantic import BaseModel
from typing import Optional
from demo.processor import IDPhotoProcessor
import numpy as np
import cv2
import base64

app = FastAPI()
processor = IDPhotoProcessor()

def numpy_to_base64(img: np.ndarray):
    retval, buffer = cv2.imencode(".png", img)
    return base64.b64encode(buffer).decode("utf-8")

class IDPhotoRequest(BaseModel):
    mode_option: str = "尺寸列表"
    size_list_option: Optional[str] = "一寸"
    color_option: str = "白色"
    render_option: str = "纯色"
    image_kb_options: str = "不压缩"
    custom_color_R: Optional[int] = None
    custom_color_G: Optional[int] = None
    custom_color_B: Optional[int] = None
    custom_size_height: Optional[int] = None
    custom_size_width: Optional[int] = None
    custom_image_kb: Optional[int] = None
    language: str = "zh"
    matting_model_option: str = "hivision_modnet"
    watermark_option: str = "不添加水印"
    watermark_text: Optional[str] = None
    watermark_text_color: Optional[str] = "#000000"
    watermark_text_size: Optional[int] = 20
    watermark_text_opacity: Optional[float] = 0.5
    watermark_text_angle: Optional[int] = 30
    watermark_text_space: Optional[int] = 25
    face_detect_option: str = "mtcnn"
    head_measure_ratio: float = 0.2
    top_distance_max: float = 0.12
    top_distance_min: float = 0.10

@app.post("/process_idphoto")
async def process_idphoto(
    input_image: UploadFile,  # 要处理的输入图像文件
    mode_option: str = Form("尺寸列表"),  # 尺寸模式选项: "尺寸列表", "自定义尺寸", "只换底"
    size_list_option: Optional[str] = Form("一寸"),  # 预设尺寸选项,如"一寸", "小一寸"等
    color_option: str = Form("白色"),  # 背景颜色选项: "白色", "蓝色", "红色", "自定义底色"
    render_option: str = Form("纯色"),  # 渲染选项: "纯色", "渐变"
    image_kb_options: str = Form("不压缩"),  # 图像压缩选项: "不压缩", "自定义"
    custom_color_R: Optional[int] = Form(None),  # 自定义背景颜色的红色分量 (0-255)
    custom_color_G: Optional[int] = Form(None),  # 自定义背景颜色的绿色分量 (0-255)
    custom_color_B: Optional[int] = Form(None),  # 自定义背景颜色的蓝色分量 (0-255)
    custom_size_height: Optional[int] = Form(None),  # 自定义尺寸的高度
    custom_size_width: Optional[int] = Form(None),  # 自定义尺寸的宽度
    custom_image_kb: Optional[int] = Form(None),  # 自定义压缩大小 (KB)
    language: str = Form("zh"),  # 语言选项: "zh" (中文), "en" (英文)
    matting_model_option: str = Form("hivision_modnet"),  # 人像抠图模型选项
    watermark_option: str = Form("不添加水印"),  # 水印选项: "不添加水印", "添加水印"
    watermark_text: Optional[str] = Form(None),  # 水印文本
    watermark_text_color: Optional[str] = Form("#000000"),  # 水印颜色,十六进制格式
    watermark_text_size: Optional[int] = Form(20),  # 水印文字大小
    watermark_text_opacity: Optional[float] = Form(0.5),  # 水印不透明度 (0-1)
    watermark_text_angle: Optional[int] = Form(30),  # 水印角度 (0-360)
    watermark_text_space: Optional[int] = Form(25),  # 水印间距
    face_detect_option: str = Form("mtcnn"),  # 人脸检测模型选项
    head_measure_ratio: float = Form(0.2),  # 头部比例 (0.1-0.5)
    top_distance_max: float = Form(0.12),  # 头顶最大距离 (0.02-0.5)
    top_distance_min: float = Form(0.10)  # 头顶最小距离 (0.02-0.5)
):
    try:
        image_bytes = await input_image.read()
        nparr = np.frombuffer(image_bytes, np.uint8)
        img = cv2.imdecode(nparr, cv2.IMREAD_COLOR)

        result = processor.process(
            img,
            mode_option,
            size_list_option,
            color_option,
            render_option,
            image_kb_options,
            custom_color_R,
            custom_color_G,
            custom_color_B,
            custom_size_height,
            custom_size_width,
            custom_image_kb,
            language,
            matting_model_option,
            watermark_option,
            watermark_text,
            watermark_text_color,
            watermark_text_size,
            watermark_text_opacity,
            watermark_text_angle,
            watermark_text_space,
            face_detect_option,
            head_measure_ratio,
            top_distance_max,
            top_distance_min
        )

        return {
            "status": "success",
            "standard_image": numpy_to_base64(result[0]),
            "hd_image": numpy_to_base64(result[1]),
            "standard_png": numpy_to_base64(result[2]),
            "hd_png": numpy_to_base64(result[3]),
            "layout_image": numpy_to_base64(result[4].value) if result[4].value is not None else None,
            "notification": result[5].value if result[5].visible else None,
            "download_path": result[6].value if result[6].visible else None
        }
    except Exception as e:
        raise HTTPException(status_code=400, detail=str(e))


if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=7890)