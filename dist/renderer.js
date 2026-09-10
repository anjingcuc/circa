const { invoke } = window.__TAURI__.core;
const { getCurrentWindow } = window.__TAURI__.window;
const { listen } = window.__TAURI__.event;
const win = getCurrentWindow();

const video = document.getElementById('cam');
const camSelect = document.getElementById('camSelect');
const closeBtn = document.getElementById('closeBtn');
const minimizeBtn = document.getElementById('minimizeBtn');
const clickThroughBtn = document.getElementById('clickThroughBtn');
const container = document.getElementById('container');
const controls = document.getElementById('controls');

let currentStream = null;

async function startCamera(deviceId) {
  if (currentStream) currentStream.getTracks().forEach((t) => t.stop());
  const constraints = {
    video: deviceId
      ? { deviceId: { exact: deviceId }, width: { ideal: 1280 }, height: { ideal: 720 } }
      : { width: { ideal: 1280 }, height: { ideal: 720 } },
    audio: false,
  };
  currentStream = await navigator.mediaDevices.getUserMedia(constraints);
  video.srcObject = currentStream;
}

async function listCameras(preferredId) {
  const devices = await navigator.mediaDevices.enumerateDevices();
  const cams = devices.filter((d) => d.kind === 'videoinput');
  camSelect.innerHTML = '';
  cams.forEach((c, i) => {
    const opt = document.createElement('option');
    opt.value = c.deviceId;
    opt.textContent = c.label || `Camera ${i + 1}`;
    camSelect.appendChild(opt);
  });
  if (preferredId && cams.some((c) => c.deviceId === preferredId)) {
    camSelect.value = preferredId;
  }
}

function showError(msg) {
  container.innerHTML = `<div class="error">Camera error<br><br>${msg}</div>`;
}

(async () => {
  try {
    // Prime permission so enumerateDevices returns labels.
    const tmp = await navigator.mediaDevices.getUserMedia({ video: true });
    tmp.getTracks().forEach((t) => t.stop());

    const saved = localStorage.getItem('cameraId');
    await listCameras(saved);
    if (camSelect.options.length > 0) {
      await startCamera(camSelect.value);
    } else {
      showError('No camera detected.');
    }
  } catch (e) {
    showError(e.message || String(e));
  }
})();

camSelect.addEventListener('change', async () => {
  try {
    await startCamera(camSelect.value);
    localStorage.setItem('cameraId', camSelect.value);
  } catch (e) {
    showError(e.message || String(e));
  }
});

// Refresh device list if cameras are plugged/unplugged.
navigator.mediaDevices.addEventListener('devicechange', () =>
  listCameras(camSelect.value)
);

closeBtn.addEventListener('click', () => win.close());
minimizeBtn.addEventListener('click', () => win.minimize());
clickThroughBtn.addEventListener('click', () => invoke('toggle_click_through'));

// The Rust cursor watcher needs to know where the controls are (in physical
// pixels) so it can un-ignore mouse events while the cursor is over them.
function reportControlsRect() {
  const r = controls.getBoundingClientRect();
  const dpr = window.devicePixelRatio || 1;
  invoke('set_controls_rect', {
    x: Math.round(r.left * dpr),
    y: Math.round(r.top * dpr),
    width: Math.round(r.width * dpr),
    height: Math.round(r.height * dpr),
  });
}
reportControlsRect();
win.onResized(reportControlsRect);
new ResizeObserver(reportControlsRect).observe(controls);

listen('click-through-changed', (e) => {
  clickThroughBtn.classList.toggle('active', e.payload);
});

// Scroll wheel resizes the window. Step is small so tuning feels precise.
window.addEventListener(
  'wheel',
  (e) => {
    e.preventDefault();
    const step = e.deltaY < 0 ? 16 : -16;
    invoke('resize_window', { delta: step });
  },
  { passive: false }
);
