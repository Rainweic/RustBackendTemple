import requests

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
    print(f"Register User API response: {response.status_code}, {response}")

# 测试用户登录API
def test_user_login():
    url = f"{BASE_URL}/user/login"
    payload = {
        "username": "testuser",
        "password": "testpassword"
    }
    response = requests.post(url, json=payload)
    print(f"Login API response: {response.status_code}, {response.json()}")
    return response.json().get("token")

# 测试图片上传API
def test_upload_image(token):
    url = f"{BASE_URL}/image/upload"
    headers = {
        "Authorization": f"Bearer {token}"
    }
    files = {
        "file": ("test.png", open("test.png", "rb"), "image/png")
    }
    response = requests.post(url, headers=headers, files=files)
    print(f"Upload Image API response: {response.status_code}, {response}")

if __name__ == "__main__":
    test_register_user()
    token = test_user_login()
    if token:
        test_upload_image(token)