const { spawn } = require('child_process');
const http = require('http');

async function testTimesNewRoman() {
  const chromePath = 'C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe';
  const chromeProc = spawn(chromePath, [
    '--headless=new',
    '--remote-debugging-port=9222',
    '--disable-gpu',
    '--no-sandbox',
    'about:blank'
  ]);

  await new Promise(r => setTimeout(r, 1500));

  let versionInfo;
  for (let i = 0; i < 10; i++) {
    try {
      versionInfo = await new Promise((resolve, reject) => {
        http.get('http://127.0.0.1:9222/json/list', res => {
          let data = '';
          res.on('data', chunk => data += chunk);
          res.on('end', () => resolve(JSON.parse(data)[0]));
        }).on('error', reject);
      });
      break;
    } catch (e) {
      await new Promise(r => setTimeout(r, 500));
    }
  }

  const ws = new WebSocket(versionInfo.webSocketDebuggerUrl);
  let id = 1;
  const callbacks = new Map();
  ws.onmessage = evt => {
    const msg = JSON.parse(evt.data);
    if (msg.id && callbacks.has(msg.id)) {
      callbacks.get(msg.id)(msg);
      callbacks.delete(msg.id);
    }
  };
  await new Promise(r => ws.onopen = r);
  function send(m, p = {}) {
    return new Promise(res => {
      const msgId = id++;
      callbacks.set(msgId, res);
      ws.send(JSON.stringify({ id: msgId, method: m, params: p }));
    });
  }

  await send('Page.enable');
  await send('Runtime.enable');
  await send('Page.navigate', { url: 'http://localhost:8080' });
  await new Promise(r => setTimeout(r, 4500));

  const result = await send('Runtime.evaluate', {
    expression: `(() => {
      const style = document.createElement('style');
      style.id = 'times-new-roman-test';
      style.innerHTML = \`
        @font-face {
          font-family: 'bariolmedium';
          src: local('Times New Roman'), local('Times'), serif;
        }
        @font-face {
          font-family: 'bariolnumbers';
          src: local('Times New Roman'), local('Times'), serif;
        }
        body, #plugin-menu, .mobile-calendar, .timecode, #search, .rhpane {
          font-family: "Times New Roman", Times, serif !important;
        }
      \`;
      document.head.appendChild(style);

      const bodyFont = window.getComputedStyle(document.body).fontFamily;
      const menuFont = window.getComputedStyle(document.querySelector('#plugin-menu') || document.body).fontFamily;
      return {
        bodyFont,
        menuFont,
        canvasRunning: document.querySelectorAll('canvas').length > 0
      };
    })()`,
    returnByValue: true
  });

  console.log('TIMES NEW ROMAN TEST:', result.result.result.value);
  ws.close();
  chromeProc.kill();
}

testTimesNewRoman().catch(console.error);
