# import json


def test_register_and_login_and_me(client):
    # 新規登録
    resp = client.post(
        "api/auth/register", json={"username": "bob", "password": "pw123"}
    )
    assert resp.status_code == 201
    data = resp.get_json()
    assert data["message"] == "User registerd successfully"

    # 同じユーザーの再登録＝＞400
    resp2 = client.post(
        "api/auth/register", json={"username": "bob", "password": "pw123"}
    )
    assert resp2.status_code == 400

    # ログイン成功
    resp3 = client.post("api/auth/login", json={"username": "bob", "password": "pw123"})
    assert resp3.status_code == 200
    data3 = resp3.get_json()
    assert "token" in data3

    token = data3["token"]

    # 保護ルートの呼び出し
    resp4 = client.get("/api/users/me", headers={"Authorization": f"Bearer {token}"})
    print("RESP4 STATUS:", resp4.status_code)
    print("RESP4 JSON:  ", resp4.get_json())
    assert resp4.status_code == 200
    data4 = resp4.get_json()
    assert data4["username"] == "bob"
