// index.html 専用スクリプト
document.addEventListener('DOMContentLoaded', () => {
    document.getElementById('api-test').addEventListener('click', async () => {
      try {
        const response = await fetch('http://localhost:8080/api/hello');
        const data = await response.json();
        document.getElementById('api-result').textContent = 'APIレスポンス: ' + JSON.stringify(data);
      } catch (err) {
        document.getElementById('api-result').textContent = 'エラー: ' + err.message;
      }
    });
    
    document.getElementById('logout-link').addEventListener('click', () => {
      localStorage.removeItem('auth_token');
      window.location.reload();
    });
  });
  