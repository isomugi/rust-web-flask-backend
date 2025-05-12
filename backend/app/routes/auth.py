from flask import Blueprint, jsonify, request
from flask_jwt_extended import create_access_token

# from werkzeug.security import generate_password_hash

from .. import db
from ..models.user import User

bp = Blueprint("auth", __name__, url_prefix="/api/auth")


@bp.route("/register", methods=["POST"])
def register():
    data = request.get_json()

    if not data or not data.get("username") or not data.get("password"):
        return jsonify({"error": "Username and password are required"}), 400

    if User.query.filter_by(username=data["username"]).first():
        return jsonify({"error": "Username already exists"}), 400

    user = User()
    user.username = data["username"]
    user.set_password(data["password"])

    db.session.add(user)
    db.session.commit()

    return (
        jsonify({"message": "User registerd successfully", "user": user.to_dict()}),
        201,
    )


@bp.route("/login", methods=["POST"])
def login():
    data = request.get_json()

    if not data or not data.get("username") or not data.get("password"):
        return jsonify({"error": "Username and password are required"}), 400

    user = User.query.filter_by(username=data["username"]).first()

    if user is None or not user.check_password(data["password"]):
        return jsonify({"error": "Invalid username or password"}), 401

    # ここで本来はトークンを生成して返す
    access_token = create_access_token(identity=str(user.id))
    return (
        jsonify(
            {
                "message": "Login successful",
                "token": access_token,
                "user": user.to_dict(),
            }
        ),
        200,
    )
