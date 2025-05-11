// メッセージの表示機構
export function showMessage(text, type='success') {
    const messageEl = document.getElementById('message');
    messageEl.textContent = text;
    messageEl.className = 'message ' + type;
    messageEl.style.display = 'block';
    setTimeout(() => { messageEl.style.display = 'none'; }, 3000);
}

// ナビの表示切替
export function initNav() {
    const token = localStorage.getItem('auth_token');
    document.getElementById('')
    document.getElementById('login-link').style.display  = token ? 'none' : 'inline';
    document.getElementById('logout-link').style.display = token ? 'inline' : 'none';
    if (token) {
      // ユーザー名はトークンデコード等でセットする想定
      // document.getElementById('username').textContent = decodeToken(token).username;

    }
  }

async function fetchUserInfo() {
  const token = localStorage.getItem('auth_token');
  const response = await fetch('http://localhost:8080/api/user/me', {
    headers: {
      'Authorization': `Bearer ${token}`
    }
  });
  if(response.ok){
    const user = await response.json();

  }else{
    
  }
}

document.addEventListener('DOMContentLoaded', initNav);