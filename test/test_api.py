import requests
import time
from concurrent.futures import ThreadPoolExecutor, as_completed

# 定义API的基础URL
BASE_URL = "http://localhost:8080"

# 定义LOCALES字典
LOCALES = {
    'face_model': {'en': {'label': 'Face detection model'}, 'zh': {'label': '人脸检测模型'}},
    'matting_model': {'en': {'label': 'Matting model'}, 'zh': {'label': '抠图模型'}},
    'key_param': {'en': {'label': 'Key Parameters'}, 'zh': {'label': '核心参数'}},
    'advance_param': {'en': {'label': 'Advance Parameters'}, 'zh': {'label': '高级参数'}},
    'size_mode': {'en': {'label': 'ID photo size options', 'choices': ['Size List', 'Only Change Background', 'Custom Size'], 'custom_size_eror': 'The width should not be greater than the length; the length and width should not be less than 100, and no more than 1800.'}, 'zh': {'label': '证件照尺寸选项', 'choices': ['尺寸列表', '只换底', '自定义尺寸'], 'custom_size_eror': '宽度不应大于长度；长度和宽度不应小于100，不大于1800。'}},
    'size_list': {'en': {'label': 'Size list', 'choices': ['One inch\t\t(413, 295)', 'Two inches\t\t(626, 413)', 'Small one inch\t\t(378, 260)', 'Small two inches\t\t(531, 413)', 'Large one inch\t\t(567, 390)', 'Large two inches\t\t(626, 413 )', 'Five inches\t\t(1499, 1050)', 'Teacher qualification certificate\t\t(413, 295)', 'National civil service exa\t\t(413, 295)', 'Primary accounting exam\t\t(413, 295)', 'English CET-4 and CET-6 exams\t\t(192, 144)', 'Computer level exam\t\t(567, 390)', 'Graduate entrance exam\t\t(709, 531)', 'Social security card\t\t(441, 358)', "Electronic driver's license\t\t(378, 260)", 'American visa\t\t(600, 600)', 'Japanese visa\t\t(413, 295)', 'Korean visa\t\t(531, 413)'], 'develop': {'One inch\t\t(413, 295)': (413, 295), 'Two inches\t\t(626, 413)': (626, 413), 'Small one inch\t\t(378, 260)': (378, 260), 'Small two inches\t\t(531, 413)': (531, 413), 'Large one inch\t\t(567, 390)': (567, 390), 'Large two inches\t\t(626, 413 )': (626, 413), 'Five inches\t\t(1499, 1050)': (1499, 1050), 'Teacher qualification certificate\t\t(413, 295)': (413, 295), 'National civil service exa\t\t(413, 295)': (413, 295), 'Primary accounting exam\t\t(413, 295)': (413, 295), 'English CET-4 and CET-6 exams\t\t(192, 144)': (192, 144), 'Computer level exam\t\t(567, 390)': (567, 390), 'Graduate entrance exam\t\t(709, 531)': (709, 531), 'Social security card\t\t(441, 358)': (441, 358), "Electronic driver's license\t\t(378, 260)": (378, 260), 'American visa\t\t(600, 600)': (600, 600), 'Japanese visa\t\t(413, 295)': (413, 295), 'Korean visa\t\t(531, 413)': (531, 413)}}, 'zh': {'label': '预设尺寸', 'choices': ['一寸\t\t(413, 295)', '二寸\t\t(626, 413)', '小一寸\t\t(378, 260)', '小二寸\t\t(531, 413)', '大一寸\t\t(567, 390)', '大二寸\t\t(626, 413)', '五寸\t\t(1499, 1050)', '教师资格证\t\t(413, 295)', '国家公务员考试\t\t(413, 295)', '初级会计考试\t\t(413, 295)', '英语四六级考试\t\t(192, 144)', '计算机等级考试\t\t(567, 390)', '研究生考试\t\t(709, 531)', '社保卡\t\t(441, 358)', '电子驾驶证\t\t(378, 260)', '美国签证\t\t(600, 600)', '日本签证\t\t(413, 295)', '韩国签证\t\t(531, 413)'], 'develop': {'一寸\t\t(413, 295)': (413, 295), '二寸\t\t(626, 413)': (626, 413), '小一寸\t\t(378, 260)': (378, 260), '小二寸\t\t(531, 413)': (531, 413), '大一寸\t\t(567, 390)': (567, 390), '大二寸\t\t(626, 413)': (626, 413), '五寸\t\t(1499, 1050)': (1499, 1050), '教师资格证\t\t(413, 295)': (413, 295), '国家公务员考试\t\t(413, 295)': (413, 295), '初级会计考试\t\t(413, 295)': (413, 295), '英语四六级考试\t\t(192, 144)': (192, 144), '计算机等级考试\t\t(567, 390)': (567, 390), '研究生考试\t\t(709, 531)': (709, 531), '社保卡\t\t(441, 358)': (441, 358), '电子驾驶证\t\t(378, 260)': (378, 260), '美国签证\t\t(600, 600)': (600, 600), '日本签证\t\t(413, 295)': (413, 295), '韩国签证\t\t(531, 413)': (531, 413)}}},
    'bg_color': {'en': {'label': 'Background color', 'choices': ['Blue', 'White', 'Red', 'Black', 'Dark Blue', 'Light Gray', 'Custom'], 'develop': {'Blue': '628bce  ', 'White': 'ffffff  ', 'Red': 'd74532  ', 'Black': '000000  ', 'Dark Blue': '4b6190', 'Light Gray': 'f2f0f0'}}, 'zh': {'label': '背景颜色', 'choices': ['蓝色', '白色', '红色', '黑色', '深蓝色', '浅灰色', '自定义底色'], 'develop': {'蓝色': '628bce', '白色': 'ffffff', '红色': 'd74532', '黑色': '000000', '深蓝色': '4b6190', '浅灰色': 'f2f0f0'}}},
    'button': {'en': {'label': 'Start'}, 'zh': {'label': '开始制作'}},
    'head_measure_ratio': {'en': {'label': 'Head ratio'}, 'zh': {'label': '面部比例'}},
    'top_distance': {'en': {'label': 'Top distance'}, 'zh': {'label': '头距顶距离'}},
    'image_kb': {'en': {'label': 'Set KB size', 'choices': ['Not Set', 'Custom']}, 'zh': {'label': '设置 KB 大小', 'choices': ['不设置', '自定义']}},
    'image_kb_size': {'en': {'label': 'KB size'}, 'zh': {'label': 'KB 大小'}},
    'render_mode': {'en': {'label': 'Render mode', 'choices': ['Solid Color', 'Up-Down Gradient (White)', 'Center Gradient (White)']}, 'zh': {'label': '渲染方式', 'choices': ['纯色', '上下渐变（白色）', '中心渐变（白色）']}},
    'watermark_tab': {'en': {'label': 'Watermark'}, 'zh': {'label': '水印'}},
    'watermark_text': {'en': {'label': 'Text', 'value': 'Hello', 'placeholder': 'up to 20 characters'}, 'zh': {'label': '水印文字', 'value': 'Hello', 'placeholder': '最多20个字符'}},
    'watermark_color': {'en': {'label': 'Color'}, 'zh': {'label': '水印颜色'}},
    'watermark_size': {'en': {'label': 'Size'}, 'zh': {'label': '文字大小'}},
    'watermark_opacity': {'en': {'label': 'Opacity'}, 'zh': {'label': '水印透明度'}},
    'watermark_angle': {'en': {'label': 'Angle'}, 'zh': {'label': '水印角度'}},
    'watermark_space': {'en': {'label': 'Space'}, 'zh': {'label': '水印间距'}},
    'watermark_switch': {'en': {'label': 'Watermark', 'value': 'Not Add', 'choices': ['Not Add', 'Add']}, 'zh': {'label': '水印', 'value': '不添加', 'choices': ['不添加', '添加']}},
    'notification': {'en': {'label': 'notification', 'face_error': 'The number of faces is not equal to 1, please upload an image with a single face. If the actual number of faces is 1, it may be an issue with the accuracy of the detection model. Please switch to a different face detection model on the left or raise a Github Issue to notify the author.'}, 'zh': {'label': '通知', 'face_error': '人脸数不等于1，请上传单人照片。如果实际人脸数为1，可能是检测模型的准确度问题，请切换左侧不同的人脸检测模型或提出Github Issue通知作者。'}},
    'standard_photo': {'en': {'label': 'Standard photo'}, 'zh': {'label': '标准照'}},
    'hd_photo': {'en': {'label': 'HD photo'}, 'zh': {'label': '高清照'}},
    'standard_photo_png': {'en': {'label': 'Matting Standard photo'}, 'zh': {'label': '透明标准照'}},
    'hd_photo_png': {'en': {'label': 'Matting HD photo'}, 'zh': {'label': '透明高清照'}},
    'layout_photo': {'en': {'label': 'Layout photo'}, 'zh': {'label': '六寸排版照'}},
    'download': {'en': {'label': 'Download the photo after adjusting the KB size'}, 'zh': {'label': '下载调整 KB 大小后的照片'}},
    'matting_image': {'en': {'label': 'Matting image'}, 'zh': {'label': '抠图图像'}}
}

# 测试用户注册API
def test_register_user():
    url = f"{BASE_URL}/user/register"
    payload = {
        "username": "testuser",
        "password": "testpassword"
    }
    response = requests.post(url, json=payload)
    print(f"Register User API response: {response.status_code}, {response.text}")

# 测试用户登录API
def test_user_login():
    url = f"{BASE_URL}/user/login"
    payload = {
        "username": "testuser",
        "password": "testpassword"
    }
    response = requests.post(url, json=payload)
    print(f"Login API response: {response.status_code}, {response.text}")
    return response.json().get("token")

def upload_image(token):
    url = f"{BASE_URL}/image/upload"
    headers = {
        "Authorization": f"Bearer {token}"
    }
    
    # 直接打开图片文件，不需要base64编码
    files = {
        'input_image': ('image.jpg', open('images.jpeg', 'rb'), 'image/jpeg')
    }
    
    data = {
        "mode_option": LOCALES['size_mode']['zh']['choices'][0],  # 尺寸列表
        "size_list_option": "一寸\t\t(413, 295)",  # 选择一个预设尺寸
        "color_option": LOCALES['bg_color']['zh']['choices'][0],  # 蓝色
        "render_option": LOCALES['render_mode']['zh']['choices'][0],  # 纯色
        "image_kb_options": LOCALES['image_kb']['zh']['choices'][0],  # 不设置
        "language": "zh",
        "matting_model_option": LOCALES['matting_model']['zh']['label'],  # 抠图模型
        "watermark_option": LOCALES['watermark_switch']['zh']['choices'][0],  # 不添加水印
        "face_detect_option": LOCALES['face_model']['zh']['label'],  # 人脸检测模型
        "head_measure_ratio": "0.2",
        "top_distance_max": "0.12",
        "top_distance_min": "0.10"
    }
    
    start_time = time.time()
    response = requests.post(url, headers=headers, files=files, data=data)
    print(f"Upload Image API response: {response.status_code}, {response.text}")
    end_time = time.time()
    return response.status_code, end_time - start_time

def test_upload_image_performance(token, num_requests, max_workers):
    with ThreadPoolExecutor(max_workers=max_workers) as executor:
        futures = [executor.submit(upload_image, token) for _ in range(num_requests)]
        
        successful_requests = 0
        total_time = 0
        
        for future in as_completed(futures):
            status_code, request_time = future.result()
            if status_code == 200:
                successful_requests += 1
                total_time += request_time

    print(f"Total requests: {num_requests}")
    print(f"Successful requests: {successful_requests}")
    print(f"Failed requests: {num_requests - successful_requests}")
    print(f"Total time: {total_time:.2f} seconds")
    print(f"Average request time: {total_time / num_requests:.4f} seconds")
    print(f"Requests per second: {num_requests / total_time:.2f}")

if __name__ == "__main__":
    test_register_user()
    token = test_user_login()
    num_requests = 1  # 总请求数
    max_workers = 1  # 最大并发数

    test_upload_image_performance(token, num_requests, max_workers)