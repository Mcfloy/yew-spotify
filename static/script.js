// TODO: Transfer the methods to Rust
function parse_hash(str) {
  const parts = str.substr(1).split('&');
  const map = new Map();
  parts.flatMap(part => {
    const [key, value] = part.split('=');
    map.set(key, value);
  });
  return map;
}

function getCode(str) {
  return str.substr(6);
}

function mapIsOk(map) {
  if (!map.has('access_token') || !map.has('expires_in')) {
    return false;
  }
}

async function checkStoredAuthToken() {
  const token = localStorage.getItem('refresh_token');
  const expires_after = localStorage.getItem('expires_after');
  if (!expires_after || !token) {
    clearCookies();
  } else if (Date.now() > expires_after) {
    await refreshToken();
  }
}

function clearCookies() {
  localStorage.removeItem('access_token');
  localStorage.removeItem('token_type');
  localStorage.removeItem('expires_after');
  localStorage.removeItem('refresh_token');
}

const clientId = '';
const authorizationHeader = 'Basic ';

const scopes = [
  'user-read-playback-state',
  'user-modify-playback-state',
  'user-read-currently-playing',
  'streaming',
  'app-remote-control',
  'user-read-email',
  'user-read-private',
  'playlist-read-collaborative',
  'playlist-modify-public',
  'playlist-read-private',
  'playlist-modify-private',
  'user-library-modify',
  'user-library-read',
  'user-top-read',
  'user-read-playback-position',
  'user-read-recently-played',
  'user-follow-read',
  'user-follow-modify'
];
const authorizationCodeUrl = `https://accounts.spotify.com/authorize?client_id=${clientId}&response_type=code&redirect_uri=http%3A%2F%2F127.0.0.1%3A8887&scope=${scopes.join('%20')}`;
const accessTokenUrl = `https://accounts.spotify.com/api/token`;

async function getAccessAndRefreshTokens(code) {
  const formData = new FormData();
  formData.set('grant_type', 'authorization_code');
  formData.set('code', code);
  formData.set('redirect_uri', 'http://127.0.0.1:8887');

  const response = await fetch(accessTokenUrl, {
    method: 'POST',
    headers: {
      'Authorization': authorizationHeader,
      'Content-Type': 'application/x-www-form-urlencoded'
    },
    body: new URLSearchParams(formData)
  });
  if (response.ok) {
    const { access_token, expires_in, refresh_token } = await response.json();
    localStorage.setItem('access_token', access_token);
    localStorage.setItem('expires_after', Date.now() + (expires_in * 1000));
    localStorage.setItem('refresh_token', refresh_token);
  } else {
    console.warn('Cannot get access token or refresh token', await response.text());
  }
}

async function refreshToken() {
  const refresh_token = localStorage.getItem('refresh_token');

  const formData = new FormData();
  formData.set('grant_type', 'refresh_token');
  formData.set('refresh_token', refresh_token);

  const response = await fetch(accessTokenUrl, {
    method: 'POST',
    headers: {
      'Authorization': authorizationHeader,
      'Content-Type': 'application/x-www-form-urlencoded'
    },
    body: new URLSearchParams(formData)
  });
  if (response.ok) {
    const { access_token, expires_in, refresh_token } = await response.json();
    localStorage.setItem('access_token', access_token);
    localStorage.setItem('expires_after', Date.now() + (expires_in * 1000));
    localStorage.setItem('refresh_token', refresh_token);
    return true;
  } else {
    console.warn('Cannot get refresh token', await response.text());
    throw false;
  }
}

function redirectToAuthUrl() {
  window.location = authorizationCodeUrl;
}

function parseTimeToString(timestamp) {
  // timestamp is in ms.
  const timestampInS = timestamp / 1000;
  if (timestampInS > 3600) {
    const hours = parseInt(timestampInS / 3600);
    const timestampOfMinutesInS = timestampInS % 3600;
    const minutes = parseInt(timestampOfMinutesInS / 60);
    const seconds = parseInt(timestampOfMinutesInS % 60);
    // time with hours and minutes and seconds
    return `${hours}:${minutes}:${seconds}`;
  } else {
    // time with minutes and seconds
    const minutes = parseInt(timestampInS / 60);
    const seconds = parseInt(timestampInS % 60);
    // time with hours and minutes and seconds
    return `${minutes}:${seconds < 10 ? '0' + seconds : seconds}`;
  }
}