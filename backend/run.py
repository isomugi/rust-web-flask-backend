import sys
from pathlib import Path
sys.path.append(str(Path(__file__).parent))

from app import create_app, db

app = create_app()

if __name__ == '__main__':
    with app.app_context():
        db.create_all()
    app.run(host='127.0.0.1', port=8080, debug=True) 