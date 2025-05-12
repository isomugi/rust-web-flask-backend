# import pytest
from app.models.user import User


def test_set_and_check_password():
    user = User()
    user.username = "alice"
    user.set_password("plain")
    assert user.check_password("plain")

    user.set_password("secret123")
    assert user.password_hash != "secret123"
    assert user.check_password("secret123")
    assert not user.check_password("wrongpass")
