from flask import Flask
from flask_sqlalchemy import SQLAlchemy
from flask_jwt_extended import JWTManager
from flask_cors import CORS
import os

# データベースのインスタンスを作成
db = SQLAlchemy()
# JWTのインスタンスを作成
jwt = JWTManager()

def create_app():
    app = Flask(__name__)
    
    # アプリケーションの設定
    app.config['SQLALCHEMY_DATABASE_URI'] = 'sqlite:///users.db'
    app.config['SQLALCHEMY_TRACK_MODIFICATIONS'] = False
    app.config['SECRET_KEY'] = os.environ.get('SECRET_KEY', 'dev')
    app.config['JWT_SECRET_KEY'] = os.environ.get('JWT_SECRET_KEY', 'dev')
    
    # CORSを有効化
    CORS(app)
    
    # データベースの初期化
    db.init_app(app)

    # JWTの初期化
    jwt.init_app(app)
    

    # ルートの登録
    from .routes import auth, users
    app.register_blueprint(auth.bp)
    app.register_blueprint(users.bp)
    
    return app 