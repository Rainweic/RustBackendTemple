import requests
import time
from concurrent.futures import ThreadPoolExecutor, as_completed
import base64


# 定义API的基础URL
BASE_URL = "http://localhost:8080"

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
        "mode_option": "尺寸列表",
        "size_list_option": "一寸",
        "color_option": "白色",
        "render_option": "纯色",
        "image_kb_options": "不压缩",
        "language": "zh",
        "matting_model_option": "hivision_modnet",
        "watermark_option": "不添加水印",
        "face_detect_option": "mtcnn",
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