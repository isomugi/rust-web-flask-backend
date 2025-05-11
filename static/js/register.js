// register.html 専用スクリプト
import { showMessage } from './common.js';
document.addEventListener('DOMContentLoaded', () => {
    document.getElementById('register-form').addEventListener('submit', async (e) => {
      e.preventDefault();
      const username = document.getElementById('username').value;
      const password = document.getElementById('password').value;
      if (!username || !password) {
        return showMessage('ユーザー名とパスワードを入力してください','error');
      }
      try {
        const response = await fetch('http://localhost:8080/api/auth/register', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ username, password })
        });
        const result = await response.json();
        if (response.ok) {
          showMessage('登録成功！','success');
          setTimeout(() => { window.location.href = 'login.html'; }, 1000);
        } else {
          showMessage('登録失敗：' + (result.error || 'エラーが発生しました'), 'error');
        }
      } catch (err) {
        showMessage('通信エラー：' + err.message, 'error');
      }
    });
  });
  