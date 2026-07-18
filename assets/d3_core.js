// Load d3.js from CDN
export async function loadD3() {
  if (typeof d3 !== 'undefined') {
    return true;
  }

  return new Promise((resolve, reject) => {
    const script = document.createElement('script');
    script.src = 'https://cdn.jsdelivr.net/npm/d3@7.9.0/dist/d3.min.js';
    script.onload = () => resolve(true);
    script.onerror = () => reject(new Error('Failed to load d3.js'));
    document.head.appendChild(script);
  });
}

export function isD3Loaded() {
  return typeof d3 !== 'undefined';
}
