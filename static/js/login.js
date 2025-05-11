// login.html 専用スクリプト
import { showMessage } from './common.js';
document.addEventListener('DOMContentLoaded', () => {
    document.getElementById('login-button').addEventListener('click', async (e) => {
      e.preventDefault();
      const username = document.getElementById('username').value;
      const password = document.getElementById('password').value;
      if (!username || !password) {
        return showMessage('ユーザー名とパスワードを入力してください','error');
      }
      try {
        const response = await fetch('http://localhost:8080/api/auth/login', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ username, password })
        });
        const data = await response.json();
        if (response.ok) {
          localStorage.setItem('auth_token', data.token);
          showMessage('ログイン成功！','success');
          setTimeout(() => { window.location.href = 'index.html'; }, 1000);
        } else {
          showMessage('ログイン失敗：' + (data.error || 'エラーが発生しました'), 'error');
        }
      } catch (err) {
        showMessage('通信エラー：' + err.message, 'error');
      }
    });
  });
  