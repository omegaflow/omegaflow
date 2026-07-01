export const ANTIGEN_PATHS = [
    'location', 'location.href', 'location.assign', 'location.replace',
    'history', 'history.pushState', 'history.replaceState',
    'href', 'opener',
    'document.write', 'document.open', 'document.close', 'document.writeln',
    'document.location', 'document.URL',
    'eval', 'Function', 'setTimeout', 'setInterval', 'setImmediate',
    'requestAnimationFrame', 'queueMicrotask',
    'fetch', 'XMLHttpRequest', 'WebSocket', 'EventSource',
    'importScripts', 'createObjectURL', 'revokeObjectURL',
    'alert', 'confirm', 'prompt', 'print', 'open', 'close', 'stop',
    'focus', 'blur', 'postMessage',
    'scrollTo', 'scrollBy', 'scroll', 'scrollIntoView', 'scrollIntoViewIfNeeded',
    'moveTo', 'moveBy', 'resizeTo', 'resizeBy',
    'cookie', 'localStorage', 'sessionStorage', 'indexedDB',
    'caches', 'serviceWorker', 'SharedWorker', 'Worker',
    'submit', 'reset', 'click', 'select',
    'reportError', 'reportValidation',
    'showDirectoryPicker', 'showOpenFilePicker', 'showSaveFilePicker',
    'getScreenDetails', 'getDisplayMedia', 'getUserMedia',
    'requestPermission', 'requestMIDIAccess', 'requestDeviceToken',
    'dispatchEvent',
];

export const ANTIGEN_NATIVE_PATTERNS = [];

export const REMOTE_PREFIX = 'omega_flow.';

export function isAntigen(path, fn) {
    const segments = path.split('.');
    const lastSeg = segments[segments.length - 1];
    for (const seg of ANTIGEN_PATHS) {
        if (path === seg || path.startsWith(seg + '.') || path.startsWith(seg + '[')) {
            return true;
        }
        if (lastSeg === seg) return true;
    }
    return false;
}

export function distress(category, detail) {
    const msg = category + (detail ? ': ' + detail : '');
    fetch('/crash', { method: 'POST', body: msg }).catch(() => {});
}

